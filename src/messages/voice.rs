//! 语音消息

use super::{Message, UserInfo, message_type};
use serde::{Deserialize, Serialize};

/// 语音消息 (RC:VcMsg)
///
/// AMR 格式语音消息
///
/// # 消息属性
/// - **客户端计数与存储策略**: ISCOUNTED - 在客户端存储，且计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认已支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceMessage {
    /// 语音内容的 Base64 编码
    pub content: String,
    /// 语音时长（秒）
    pub duration: u32,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl VoiceMessage {
    /// 创建语音消息
    ///
    /// # Arguments
    /// * `content` - Base64 编码的语音数据
    /// * `duration` - 语音时长（秒）
    pub fn new(content: impl Into<String>, duration: u32) -> Self {
        Self {
            content: content.into(),
            duration,
            extra: None,
            user: None,
        }
    }

    pub fn with_extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    pub fn with_user(mut self, user: UserInfo) -> Self {
        self.user = Some(user);
        self
    }
}

impl Message for VoiceMessage {
    fn message_type(&self) -> &'static str {
        message_type::VOICE
    }
}

/// 高清语音消息 (RC:HQVCMsg)
///
/// AAC 格式高清语音消息
///
/// # 消息属性
/// - **客户端计数与存储策略**: ISCOUNTED - 在客户端存储，且计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认已支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HQVoiceMessage {
    /// 远程音频地址
    #[serde(rename = "remoteUrl")]
    pub remote_url: String,
    /// 语音时长（秒）
    pub duration: u32,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl HQVoiceMessage {
    /// 创建高清语音消息
    ///
    /// # Arguments
    /// * `remote_url` - 远程音频 URL
    /// * `duration` - 语音时长（秒）
    pub fn new(remote_url: impl Into<String>, duration: u32) -> Self {
        Self {
            remote_url: remote_url.into(),
            duration,
            extra: None,
            user: None,
        }
    }

    pub fn with_extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    pub fn with_user(mut self, user: UserInfo) -> Self {
        self.user = Some(user);
        self
    }
}

impl Message for HQVoiceMessage {
    fn message_type(&self) -> &'static str {
        message_type::HQ_VOICE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_message() {
        let msg = VoiceMessage::new("base64encodeddata", 10);
        assert_eq!(msg.message_type(), "RC:VcMsg");
        assert_eq!(msg.duration, 10);
    }

    #[test]
    fn test_hq_voice_message() {
        let msg = HQVoiceMessage::new("http://example.com/voice.aac", 30);
        assert_eq!(msg.message_type(), "RC:HQVCMsg");
        assert_eq!(msg.duration, 30);
    }
}
