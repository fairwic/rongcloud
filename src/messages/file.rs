//! 文件消息

use super::{Message, UserInfo, message_type};
use serde::{Deserialize, Serialize};

/// 文件消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMessage {
    /// 文件名
    pub name: String,
    /// 文件大小（字节）
    pub size: u64,
    /// 文件类型
    #[serde(rename = "type")]
    pub file_type: String,
    /// 文件远程地址
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl FileMessage {
    pub fn new(
        name: impl Into<String>,
        file_type: impl Into<String>,
        file_url: impl Into<String>,
        size: u64,
    ) -> Self {
        Self {
            name: name.into(),
            size,
            file_type: file_type.into(),
            file_url: file_url.into(),
            extra: None,
            user: None,
        }
    }

    pub fn with_extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }
}

impl Message for FileMessage {
    fn message_type(&self) -> &'static str {
        message_type::FILE
    }
}

/// 位置消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LBSMessage {
    /// 纬度
    pub latitude: f64,
    /// 经度
    pub longitude: f64,
    /// 位置名称
    #[serde(rename = "poi", skip_serializing_if = "Option::is_none")]
    pub poi: Option<String>,
    /// 位置缩略图（Base64）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl LBSMessage {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            poi: None,
            content: None,
            extra: None,
            user: None,
        }
    }

    pub fn with_poi(mut self, poi: impl Into<String>) -> Self {
        self.poi = Some(poi.into());
        self
    }

    pub fn with_thumbnail(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }
}

impl Message for LBSMessage {
    fn message_type(&self) -> &'static str {
        message_type::LBS
    }
}

/// 小视频消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SightMessage {
    /// 视频远程地址
    #[serde(rename = "sightUrl")]
    pub sight_url: String,
    /// 视频时长（秒）
    pub duration: u32,
    /// 视频大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    /// 缩略图 Base64
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl SightMessage {
    pub fn new(sight_url: impl Into<String>, duration: u32) -> Self {
        Self {
            sight_url: sight_url.into(),
            duration,
            size: None,
            content: None,
            extra: None,
            user: None,
        }
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_thumbnail(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }
}

impl Message for SightMessage {
    fn message_type(&self) -> &'static str {
        message_type::SIGHT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_message() {
        let msg = FileMessage::new("doc.pdf", "pdf", "http://example.com/doc.pdf", 1024);
        assert_eq!(msg.message_type(), "RC:FileMsg");
        assert_eq!(msg.name, "doc.pdf");
    }

    #[test]
    fn test_lbs_message() {
        let msg = LBSMessage::new(39.9042, 116.4074).with_poi("天安门");
        assert_eq!(msg.message_type(), "RC:LBSMsg");
        assert_eq!(msg.poi, Some("天安门".to_string()));
    }

    #[test]
    fn test_sight_message() {
        let msg = SightMessage::new("http://example.com/video.mp4", 60);
        assert_eq!(msg.message_type(), "RC:SightMsg");
        assert_eq!(msg.duration, 60);
    }
}
