//! 文本消息

use super::{MentionedInfo, Message, UserInfo, message_type};
use serde::{Deserialize, Serialize};

/// 文本消息 (RC:TxtMsg)
///
/// 最基本的消息类型，包含文本内容
///
/// # 消息属性
/// - **客户端计数与存储策略**: ISCOUNTED - 在客户端存储，且计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认已支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxtMessage {
    /// 消息内容
    pub content: String,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
    /// @提及信息
    #[serde(rename = "mentionedInfo", skip_serializing_if = "Option::is_none")]
    pub mentioned_info: Option<MentionedInfo>,
}

impl TxtMessage {
    /// 创建文本消息
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            extra: None,
            user: None,
            mentioned_info: None,
        }
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

    /// 设置 @提及信息
    pub fn with_mention(mut self, mentioned: MentionedInfo) -> Self {
        self.mentioned_info = Some(mentioned);
        self
    }
}

impl Message for TxtMessage {
    fn message_type(&self) -> &'static str {
        message_type::TEXT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_txt_message_creation() {
        let msg = TxtMessage::new("Hello World");
        assert_eq!(msg.content, "Hello World");
        assert_eq!(msg.message_type(), "RC:TxtMsg");
    }

    #[test]
    fn test_txt_message_builder() {
        let msg = TxtMessage::new("Hello")
            .with_extra(r#"{"key":"value"}"#)
            .with_user(UserInfo::new("user1", "张三"));

        assert_eq!(msg.content, "Hello");
        assert!(msg.extra.is_some());
        assert!(msg.user.is_some());
    }

    #[test]
    fn test_txt_message_serialization() {
        let msg = TxtMessage::new("Hello");
        let json = msg.to_json().unwrap();

        assert!(json.contains("Hello"));
        assert!(json.contains("content"));
    }
}
