//! 消息发送 API
//!
//! 包含单聊、群聊、系统消息等发送接口

use crate::core::RongCloud;
use crate::core::RongCloudError;
use crate::types::RcResponse;
use serde::Serialize;
use std::collections::HashMap;

// ============================================================================
// 消息发送参数结构体
// ============================================================================

/// 单聊消息参数
///
/// 包含发送单聊消息所需的所有字段
#[derive(Debug, Clone, Serialize, Default)]
pub struct PrivateMessage {
    /// 发送人 ID (必填)
    #[serde(rename = "fromUserId")]
    pub from_user_id: String,
    /// 接收人 ID 列表 (必填)
    #[serde(rename = "toUserId")]
    pub to_user_ids: Vec<String>,
    /// 消息类型，如 RC:TxtMsg (必填)
    #[serde(rename = "objectName")]
    pub object_name: String,
    /// 消息内容 JSON 字符串 (必填)
    pub content: String,
    /// 推送内容 (可选)
    #[serde(rename = "pushContent", skip_serializing_if = "Option::is_none")]
    pub push_content: Option<String>,
    /// 推送附加数据 (可选)
    #[serde(rename = "pushData", skip_serializing_if = "Option::is_none")]
    pub push_data: Option<String>,
    /// 推送扩展配置 JSON (可选)
    #[serde(rename = "pushExt", skip_serializing_if = "Option::is_none")]
    pub push_ext: Option<String>,
    /// iOS 角标数量 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    /// 是否在服务端存储消息。0: 不存储, 1: 存储。默认 1 (可选)
    #[serde(rename = "isPersisted", skip_serializing_if = "Option::is_none")]
    pub is_persisted: Option<i32>,
    /// 是否算作未读消息数。0: 不计数, 1: 计数。默认 1 (可选)
    #[serde(rename = "isCounted", skip_serializing_if = "Option::is_none")]
    pub is_counted: Option<i32>,
    /// 是否过滤黑名单。0: 不过滤, 1: 过滤。默认 0 (可选)
    #[serde(rename = "verifyBlacklist", skip_serializing_if = "Option::is_none")]
    pub verify_blacklist: Option<i32>,
    /// 发送者是否也收到消息。0: 不接收, 1: 接收。默认 0 (可选)
    #[serde(rename = "isIncludeSender", skip_serializing_if = "Option::is_none")]
    pub is_include_sender: Option<i32>,
    /// iOS 静默推送开关。0: 关闭, 1: 开启 (可选)
    #[serde(rename = "contentAvailable", skip_serializing_if = "Option::is_none")]
    pub content_available: Option<i32>,
    /// 是否禁用推送。true: 禁用 (可选)
    #[serde(rename = "disablePush", skip_serializing_if = "Option::is_none")]
    pub disable_push: Option<bool>,
    /// 是否支持消息扩展。true: 支持 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansion: Option<bool>,
    /// 消息扩展内容 (expansion 为 true 时有效)
    #[serde(rename = "extraContent", skip_serializing_if = "Option::is_none")]
    pub extra_content: Option<HashMap<String, String>>,
    /// 是否需要已读回执。0: 不需要, 1: 需要。默认 0 (可选)
    #[serde(rename = "needReadReceipt", skip_serializing_if = "Option::is_none")]
    pub need_read_receipt: Option<i32>,
    /// 幂等标识，相同值在 1 分钟内重复发送会报错 (可选)
    #[serde(rename = "msgRandom", skip_serializing_if = "Option::is_none")]
    pub msg_random: Option<i64>,
    /// 是否禁止更新会话最后一条消息 (可选)
    #[serde(
        rename = "disableUpdateLastMsg",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_update_last_msg: Option<bool>,
}

