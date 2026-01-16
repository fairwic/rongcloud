//! 会话类型定义
//!
//! 定义融云支持的各种会话类型

use serde_repr::{Deserialize_repr, Serialize_repr};

/// 会话类型枚举
///
/// 用于指定消息发送的目标会话类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ConversationType {
    /// 单聊会话
    Private = 1,
    /// 讨论组（已废弃，保留兼容）
    Discussion = 2,
    /// 群组会话
    Group = 3,
    /// 聊天室会话
    ChatRoom = 4,
    /// 客服会话
    CustomerService = 5,
    /// 系统消息
    System = 6,
    /// 应用公众服务
    AppPublicService = 7,
    /// 公众服务
    PublicService = 8,
    /// 超级群
    UltraGroup = 10,
}

impl ConversationType {
    /// 获取会话类型的中文描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::Private => "单聊",
            Self::Discussion => "讨论组",
            Self::Group => "群组",
            Self::ChatRoom => "聊天室",
            Self::CustomerService => "客服",
            Self::System => "系统",
            Self::AppPublicService => "应用公众服务",
            Self::PublicService => "公众服务",
            Self::UltraGroup => "超级群",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_type_value() {
        assert_eq!(ConversationType::Private as u8, 1);
        assert_eq!(ConversationType::Group as u8, 3);
        assert_eq!(ConversationType::ChatRoom as u8, 4);
        assert_eq!(ConversationType::System as u8, 6);
        assert_eq!(ConversationType::UltraGroup as u8, 10);
    }

    #[test]
    fn test_conversation_type_description() {
        assert_eq!(ConversationType::Private.description(), "单聊");
        assert_eq!(ConversationType::Group.description(), "群组");
    }

    #[test]
    fn test_serde_roundtrip() {
        let conv_type = ConversationType::Group;
        let json = serde_json::to_string(&conv_type).unwrap();
        assert_eq!(json, "3");

        let parsed: ConversationType = serde_json::from_str("3").unwrap();
        assert_eq!(parsed, ConversationType::Group);
    }
}
