//! API 端点路径常量
//!
//! 集中定义所有融云 API 的 URL 路径

// ============================================================================
// 用户相关 API
// ============================================================================

/// 注册用户获取 Token
pub const USER_GET_TOKEN: &str = "/user/getToken.json";
/// 刷新用户信息
pub const USER_REFRESH: &str = "/user/refresh.json";
/// 获取用户信息
pub const USER_INFO: &str = "/user/info.json";
/// 作废 Token
pub const USER_TOKEN_EXPIRE: &str = "/user/token/expire.json";
/// 注销用户
pub const USER_DEACTIVATE: &str = "/user/deactivate.json";
/// 查询已注销用户
pub const USER_DEACTIVATE_QUERY: &str = "/user/deactivate/query.json";
/// 重新激活用户
pub const USER_REACTIVATE: &str = "/user/reactivate.json";
/// 封禁用户
pub const USER_BLOCK: &str = "/user/block.json";
/// 解除用户封禁
pub const USER_UNBLOCK: &str = "/user/unblock.json";
/// 获取封禁用户列表
pub const USER_BLOCK_QUERY: &str = "/user/block/query.json";
/// 检查用户在线状态
pub const USER_CHECK_ONLINE: &str = "/user/checkOnline.json";
/// 设置用户单聊禁言
pub const USER_CHAT_FB_SET: &str = "/user/chat/fb/set.json";
/// 查询单聊禁言用户列表
pub const USER_CHAT_FB_QUERY: &str = "/user/chat/fb/querylist.json";
/// 设置用户标签
pub const USER_TAG_SET: &str = "/user/tag/set.json";
/// 批量设置用户标签
pub const USER_TAG_BATCH_SET: &str = "/user/tag/batch/set.json";
/// 获取用户标签
pub const USER_TAGS_GET: &str = "/user/tags/get.json";

// ============================================================================
// 用户黑/白名单 API
// ============================================================================

/// 添加用户到黑名单
pub const USER_BLACKLIST_ADD: &str = "/user/blacklist/add.json";
/// 从黑名单中移除用户
pub const USER_BLACKLIST_REMOVE: &str = "/user/blacklist/remove.json";
/// 获取用户黑名单列表
pub const USER_BLACKLIST_QUERY: &str = "/user/blacklist/query.json";
/// 为用户开启白名单
pub const USER_WHITE_SETTING_SET: &str = "/user/whitesetting/set.json";
/// 查询用户白名单服务状态
pub const USER_WHITE_SETTING_QUERY: &str = "/user/whitesetting/query.json";
/// 添加用户到白名单
pub const USER_WHITELIST_ADD: &str = "/user/whitelist/add.json";
/// 从用户白名单中移除
pub const USER_WHITELIST_REMOVE: &str = "/user/whitelist/remove.json";
/// 获取用户白名单列表
pub const USER_WHITELIST_QUERY: &str = "/user/whitelist/query.json";

// ============================================================================
// 用户信息托管 API
// ============================================================================

/// 设置用户信息
pub const USER_PROFILE_SET: &str = "/user/profile/set.json";
/// 清理用户资料
pub const USER_PROFILE_CLEAN: &str = "/user/profile/clean.json";
/// 批量查询用户资料
pub const USER_PROFILE_BATCH_QUERY: &str = "/user/profile/batch/query.json";
/// 分页获取应用全部用户列表
pub const USER_PROFILE_QUERY: &str = "/user/profile/query.json";

// ============================================================================
// 好友管理 API
// ============================================================================

/// 添加好友
pub const FRIEND_ADD: &str = "/friend/add.json";
/// 解除好友
pub const FRIEND_DELETE: &str = "/friend/delete.json";
/// 清理好友
pub const FRIEND_CLEAN: &str = "/friend/clean.json";
/// 设置好友自定义属性
pub const FRIEND_PROFILE_SET: &str = "/friend/profile/set.json";
/// 获取用户好友列表
pub const FRIEND_GET: &str = "/friend/get.json";
/// 检查好友关系
pub const FRIEND_CHECK: &str = "/friend/check.json";
/// 设置添加好友验证等级
pub const FRIEND_PERMISSION_SET: &str = "/friend/permission/set.json";
/// 查询添加好友验证等级
pub const FRIEND_PERMISSION_GET: &str = "/friend/permission/get.json";

