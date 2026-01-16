//! 图片消息

use super::{Message, UserInfo, message_type};
use serde::{Deserialize, Serialize};

/// 图片消息 (RC:ImgMsg)
///
/// 发送图片内容，包含缩略图和原图 URL
///
/// # 消息属性
/// - **客户端计数与存储策略**: ISCOUNTED - 在客户端存储，且计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认已支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImgMessage {
    /// 图片的 Base64 缩略图数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 原图 URL
    #[serde(rename = "imageUri")]
    pub image_uri: String,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl ImgMessage {
    /// 创建图片消息
    ///
    /// # Arguments
    /// * `image_uri` - 原图 URL
    pub fn new(image_uri: impl Into<String>) -> Self {
        Self {
            content: None,
            image_uri: image_uri.into(),
            extra: None,
            user: None,
        }
    }

    /// 设置缩略图内容（Base64 编码）
    pub fn with_thumbnail(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// 设置扩展信息
    pub fn with_extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    /// 设置用户信息
    pub fn with_user(mut self, user: UserInfo) -> Self {
        self.user = Some(user);
        self
    }
}

impl Message for ImgMessage {
    fn message_type(&self) -> &'static str {
        message_type::IMAGE
    }
}

/// 图文消息 (RC:ImgTextMsg)
///
/// 包含标题、描述、图片和链接的富文本消息
///
/// # 消息属性
/// - **客户端计数与存储策略**: ISCOUNTED - 在客户端存储，且计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认已支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImgTextMessage {
    /// 消息标题
    pub title: String,
    /// 消息描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 图片 URL
    #[serde(rename = "imageUri", skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    /// 图文消息跳转的 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl ImgTextMessage {
    /// 创建图文消息
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: None,
            image_uri: None,
            url: None,
            extra: None,
            user: None,
        }
    }

    /// 设置描述内容
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// 设置图片 URL
    pub fn with_image(mut self, image_uri: impl Into<String>) -> Self {
        self.image_uri = Some(image_uri.into());
        self
    }

    /// 设置跳转链接
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

impl Message for ImgTextMessage {
    fn message_type(&self) -> &'static str {
        message_type::IMAGE_TEXT
    }
}

/// GIF 消息 (RC:GIFMsg)
///
/// GIF 动图消息
///
/// # 消息属性
/// - **客户端计数与存储策略**: ISCOUNTED - 在客户端存储，且计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认已支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GifMessage {
    /// GIF 远程地址
    #[serde(rename = "remoteUrl")]
    pub remote_url: String,
    /// GIF 宽度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// GIF 高度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// GIF 大小（字节）
    #[serde(rename = "gifDataSize", skip_serializing_if = "Option::is_none")]
    pub gif_data_size: Option<u64>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl GifMessage {
    pub fn new(remote_url: impl Into<String>) -> Self {
        Self {
            remote_url: remote_url.into(),
            width: None,
            height: None,
            gif_data_size: None,
            extra: None,
            user: None,
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

impl Message for GifMessage {
    fn message_type(&self) -> &'static str {
        message_type::GIF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_img_message() {
        let msg = ImgMessage::new("http://example.com/image.jpg").with_thumbnail("base64data");

        assert_eq!(msg.message_type(), "RC:ImgMsg");
        assert_eq!(msg.image_uri, "http://example.com/image.jpg");
    }

    #[test]
    fn test_img_text_message() {
        let msg = ImgTextMessage::new("标题")
            .with_content("描述")
            .with_image("http://example.com/cover.jpg")
            .with_url("http://example.com/article");

        assert_eq!(msg.message_type(), "RC:ImgTextMsg");
        assert_eq!(msg.title, "标题");
    }

    #[test]
    fn test_gif_message() {
        let msg = GifMessage::new("http://example.com/funny.gif").with_size(200, 150);

        assert_eq!(msg.message_type(), "RC:GIFMsg");
        assert_eq!(msg.width, Some(200));
    }
}
