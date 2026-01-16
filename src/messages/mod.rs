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

// 融云即时通讯（IM）服务提供丰富的预定义消息类型（或“内置消息类型），以简化即时通讯应用的开发，提高开发效率。

// 一个预定义消息类型（或“内置消息类型）具有唯一的类型标识（ObjectName），且消息内容必须符合预定义的内容体结构。不同消息类型在服务端和客户端具有不同的处理逻辑。
// 消息属性

// 不同消息类型在服务端和客户端具有不同的处理逻辑，具体体现为是否存储、是否计入未读数、是否支持离线消息机制、是否支持推送等几个属性。

//     客户端计数与存储策略：计数策略是指是否影响未读消息数。如接收的消息影响会话未读数，则会话列表中会话的未读消息数 +1。存储策略是指发送、接收该消息后，客户端本地数据库是否存储该消息。
//     离线消息缓存：即时通讯服务端在接收者不在线时，是否对单聊、群聊、系统会话中的消息进行缓存，默认最长缓存 7 天。接收人在 7 天内上线，均可接收到该消息。超过 7 天后，消息被离线缓存淘汰。
//     远程推送通知：在接收者不在线时，是否默认为该条消息触发远程推送通知。支持远程推送通知的一个必要条件是该条消息设置了推送通知标题和推送通知内容。即时通讯服务为部分用户常用的消息类型预置了推送通知标题和通知内容。除非已明确声明不支持推送，其他消息类型如果在发送消息时主动设置了推送通知标题和通知内容，则可在接收者不在线时触发远程推送通知。

//     重要

//         客户端的计数行为只影响会话未读数，不影响 App 应用角标显示的未读数。
//         通过服务端 API 发送单聊、群聊、聊天室、超级群、系统消息，默认仅会存入收件人在服务端的历史消息记录，且该行为依赖对应的云端存储服务。如果云端存储服务不可用，则无法存入历史消息记录。如果需要同时在发件人在服务端的历史消息记录中存储该消息，请参考客户端如何同步已发消息。
//         超级群业务和聊天室业务不支持离线消息缓存机制。
//         Web 端 和 小程序端因本地存储不可靠，不支持客户端消息存储。
//         由于 Web、小程序、PC 端没有推送服务平台，无法收到推送提醒。

// 消息分类

// 为了便于理解，我们将预定义的消息类型大致分为以下几类：

//     用户内容类消息：用户内容类消息包含了用户之间可能发送的消息内容，包含文本、图片、GIF、语音、文件、小视频、位置、引用、合并转发等类型。
//     通知类消息：表示一个通知信息，可能需要展现在聊天界面上，如提示条通知。
//     状态类消息：表示一个状态，用来实现输入状态（提示“对方正在输入”）、单聊会话已读通知等功能。
//     信令类消息：即时通讯服务在实现自身业务功能时使用的，应用程序一般不需要对其做任何处理。

// 每个分类下均提供了多种预定义的消息类型。如果客户端 SDK 发送预定义的消息类型，可直接构建消息对象，设置相应类型的消息内容体并发送。如果使用服务端 API，需要指定需要发送的消息类型的类型标识，并严格按照预定义的消息类型内容体结构要求传入消息内容。
// 用户内容类消息

// 用户内容类消息包含了用户之间可能发送的消息内容，包含文本、图片、GIF、语音、文件、小视频、位置、引用、合并转发等类型。

// 即时通讯服务端为用户内容类的消息类型预置了推送通知标题和通知内容。如果发送消息时未提供自定义的推送通知标题和通知内容，则默认使用预置的推送通知标题和通知内容。默认的推送通知标题与内容可参见下表中各消息类型文档。
// 消息类型	ObjectName	客户端计数与存储策略	离线消息缓存	远程推送通知
// 文本消息	RC:TxtMsg	ISCOUNTED：在客户端存储，且计入未读数	支持	默认已支持推送
// 高清语音消息	RC:HQVCMsg	ISCOUNTED：在客户端存储，且计入未读数	支持	默认已支持推送
// 图片消息	RC:ImgMsg	ISCOUNTED：在客户端存储，且计入未读数	支持	默认已支持推送
// GIF 图片消息	RC:GIFMsg	ISCOUNTED：在客户端存储，且计入未读数	支持	默认已支持推送
// 图文消息	RC:ImgTextMsg	ISCOUNTED：在客户端存储，且计入未读数	支持	默认已支持推送
// 文件消息	RC:FileMsg	ISCOUNTED：在客户端存储，且计入未读数	支持	默认已支持推送
// 位置消息	RC:LBSMsg	ISCOUNTED：在客户端存储，且计入未读数	支持	默认已支持推送
// 小视频消息	RC:SightMsg	ISCOUNTED：在客户端存储，且计入未读数	支持	默认已支持推送
// 引用消息	RC:ReferenceMsg	ISCOUNTED：在客户端存储，且计入未读数	支持	默认已支持推送
// 合并转发消息	RC:CombineMsg	ISCOUNTED：在客户端存储，且计入未读数	支持	默认已支持推送
// 通知类消息

// 通知信息一般需要展示在聊天界面上，如“张三加入了群组”。此类消息不增加未读消息计数。

