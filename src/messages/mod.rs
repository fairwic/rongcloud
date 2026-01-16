//! 融云消息类型系统
//!
//! 定义所有支持的消息类型及其序列化

mod file;
mod image;
mod notification;
mod text;
mod voice;

pub use file::*;
pub use image::*;
pub use notification::*;
pub use text::*;
pub use voice::*;

use serde::{Deserialize, Serialize};

// ============================================================================
// 消息类型标识符常量
// ============================================================================

/// 消息类型标识符常量模块
///
/// 每个消息类型都有一个唯一的字符串标识符，格式为 `RC:*Msg`
pub mod message_type {
    /// 文本消息
    pub const TEXT: &str = "RC:TxtMsg";
    /// 图片消息
    pub const IMAGE: &str = "RC:ImgMsg";
    /// 语音消息
    pub const VOICE: &str = "RC:VcMsg";
    /// 高清语音消息
    pub const HQ_VOICE: &str = "RC:HQVCMsg";
    /// 图文消息
    pub const IMAGE_TEXT: &str = "RC:ImgTextMsg";
    /// 文件消息
    pub const FILE: &str = "RC:FileMsg";
    /// 位置消息
    pub const LBS: &str = "RC:LBSMsg";
    /// GIF 消息
    pub const GIF: &str = "RC:GIFMsg";
    /// 小视频消息
    pub const SIGHT: &str = "RC:SightMsg";
    /// 命令消息
    pub const CMD: &str = "RC:CmdMsg";
    /// 命令通知消息
    pub const CMD_NTF: &str = "RC:CmdNtf";
    /// 联系人通知消息
    pub const CONTACT_NTF: &str = "RC:ContactNtf";
    /// 资料通知消息
    pub const PROFILE_NTF: &str = "RC:ProfileNtf";
    /// 信息通知消息
    pub const INFO_NTF: &str = "RC:InfoNtf";
    /// 群组通知消息
    pub const GROUP_NTF: &str = "RC:GrpNtf";
    /// 已读回执
    pub const READ_RECEIPT: &str = "RC:ReadNtf";
    /// 正在输入状态
    pub const TYPING_STATUS: &str = "RC:TypSts";
    /// 撤回命令消息
    pub const RECALL_CMD: &str = "RC:RcCmd";
}

// ============================================================================
// 消息 Trait
// ============================================================================

/// 消息 trait
///
/// 所有消息类型都需要实现此 trait
pub trait Message: Serialize {
    /// 获取消息类型标识符
    fn message_type(&self) -> &'static str;

    /// 将消息序列化为 JSON 字符串
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

// ============================================================================
// 用户信息（可嵌入消息）
// ============================================================================

/// 用户信息结构
///
/// 可以嵌入到消息中，用于在接收端显示发送者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户 ID
    #[serde(rename = "id")]
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户头像 URL
    #[serde(rename = "portraitUri", skip_serializing_if = "Option::is_none")]
    pub portrait_uri: Option<String>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

impl UserInfo {
    pub fn new(user_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            name: name.into(),
            portrait_uri: None,
            extra: None,
        }
    }

    pub fn with_portrait(mut self, uri: impl Into<String>) -> Self {
        self.portrait_uri = Some(uri.into());
        self
    }
}

// ============================================================================
// @提及信息
// ============================================================================

/// @提及类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MentionedType {
    /// @指定用户
    #[serde(rename = "1")]
    Users = 1,
    /// @所有人
    #[serde(rename = "2")]
    All = 2,
}

/// @提及信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentionedInfo {
    /// @提及类型
    #[serde(rename = "type")]
    pub mentioned_type: MentionedType,
    /// @的用户 ID 列表（当 type 为 Users 时使用）
    #[serde(rename = "userIdList", skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
    /// @内容（如 "@张三"）
    #[serde(rename = "mentionedContent", skip_serializing_if = "Option::is_none")]
    pub mentioned_content: Option<String>,
}

impl MentionedInfo {
    /// 创建 @指定用户 的提及信息
    pub fn users(user_ids: Vec<String>) -> Self {
        Self {
            mentioned_type: MentionedType::Users,
            user_id_list: Some(user_ids),
            mentioned_content: None,
        }
    }

    /// 创建 @所有人 的提及信息
    pub fn all() -> Self {
        Self {
            mentioned_type: MentionedType::All,
            user_id_list: None,
            mentioned_content: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_constants() {
        assert_eq!(message_type::TEXT, "RC:TxtMsg");
        assert_eq!(message_type::IMAGE, "RC:ImgMsg");
        assert_eq!(message_type::VOICE, "RC:VcMsg");
    }

    #[test]
    fn test_user_info_builder() {
        let user = UserInfo::new("user1", "张三").with_portrait("http://example.com/avatar.png");

        assert_eq!(user.user_id, "user1");
        assert_eq!(user.name, "张三");
        assert_eq!(
            user.portrait_uri,
            Some("http://example.com/avatar.png".to_string())
        );
    }

    #[test]
    fn test_mentioned_info() {
        let mention_users = MentionedInfo::users(vec!["user1".to_string(), "user2".to_string()]);
        assert_eq!(mention_users.mentioned_type, MentionedType::Users);
        assert!(mention_users.user_id_list.is_some());

        let mention_all = MentionedInfo::all();
        assert_eq!(mention_all.mentioned_type, MentionedType::All);
    }
}
