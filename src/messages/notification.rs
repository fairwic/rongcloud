//! 通知类和命令类消息

use super::{Message, UserInfo, message_type};
use serde::{Deserialize, Serialize};

/// 命令消息 (RC:CmdMsg)
///
/// 用于发送不需要在会话界面展示的指令
///
/// # 消息属性
/// - **客户端计数与存储策略**: NONE - 在客户端不存储，不计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认未支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdMessage {
    /// 命令名称
    pub name: String,
    /// 命令参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl CmdMessage {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: None,
            user: None,
        }
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }
}

impl Message for CmdMessage {
    fn message_type(&self) -> &'static str {
        message_type::CMD
    }
}

/// 命令通知消息 (RC:CmdNtf)
///
/// 命令提醒消息
///
/// # 消息属性
/// - **客户端计数与存储策略**: ISPERSISTED - 在客户端存储，不计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认未支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdNtfMessage {
    /// 通知名称
    pub name: String,
    /// 通知参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl CmdNtfMessage {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: None,
            user: None,
        }
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }
}

impl Message for CmdNtfMessage {
    fn message_type(&self) -> &'static str {
        message_type::CMD_NTF
    }
}

/// 联系人通知消息 (RC:ContactNtf)
///
/// 好友相关通知
///
/// # 消息属性
/// - **客户端计数与存储策略**: ISPERSISTED - 在客户端存储，不计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认未支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactNtfMessage {
    /// 操作类型
    pub operation: String,
    /// 操作发起者 ID
    #[serde(rename = "sourceUserId")]
    pub source_user_id: String,
    /// 操作目标 ID
    #[serde(rename = "targetUserId")]
    pub target_user_id: String,
    /// 附加消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl ContactNtfMessage {
    pub fn new(
        operation: impl Into<String>,
        source_user_id: impl Into<String>,
        target_user_id: impl Into<String>,
    ) -> Self {
        Self {
            operation: operation.into(),
            source_user_id: source_user_id.into(),
            target_user_id: target_user_id.into(),
            message: None,
            extra: None,
            user: None,
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl Message for ContactNtfMessage {
    fn message_type(&self) -> &'static str {
        message_type::CONTACT_NTF
    }
}

/// 资料通知消息 (RC:ProfileNtf)
///
/// 用户资料变更通知
///
/// # 消息属性
/// - **客户端计数与存储策略**: ISPERSISTED - 在客户端存储，不计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认未支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileNtfMessage {
    /// 操作类型
    pub operation: String,
    /// 操作参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl ProfileNtfMessage {
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            data: None,
            extra: None,
            user: None,
        }
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }
}

impl Message for ProfileNtfMessage {
    fn message_type(&self) -> &'static str {
        message_type::PROFILE_NTF
    }
}

/// 信息通知消息 (RC:InfoNtf)
///
/// 提示条通知消息
///
/// # 消息属性
/// - **客户端计数与存储策略**: ISPERSISTED - 在客户端存储，不计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认未支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoNtfMessage {
    /// 通知内容
    pub message: String,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl InfoNtfMessage {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            extra: None,
            user: None,
        }
    }
}

impl Message for InfoNtfMessage {
    fn message_type(&self) -> &'static str {
        message_type::INFO_NTF
    }
}

/// 群组通知消息 (RC:GrpNtf)
///
/// 群组操作通知
///
/// # 消息属性
/// - **客户端计数与存储策略**: ISPERSISTED - 在客户端存储，不计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认未支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupNtfMessage {
    /// 操作名称
    #[serde(rename = "operatorUserId")]
    pub operator_user_id: String,
    /// 操作类型
    pub operation: String,
    /// 通知内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 操作参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
}

impl GroupNtfMessage {
    pub fn new(operator_user_id: impl Into<String>, operation: impl Into<String>) -> Self {
        Self {
            operator_user_id: operator_user_id.into(),
            operation: operation.into(),
            message: None,
            data: None,
            extra: None,
            user: None,
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }
}

impl Message for GroupNtfMessage {
    fn message_type(&self) -> &'static str {
        message_type::GROUP_NTF
    }
}

/// 已读回执消息 (RC:ReadNtf)
///
/// 单聊已读回执
///
/// # 消息属性
/// - **客户端计数与存储策略**: NONE - 在客户端不存储，不计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认未支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadReceiptMessage {
    /// 最后一条已读消息的发送时间
    #[serde(rename = "lastMessageSendTime")]
    pub last_message_send_time: i64,
    /// 消息类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub read_type: Option<i32>,
    /// 消息 UID 列表
    #[serde(rename = "messageUIdList", skip_serializing_if = "Option::is_none")]
    pub message_uid_list: Option<Vec<String>>,
}

impl ReadReceiptMessage {
    pub fn new(last_message_send_time: i64) -> Self {
        Self {
            last_message_send_time,
            read_type: None,
            message_uid_list: None,
        }
    }
}

impl Message for ReadReceiptMessage {
    fn message_type(&self) -> &'static str {
        message_type::READ_RECEIPT
    }
}

/// 正在输入状态消息 (RC:TypSts)
///
/// 表示用户正在输入的状态消息
///
/// # 消息属性
/// - **客户端计数与存储策略**: STATUS - 在客户端不存储，不计入未读数  
/// - **离线消息缓存**: 不支持  
/// - **远程推送通知**: 不支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingStatusMessage {
    /// 正在输入的消息类型
    #[serde(rename = "typingContentType")]
    pub typing_content_type: String,
}

impl TypingStatusMessage {
    pub fn new(typing_content_type: impl Into<String>) -> Self {
        Self {
            typing_content_type: typing_content_type.into(),
        }
    }

    /// 创建"正在输入文字"状态
    pub fn text() -> Self {
        Self::new(message_type::TEXT)
    }

    /// 创建"正在输入语音"状态
    pub fn voice() -> Self {
        Self::new(message_type::VOICE)
    }
}

impl Message for TypingStatusMessage {
    fn message_type(&self) -> &'static str {
        message_type::TYPING_STATUS
    }
}

/// 撤回命令消息 (RC:RcCmd)
///
/// 消息撤回命令
///
/// # 消息属性
/// - **客户端计数与存储策略**: NONE - 在客户端不存储，不计入未读数  
/// - **离线消息缓存**: 支持  
/// - **远程推送通知**: 默认已支持推送  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallCmdMessage {
    /// 被撤回的消息 UID
    #[serde(rename = "messageUId")]
    pub message_uid: String,
    /// 会话类型
    #[serde(rename = "conversationType")]
    pub conversation_type: i32,
    /// 发送时间
    #[serde(rename = "sentTime")]
    pub sent_time: i64,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

impl RecallCmdMessage {
    pub fn new(message_uid: impl Into<String>, conversation_type: i32, sent_time: i64) -> Self {
        Self {
            message_uid: message_uid.into(),
            conversation_type,
            sent_time,
            user: None,
            extra: None,
        }
    }
}

impl Message for RecallCmdMessage {
    fn message_type(&self) -> &'static str {
        message_type::RECALL_CMD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd_message() {
        let msg = CmdMessage::new("custom_cmd").with_data(r#"{"action":"refresh"}"#);
        assert_eq!(msg.message_type(), "RC:CmdMsg");
    }

    #[test]
    fn test_group_ntf_message() {
        let msg = GroupNtfMessage::new("user1", "Add").with_message("user1 加入了群组");
        assert_eq!(msg.message_type(), "RC:GrpNtf");
    }

    #[test]
    fn test_typing_status() {
        let msg = TypingStatusMessage::text();
        assert_eq!(msg.typing_content_type, "RC:TxtMsg");
    }
}