impl PrivateMessage {
    pub fn new(
        from_user_id: impl Into<String>,
        object_name: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            from_user_id: from_user_id.into(),
            object_name: object_name.into(),
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn to_user(mut self, user_id: impl Into<String>) -> Self {
        self.to_user_ids.push(user_id.into());
        self
    }

    pub fn to_users(mut self, user_ids: Vec<String>) -> Self {
        self.to_user_ids = user_ids;
        self
    }

    pub fn push_content(mut self, push_content: impl Into<String>) -> Self {
        self.push_content = Some(push_content.into());
        self
    }

    pub fn push_data(mut self, push_data: impl Into<String>) -> Self {
        self.push_data = Some(push_data.into());
        self
    }

    pub fn push_ext(mut self, push_ext: impl Into<String>) -> Self {
        self.push_ext = Some(push_ext.into());
        self
    }

    pub fn is_persisted(mut self, value: i32) -> Self {
        self.is_persisted = Some(value);
        self
    }

    pub fn is_counted(mut self, value: i32) -> Self {
        self.is_counted = Some(value);
        self
    }

    pub fn verify_blacklist(mut self, value: i32) -> Self {
        self.verify_blacklist = Some(value);
        self
    }

    pub fn is_include_sender(mut self, value: i32) -> Self {
        self.is_include_sender = Some(value);
        self
    }

    pub fn disable_push(mut self, value: bool) -> Self {
        self.disable_push = Some(value);
        self
    }

    pub fn expansion(mut self, value: bool) -> Self {
        self.expansion = Some(value);
        self
    }

    pub fn disable_update_last_msg(mut self, value: bool) -> Self {
        self.disable_update_last_msg = Some(value);
        self
    }
}

/// 群聊消息参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct GroupMessage {
    /// 发送人 ID (必填)
    #[serde(rename = "fromUserId")]
    pub from_user_id: String,
    /// 目标群组 ID 列表，最多 3 个 (必填)
    #[serde(rename = "toGroupId")]
    pub to_group_ids: Vec<String>,
    /// 消息类型 (必填)
    #[serde(rename = "objectName")]
    pub object_name: String,
    /// 消息内容 JSON (必填)
    pub content: String,
    /// 推送内容 (可选)
    #[serde(rename = "pushContent", skip_serializing_if = "Option::is_none")]
    pub push_content: Option<String>,
    /// 推送附加数据 (可选)
    #[serde(rename = "pushData", skip_serializing_if = "Option::is_none")]
    pub push_data: Option<String>,
    /// 推送扩展配置 (可选)
    #[serde(rename = "pushExt", skip_serializing_if = "Option::is_none")]
    pub push_ext: Option<String>,
    /// 是否在服务端存储消息 (可选)
    #[serde(rename = "isPersisted", skip_serializing_if = "Option::is_none")]
    pub is_persisted: Option<i32>,
    /// 发送者是否也收到消息 (可选)
    #[serde(rename = "isIncludeSender", skip_serializing_if = "Option::is_none")]
    pub is_include_sender: Option<i32>,
    /// iOS 静默推送开关 (可选)
    #[serde(rename = "contentAvailable", skip_serializing_if = "Option::is_none")]
    pub content_available: Option<i32>,
    /// 定向用户 ID 列表，仅这些用户收到消息 (可选)
    #[serde(rename = "toUserId", skip_serializing_if = "Option::is_none")]
    pub to_user_ids: Option<Vec<String>>,
    /// 是否禁用推送 (可选)
    #[serde(rename = "disablePush", skip_serializing_if = "Option::is_none")]
    pub disable_push: Option<bool>,
    /// 是否支持消息扩展 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansion: Option<bool>,
    /// 消息扩展内容 (可选)
    #[serde(rename = "extraContent", skip_serializing_if = "Option::is_none")]
    pub extra_content: Option<HashMap<String, String>>,
    /// 是否为 @消息。0: 非@消息, 1: @消息 (可选)
    #[serde(rename = "isMentioned", skip_serializing_if = "Option::is_none")]
    pub is_mentioned: Option<i32>,
    /// 是否需要已读回执 (可选)
    #[serde(rename = "needReadReceipt", skip_serializing_if = "Option::is_none")]
    pub need_read_receipt: Option<i32>,
    /// 幂等标识 (可选)
    #[serde(rename = "msgRandom", skip_serializing_if = "Option::is_none")]
    pub msg_random: Option<i64>,
    /// 是否禁止更新会话最后一条消息 (可选)
    #[serde(
        rename = "disableUpdateLastMsg",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_update_last_msg: Option<bool>,
}