// ============================================================================
// 消息管理 API
// ============================================================================

/// 发送单聊消息
pub const MESSAGE_PRIVATE_PUBLISH: &str = "/message/private/publish.json";
/// 发送单聊模板消息
pub const MESSAGE_PRIVATE_TEMPLATE: &str = "/message/private/publish_template.json";
/// 发送单聊状态消息
pub const STATUS_MESSAGE_PRIVATE: &str = "/statusmessage/private/publish.json";
/// 发送群聊消息
pub const MESSAGE_GROUP_PUBLISH: &str = "/message/group/publish.json";
/// 发送群聊状态消息
pub const STATUS_MESSAGE_GROUP: &str = "/statusmessage/group/publish.json";
/// 发送超级群消息
pub const MESSAGE_ULTRA_GROUP_PUBLISH: &str = "/message/ultragroup/publish.json";
/// 设置单群聊消息扩展
pub const MESSAGE_EXPANSION_SET: &str = "/message/expansion/set.json";
/// 删除单群聊消息扩展
pub const MESSAGE_EXPANSION_DELETE: &str = "/message/expansion/delete.json";
/// 获取单群聊消息扩展
pub const MESSAGE_EXPANSION_QUERY: &str = "/message/expansion/query.json";
/// 撤回消息
pub const MESSAGE_RECALL: &str = "/message/recall.json";
/// 获取历史消息日志
pub const MESSAGE_HISTORY: &str = "/message/history.json";
/// 删除历史消息日志
pub const MESSAGE_HISTORY_DELETE: &str = "/message/history/delete.json";
/// 清除消息
pub const CONVERSATION_MESSAGE_CLEAN: &str = "/conversation/message/history/clean.json";

// ============================================================================
// 会话管理 API
// ============================================================================

/// 会话置顶
pub const CONVERSATION_TOP_SET: &str = "/conversation/top/set.json";

// ============================================================================
// 系统通知 API
// ============================================================================

/// 发送系统通知普通消息
pub const MESSAGE_SYSTEM_PUBLISH: &str = "/message/system/publish.json";
/// 发送系统通知模板消息
pub const MESSAGE_SYSTEM_TEMPLATE: &str = "/message/system/publish_template.json";
/// 发送全量用户落地通知
pub const MESSAGE_BROADCAST: &str = "/message/broadcast.json";
/// 发送在线用户广播
pub const MESSAGE_ONLINE_BROADCAST: &str = "/message/online/broadcast.json";
/// 发送推送
pub const PUSH: &str = "/push.json";
/// 撤回全量用户落地通知
pub const MESSAGE_BROADCAST_RECALL: &str = "/message/broadcast/recall.json";
/// 发送指定用户不落地通知
pub const PUSH_USER: &str = "/push/user.json";

// ============================================================================
// 群组管理 API
// ============================================================================

/// 创建群组
pub const GROUP_CREATE: &str = "/group/create.json";
/// 解散群组
pub const GROUP_DISMISS: &str = "/group/dismiss.json";
/// 加入群组
pub const GROUP_JOIN: &str = "/group/join.json";
/// 退出群组
pub const GROUP_QUIT: &str = "/group/quit.json";
/// 查询群组成员
pub const GROUP_USER_QUERY: &str = "/group/user/query.json";
/// 同步用户所在群组
pub const GROUP_SYNC: &str = "/group/sync.json";
/// 查询用户所在群组
pub const USER_GROUP_QUERY: &str = "/user/group/query.json";
/// 刷新群组信息
pub const GROUP_REFRESH: &str = "/group/refresh.json";

// ============================================================================
// 群组管理（用户信息托管）API
// ============================================================================