// 即时通讯服务端没有为通知类的消息类型预置推送通知标题和通知内容。如果发送消息时未提供自定义的推送通知标题和通知内容，即使收件人不在线，默认也不会触发远程推送。如需支持推送，请传入自定义的推送通知内容。
// 消息类型	ObjectName	客户端计数与存储策略	离线消息缓存	远程推送通知
// 撤回通知消息	RC:RcNtf	ISPERSISTED：在客户端存储，不计入未读数	支持	默认未支持推送
// 联系人(好友)通知消息	RC:ContactNtf	ISPERSISTED：在客户端存储，不计入未读数	支持	默认未支持推送
// 资料通知消息	RC:ProfileNtf	ISPERSISTED：在客户端存储，不计入未读数	支持	默认未支持推送
// 提示条通知消息	RC:InfoNtf	ISPERSISTED：在客户端存储，不计入未读数	支持	默认未支持推送
// 群组通知消息	RC:GrpNtf	ISPERSISTED：在客户端存储，不计入未读数	支持	默认未支持推送
// 命令提醒消息	RC:CmdNtf	ISPERSISTED：在客户端存储，不计入未读数	支持	默认未支持推送
// 状态类消息及属性

// 表示的是即时的状态，例如输入状态，仅当用户在线时可以接收。因为状态消息在客户端与服务端均不会存储，如果接收方不在线，则无法再收到该状态消息。如果使用即时通讯服务端 API 发送，请使用发送状态消息接口：发送单聊状态消息、发送群聊状态消息。
// 消息类型	ObjectName	客户端计数与存储策略	支持离线消息缓存	远程推送通知
// 正在输入状态消息	RC:TypSts	STATUS：在客户端不存储，不计入未读数	不支持	不支持推送
// 信令类消息
// 提示

// 即时通讯服务在实现 SDK 自身业务功能时使用。一般由 SDK 内部或即时通讯服务端发送，开发者不需要对其做任何处理。

// 一般为需要确保收到，但不需要展示的消息，例如运营平台向终端发送的指令信息。如果消息接收方不在线，再次上线时可通过离线消息收到。全量消息路由数据中会包含这些类型的消息。消息回调服务支持配置这些消息类型。
// 信令类消息

// 即时通讯服务端为部分信令类的消息类型预置了推送通知标题和通知内容。如果发送消息时未提供自定义的推送通知标题和通知内容，则默认使用预置的推送通知标题和通知内容。默认的推送通知标题与内容可参见下表中各消息类型文档。
// 消息类型	ObjectName	客户端计数与存储策略	支持离线消息缓存	远程推送通知
// 命令消息	RC:CmdMsg	NONE：在客户端不存储，不计入未读数	支持	默认未支持推送
// 撤回命令消息	RC:RcCmd	NONE：在客户端不存储，不计入未读数	支持	默认已支持推送
// 单聊已读回执消息	RC:ReadNtf	NONE：在客户端不存储，不计入未读数	支持	默认未支持推送
// 群聊已读回执请求消息	RC:RRReqMsg	NONE：在客户端不存储，不计入未读数	支持	默认未支持推送
// 群聊已读回执响应消息	RC:RRRspMsg	NONE：在客户端不存储，不计入未读数	支持	默认未支持推送
// 多端已读状态同步消息	RC:SRSMsg	NONE：在客户端不存储，不计入未读数	支持	默认未支持推送
// 聊天室属性通知消息	RC:chrmKVNotiMsg	NONE：在客户端不存储，不计入未读数	支持	默认未支持推送
// 音视频信令类消息

// 即时通讯服务端为音视频信令类的消息类型预置了推送通知标题和通知内容。如果发送消息时未提供自定义的推送通知标题和通知内容，则默认使用预置的推送通知标题和通知内容。
// 消息类型	ObjectName	客户端计数与存储策略	支持离线消息缓存	远程推送通知
// 实时音视频接受信令	RC:VCAccept	NONE：在客户端不存储，不计入未读数	支持	默认已支持推送
// 实时音视频挂断信令	RC:VCHangup	NONE：在客户端不存储，不计入未读数	支持	默认已支持推送
// 实时音视频邀请信令	RC:VCInvite	NONE：在客户端不存储，不计入未读数	支持	默认已支持推送
// 实时音视频切换信令	RC:VCModifyMedia	NONE：在客户端不存储，不计入未读数	支持	默认已支持推送
// 实时音视频成员变化信令	RC:VCModifyMem 	NONE：在客户端不存储，不计入未读数	支持	默认已支持推送
// 实时音视频响铃信令	RC:VCRinging	NONE：在客户端不存储，不计入未读数	支持	默认已支持推送
// 消息扩展功能消息

// 消息扩展功能消息 ObjectName 为 RC:MsgExMsg。在超级群消息业务中，设置、删除消息扩展信息时，可通过消息回调或全量消息路由接收到消息扩展功能类消息。客户端无法收到该类消息。
// 流式消息
// 流式消息 ObjectName 为 RC:StreamMsg，支持单聊和群聊的发送流式消息，客户端打字机效果实时展示，客户端从 5.16.1 版本开始支持。

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