impl GroupMessage {
    pub fn new(
        from_user_id: impl Into<String>,
        object_name: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            from_user_id: from_user_id.into(),
            object_name: object_name.into(),
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn to_group(mut self, group_id: impl Into<String>) -> Self {
        self.to_group_ids.push(group_id.into());
        self
    }

    pub fn to_groups(mut self, group_ids: Vec<String>) -> Self {
        self.to_group_ids = group_ids;
        self
    }

    pub fn push_content(mut self, push_content: impl Into<String>) -> Self {
        self.push_content = Some(push_content.into());
        self
    }

    pub fn is_include_sender(mut self, value: i32) -> Self {
        self.is_include_sender = Some(value);
        self
    }

    pub fn is_mentioned(mut self, value: i32) -> Self {
        self.is_mentioned = Some(value);
        self
    }

    pub fn disable_push(mut self, value: bool) -> Self {
        self.disable_push = Some(value);
        self
    }
}

/// 系统消息参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct SystemMessage {
    /// 发送人 ID (必填)
    #[serde(rename = "fromUserId")]
    pub from_user_id: String,
    /// 接收人 ID 列表，最多 100 个 (必填)
    #[serde(rename = "toUserId")]
    pub to_user_ids: Vec<String>,
    /// 消息类型 (必填)
    #[serde(rename = "objectName")]
    pub object_name: String,
    /// 消息内容 JSON (必填)
    pub content: String,
    /// 推送内容 (可选)
    #[serde(rename = "pushContent", skip_serializing_if = "Option::is_none")]
    pub push_content: Option<String>,
    /// 推送附加数据 (可选)
    #[serde(rename = "pushData", skip_serializing_if = "Option::is_none")]
    pub push_data: Option<String>,
    /// 推送扩展配置 (可选)
    #[serde(rename = "pushExt", skip_serializing_if = "Option::is_none")]
    pub push_ext: Option<String>,
    /// 是否在服务端存储消息 (可选)
    #[serde(rename = "isPersisted", skip_serializing_if = "Option::is_none")]
    pub is_persisted: Option<i32>,
    /// 是否算作未读消息数 (可选)
    #[serde(rename = "isCounted", skip_serializing_if = "Option::is_none")]
    pub is_counted: Option<i32>,
    /// iOS 静默推送开关 (可选)
    #[serde(rename = "contentAvailable", skip_serializing_if = "Option::is_none")]
    pub content_available: Option<i32>,
    /// 是否禁用推送 (可选)
    #[serde(rename = "disablePush", skip_serializing_if = "Option::is_none")]
    pub disable_push: Option<bool>,
    /// 幂等标识 (可选)
    #[serde(rename = "msgRandom", skip_serializing_if = "Option::is_none")]
    pub msg_random: Option<i64>,
}