/// 创建群组(托管)
pub const ENTRUST_GROUP_CREATE: &str = "/entrust/group/create.json";
/// 设置群组资料
pub const ENTRUST_GROUP_PROFILE_UPDATE: &str = "/entrust/group/profile/update.json";
/// 批量查询群组资料
pub const ENTRUST_GROUP_PROFILE_QUERY: &str = "/entrust/group/profile/query.json";
/// 加入群组(托管)
pub const ENTRUST_GROUP_JOIN: &str = "/entrust/group/join.json";
/// 导入群组
pub const ENTRUST_GROUP_IMPORT: &str = "/entrust/group/import.json";
/// 退出群组(托管)
pub const ENTRUST_GROUP_QUIT: &str = "/entrust/group/quit.json";
/// 踢出群组
pub const ENTRUST_GROUP_MEMBER_KICK: &str = "/entrust/group/member/kick.json";
/// 指定用户踢出所有群组
pub const ENTRUST_GROUP_MEMBER_KICK_ALL: &str = "/entrust/group/member/kick/all.json";
/// 解散群组(托管)
pub const ENTRUST_GROUP_DISMISS: &str = "/entrust/group/dismiss.json";
/// 转让群组
pub const ENTRUST_GROUP_TRANSFER_OWNER: &str = "/entrust/group/transfer/owner.json";
/// 设置群管理员
pub const ENTRUST_GROUP_MANAGER_ADD: &str = "/entrust/group/manager/add.json";
/// 移除群管理员
pub const ENTRUST_GROUP_MANAGER_REMOVE: &str = "/entrust/group/manager/remove.json";
/// 分页获取群成员信息
pub const ENTRUST_GROUP_MEMBER_QUERY: &str = "/entrust/group/member/query.json";
/// 获取指定群成员信息
pub const ENTRUST_GROUP_MEMBER_SPECIFIC_QUERY: &str = "/entrust/group/member/specific/query.json";
/// 设置群成员资料
pub const ENTRUST_GROUP_MEMBER_SET: &str = "/entrust/group/member/set.json";
/// 设置用户指定群特别关注用户
pub const ENTRUST_GROUP_MEMBER_FOLLOW: &str = "/entrust/group/member/follow.json";
/// 删除用户指定群特别关注用户
pub const ENTRUST_GROUP_MEMBER_UNFOLLOW: &str = "/entrust/group/member/unfollow.json";
/// 查询用户指定群特别关注成员列表
pub const ENTRUST_GROUP_MEMBER_FOLLOWED_GET: &str = "/entrust/group/member/followed/get.json";
/// 设置用户指定群组名称备注名
pub const ENTRUST_GROUP_REMARKNAME_SET: &str = "/entrust/group/remarkname/set.json";
/// 删除用户指定群组名称备注名
pub const ENTRUST_GROUP_REMARKNAME_DELETE: &str = "/entrust/group/remarkname/delete.json";
/// 查询用户指定群组名称备注名
pub const ENTRUST_GROUP_REMARKNAME_QUERY: &str = "/entrust/group/remarkname/query.json";
/// 分页查询应用下群组信息
pub const ENTRUST_GROUP_QUERY: &str = "/entrust/group/query.json";
/// 分页查询用户加入的群组
pub const ENTRUST_JOINED_GROUP_QUERY: &str = "/entrust/joined/group/query.json";

// ============================================================================
// 群组禁言服务 API
// ============================================================================

/// 禁言指定群成员
pub const GROUP_USER_GAG_ADD: &str = "/group/user/gag/add.json";
/// 取消指定群成员禁言
pub const GROUP_USER_GAG_ROLLBACK: &str = "/group/user/gag/rollback.json";
/// 查询群成员禁言列表
pub const GROUP_USER_GAG_LIST: &str = "/group/user/gag/list.json";
/// 设置群组全体禁言
pub const GROUP_BAN_ADD: &str = "/group/ban/add.json";
/// 取消群组全体禁言
pub const GROUP_BAN_ROLLBACK: &str = "/group/ban/rollback.json";
/// 查询群组全体禁言
pub const GROUP_BAN_QUERY: &str = "/group/ban/query.json";
/// 加入群组全体禁言白名单
pub const GROUP_USER_BAN_WHITELIST_ADD: &str = "/group/user/ban/whitelist/add.json";
/// 移出群组全体禁言白名单
pub const GROUP_USER_BAN_WHITELIST_ROLLBACK: &str = "/group/user/ban/whitelist/rollback.json";
/// 查询群组全体禁言白名单
pub const GROUP_USER_BAN_WHITELIST_QUERY: &str = "/group/user/ban/whitelist/query.json";

// ============================================================================
// 聊天室管理 API
// ============================================================================