impl SystemMessage {
    pub fn new(
        from_user_id: impl Into<String>,
        object_name: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            from_user_id: from_user_id.into(),
            object_name: object_name.into(),
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn to_user(mut self, user_id: impl Into<String>) -> Self {
        self.to_user_ids.push(user_id.into());
        self
    }

    pub fn to_users(mut self, user_ids: Vec<String>) -> Self {
        self.to_user_ids = user_ids;
        self
    }
}

/// 撤回消息参数
#[derive(Debug, Clone, Serialize)]
pub struct RecallMessage {
    /// 会话类型
    #[serde(rename = "conversationType")]
    pub conversation_type: i32,
    /// 发送人 ID
    #[serde(rename = "fromUserId")]
    pub from_user_id: String,
    /// 目标 ID
    #[serde(rename = "targetId")]
    pub target_id: String,
    /// 消息 UID
    #[serde(rename = "messageUID")]
    pub message_uid: String,
    /// 发送时间
    #[serde(rename = "sentTime")]
    pub sent_time: i64,
    /// 是否删除消息
    #[serde(rename = "isDelete", skip_serializing_if = "Option::is_none")]
    pub is_delete: Option<i32>,
    /// 是否通知管理员
    #[serde(rename = "isAdmin", skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<i32>,
}

// ============================================================================
// API 实现
// ============================================================================

impl RongCloud {
    /// 发送单聊消息 (使用完整参数结构)
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/message/private/publish
    pub async fn send_private_message(
        &self,
        message: &PrivateMessage,
    ) -> Result<RcResponse<()>, RongCloudError> {
        self.post(
            super::endpoints::MESSAGE_PRIVATE_PUBLISH,
            message,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// 发送单聊消息 (简化版本，兼容旧接口)
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/message/private/publish
    pub async fn message_private_publish(
        &self,
        from_user_id: &str,
        to_user_ids: Vec<&str>,
        object_name: &str,
        content: &str,
        push_content: Option<&str>,
        push_data: Option<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = vec![
            ("fromUserId", from_user_id.to_string()),
            ("objectName", object_name.to_string()),
            ("content", content.to_string()),
        ];
        for id in to_user_ids {
            params.push(("toUserId", id.to_string()));
        }
        if let Some(pc) = push_content {
            params.push(("pushContent", pc.to_string()));
        }
        if let Some(pd) = push_data {
            params.push(("pushData", pd.to_string()));
        }

        self.post(
            super::endpoints::MESSAGE_PRIVATE_PUBLISH,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// 发送群聊消息 (使用完整参数结构)
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/message/group/publish
    pub async fn send_group_message(
        &self,
        message: &GroupMessage,
    ) -> Result<RcResponse<()>, RongCloudError> {
        self.post(
            super::endpoints::MESSAGE_GROUP_PUBLISH,
            message,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// 发送群聊消息 (简化版本)
    pub async fn message_group_publish(
        &self,
        from_user_id: &str,
        to_group_ids: Vec<&str>,
        object_name: &str,
        content: &str,
        push_content: Option<&str>,
        push_data: Option<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = vec![
            ("fromUserId", from_user_id.to_string()),
            ("objectName", object_name.to_string()),
            ("content", content.to_string()),
        ];
        for id in to_group_ids {
            params.push(("toGroupId", id.to_string()));
        }
        if let Some(pc) = push_content {
            params.push(("pushContent", pc.to_string()));
        }
        if let Some(pd) = push_data {
            params.push(("pushData", pd.to_string()));
        }

        self.post(
            super::endpoints::MESSAGE_GROUP_PUBLISH,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// 发送系统消息 (使用完整参数结构)
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/message/system/publish
    pub async fn send_system_message(
        &self,
        message: &SystemMessage,
    ) -> Result<RcResponse<()>, RongCloudError> {
        self.post(
            super::endpoints::MESSAGE_SYSTEM_PUBLISH,
            message,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// 发送系统消息 (简化版本)
    pub async fn message_system_publish(
        &self,
        from_user_id: &str,
        to_user_ids: Vec<&str>,
        object_name: &str,
        content: &str,
        push_content: Option<&str>,
        push_data: Option<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = vec![
            ("fromUserId", from_user_id.to_string()),
            ("objectName", object_name.to_string()),
            ("content", content.to_string()),
        ];
        for id in to_user_ids {
            params.push(("toUserId", id.to_string()));
        }
        if let Some(pc) = push_content {
            params.push(("pushContent", pc.to_string()));
        }
        if let Some(pd) = push_data {
            params.push(("pushData", pd.to_string()));
        }

        self.post(
            super::endpoints::MESSAGE_SYSTEM_PUBLISH,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// 撤回消息
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/message/recall
    pub async fn message_recall(
        &self,
        conversation_type: i32,
        from_user_id: &str,
        target_id: &str,
        message_uid: &str,
        sent_time: i64,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = RecallMessage {
            conversation_type,
            from_user_id: from_user_id.to_string(),
            target_id: target_id.to_string(),
            message_uid: message_uid.to_string(),
            sent_time,
            is_delete: None,
            is_admin: None,
        };
        self.post(
            super::endpoints::MESSAGE_RECALL,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// 获取历史消息日志
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/message/history
    pub async fn message_history(
        &self,
        date: &str, // 格式: 2014010101 (年月日时)
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("date", date)];
        self.post(
            super::endpoints::MESSAGE_HISTORY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// 删除历史消息日志
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/message/history/delete
    pub async fn message_history_delete(
        &self,
        date: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("date", date)];
        self.post(
            super::endpoints::MESSAGE_HISTORY_DELETE,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::RongCloudConfig;

    #[tokio::test]
    async fn test_message_publish() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock_private = server
            .mock("POST", "/message/private/publish.json")
            .with_status(200)
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;

        let client_res = client
            .message_private_publish(
                "u1",
                vec!["u2"],
                "RC:TxtMsg",
                "{\"content\":\"hello\"}",
                None,
                None,
            )
            .await;
        mock_private.assert_async().await;
        assert!(client_res.is_ok());

        let mock_group = server
            .mock("POST", "/message/group/publish.json")
            .with_status(200)
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;
        let group_res = client
            .message_group_publish("u1", vec!["g1"], "RC:TxtMsg", "content", None, None)
            .await;
        mock_group.assert_async().await;
        assert!(group_res.is_ok());
    }

    #[test]
    fn test_private_message_builder() {
        let msg = PrivateMessage::new("sender", "RC:TxtMsg", "{\"content\":\"hello\"}")
            .to_user("user1")
            .to_user("user2")
            .is_persisted(1)
            .is_counted(1)
            .verify_blacklist(0)
            .is_include_sender(0)
            .disable_push(false)
            .expansion(true)
            .disable_update_last_msg(true);

        assert_eq!(msg.from_user_id, "sender");
        assert_eq!(msg.to_user_ids.len(), 2);
        assert_eq!(msg.is_persisted, Some(1));
        assert_eq!(msg.disable_update_last_msg, Some(true));
    }

    #[test]
    fn test_group_message_builder() {
        let msg = GroupMessage::new("sender", "RC:TxtMsg", "content")
            .to_group("group1")
            .to_groups(vec!["g1".to_string(), "g2".to_string()])
            .is_include_sender(1)
            .is_mentioned(1)
            .disable_push(true);

        assert_eq!(msg.to_group_ids.len(), 2);
        assert_eq!(msg.is_mentioned, Some(1));
    }
}