/// 创建聊天室
pub const CHATROOM_CREATE: &str = "/chatroom/create_new.json";
/// 设置聊天室销毁类型
pub const CHATROOM_DESTROY_SET: &str = "/chatroom/destroy/set.json";
/// 销毁聊天室
pub const CHATROOM_DESTROY: &str = "/chatroom/destroy.json";
/// 查询聊天室信息
pub const CHATROOM_GET: &str = "/chatroom/get.json";
/// 绑定音视频房间
pub const CHATROOM_CORRELATION_RTC: &str = "/chatroom/correlation/rtc.json";
/// 保活聊天室
pub const CHATROOM_KEEPALIVE_ADD: &str = "/chatroom/keepalive/add.json";
/// 取消保活聊天室
pub const CHATROOM_KEEPALIVE_REMOVE: &str = "/chatroom/keepalive/remove.json";
/// 查询保活聊天室
pub const CHATROOM_KEEPALIVE_QUERY: &str = "/chatroom/keepalive/query.json";
/// 获取聊天室成员
pub const CHATROOM_USER_QUERY: &str = "/chatroom/user/query.json";
/// 查询是否在聊天室中
pub const CHATROOM_USER_EXIST: &str = "/chatroom/user/exist.json";
/// 批量查询是否在聊天室中
pub const CHATROOM_USERS_EXIST: &str = "/chatroom/users/exist.json";
/// 禁言指定聊天室用户
pub const CHATROOM_USER_GAG_ADD: &str = "/chatroom/user/gag/add.json";
/// 取消禁言指定聊天室用户
pub const CHATROOM_USER_GAG_ROLLBACK: &str = "/chatroom/user/gag/rollback.json";
/// 查询聊天室禁言用户列表
pub const CHATROOM_USER_GAG_LIST: &str = "/chatroom/user/gag/list.json";
/// 设置聊天室全体禁言
pub const CHATROOM_BAN_ADD: &str = "/chatroom/ban/add.json";
/// 取消聊天室全体禁言
pub const CHATROOM_BAN_ROLLBACK: &str = "/chatroom/ban/rollback.json";
/// 查询聊天室全体禁言列表
pub const CHATROOM_BAN_QUERY: &str = "/chatroom/ban/query.json";
/// 查询聊天室全体禁言状态
pub const CHATROOM_BAN_CHECK: &str = "/chatroom/ban/check.json";
/// 加入聊天室全体禁言白名单
pub const CHATROOM_USER_BAN_WHITELIST_ADD: &str = "/chatroom/user/ban/whitelist/add.json";
/// 移出聊天室全体禁言白名单
pub const CHATROOM_USER_BAN_WHITELIST_ROLLBACK: &str = "/chatroom/user/ban/whitelist/rollback.json";
/// 查询聊天室全体禁言白名单
pub const CHATROOM_USER_BAN_WHITELIST_QUERY: &str = "/chatroom/user/ban/whitelist/query.json";
/// 全局禁言用户
pub const CHATROOM_USER_BAN_ADD: &str = "/chatroom/user/ban/add.json";
/// 取消全局禁言用户
pub const CHATROOM_USER_BAN_REMOVE: &str = "/chatroom/user/ban/remove.json";
/// 查询全局禁言用户列表
pub const CHATROOM_USER_BAN_QUERY: &str = "/chatroom/user/ban/query.json";
/// 封禁聊天室用户
pub const CHATROOM_USER_BLOCK_ADD: &str = "/chatroom/user/block/add.json";
/// 解除封禁聊天室用户
pub const CHATROOM_USER_BLOCK_ROLLBACK: &str = "/chatroom/user/block/rollback.json";
/// 查询聊天室封禁用户
pub const CHATROOM_USER_BLOCK_LIST: &str = "/chatroom/user/block/list.json";

// ============================================================================
// 聊天室属性（KV）API
// ============================================================================

/// 设置聊天室属性
pub const CHATROOM_ENTRY_SET: &str = "/chatroom/entry/set.json";
/// 批量设置聊天室属性
pub const CHATROOM_ENTRY_BATCH_SET: &str = "/chatroom/entry/batch/set.json";
/// 删除聊天室属性
pub const CHATROOM_ENTRY_REMOVE: &str = "/chatroom/entry/remove.json";
/// 批量删除聊天室属性
pub const CHATROOM_ENTRY_BATCH_REMOVE: &str = "/chatroom/entry/batch/remove.json";
/// 查询聊天室属性
pub const CHATROOM_ENTRY_QUERY: &str = "/chatroom/entry/query.json";

// ============================================================================
// 聊天室消息优先级 API
// ============================================================================

/// 添加低级别消息类型
pub const CHATROOM_MESSAGE_PRIORITY_ADD: &str = "/chatroom/message/priority/add.json";
/// 移除低级别消息类型
pub const CHATROOM_MESSAGE_PRIORITY_REMOVE: &str = "/chatroom/message/priority/remove.json";
/// 查询低级别消息类型
pub const CHATROOM_MESSAGE_PRIORITY_QUERY: &str = "/chatroom/message/priority/query.json";

// ============================================================================
// 聊天室白名单 API
// ============================================================================

/// 加入聊天室用户白名单
pub const CHATROOM_USER_WHITELIST_ADD: &str = "/chatroom/user/whitelist/add.json";
/// 移出聊天室用户白名单
pub const CHATROOM_USER_WHITELIST_REMOVE: &str = "/chatroom/user/whitelist/remove.json";
/// 查询聊天室用户白名单
pub const CHATROOM_USER_WHITELIST_QUERY: &str = "/chatroom/user/whitelist/query.json";
/// 加入聊天室消息白名单
pub const CHATROOM_WHITELIST_ADD: &str = "/chatroom/whitelist/add.json";
/// 移出聊天室消息白名单
pub const CHATROOM_WHITELIST_DELETE: &str = "/chatroom/whitelist/delete.json";
/// 查询聊天室消息白名单
pub const CHATROOM_WHITELIST_QUERY: &str = "/chatroom/whitelist/query.json";

// ============================================================================
// 内容审核 API
// ============================================================================

/// 添加消息敏感词
pub const SENSITIVE_WORD_ADD: &str = "/sensitiveword/add.json";
/// 批量添加消息敏感词
pub const SENSITIVE_WORD_BATCH_ADD: &str = "/sensitiveword/batch/add.json";
/// 移除消息敏感词
pub const SENSITIVE_WORD_DELETE: &str = "/sensitiveword/delete.json";
/// 批量移除消息敏感词
pub const SENSITIVE_WORD_BATCH_DELETE: &str = "/sensitiveword/batch/delete.json";
/// 查询消息敏感词
pub const SENSITIVE_WORD_LIST: &str = "/sensitiveword/list.json";

// ============================================================================
// 推送与通知管理 API
// ============================================================================

/// 设置指定会话免打扰
pub const CONVERSATION_NOTIFICATION_SET: &str = "/conversation/notification/set.json";
/// 查询指定会话免打扰
pub const CONVERSATION_NOTIFICATION_GET: &str = "/conversation/notification/get.json";
/// 设置指定会话类型免打扰
pub const CONVERSATION_TYPE_NOTIFICATION_SET: &str = "/conversation/type/notification/set.json";
/// 查询指定会话类型免打扰
pub const CONVERSATION_TYPE_NOTIFICATION_GET: &str = "/conversation/type/notification/get.json";
/// 设置用户免打扰时段
pub const USER_BLOCK_PUSH_PERIOD_SET: &str = "/user/blockPushPeriod/set.json";
/// 删除用户免打扰时段
pub const USER_BLOCK_PUSH_PERIOD_DELETE: &str = "/user/blockPushPeriod/delete.json";
/// 查询用户免打扰时段
pub const USER_BLOCK_PUSH_PERIOD_GET: &str = "/user/blockPushPeriod/get.json";
/// 推送 Plus
pub const PUSH_CUSTOM: &str = "/push/custom.json";
/// 推送聚合统计
pub const STAT_GET_DAY_PUSH_DATA: &str = "/stat/getDayPushData";
/// 单次推送统计
pub const STAT_GET_PUSH_ID_DATA: &str = "/stat/getPushIdData";
/// 设置用户级推送备注名
pub const USER_REMARKS_SET: &str = "/user/remarks/set.json";
/// 删除用户级推送备注名
pub const USER_REMARKS_DEL: &str = "/user/remarks/del.json";
/// 查询用户级推送备注名
pub const USER_REMARKS_GET: &str = "/user/remarks/get.json";
/// 设置群成员推送备注名
pub const GROUP_REMARKS_SET: &str = "/group/remarks/set.json";
/// 删除群成员推送备注名
pub const GROUP_REMARKS_DEL: &str = "/group/remarks/del.json";
/// 查询群成员推送备注名
pub const GROUP_REMARKS_GET: &str = "/group/remarks/get.json";
