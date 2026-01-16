功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
注册用户 /user/getToken.json 200 次/每秒，可调频 API 调试 API 调试
作废 Token /user/token/expire.json 100 次/每秒，可调频 API 调试 API 调试
获取用户信息 /user/info.json 100 次/每秒，可调频 API 调试 API 调试
修改用户信息 /user/refresh.json 100 次/每秒，可调频 API 调试 API 调试
注销用户 /user/deactivate.json 100 用户/每秒 暂不支持 API 调试
查询已注销用户 /user/deactivate/query.json 100 次/每秒 暂不支持 API 调试
重新激活用户 ID /user/reactivate.json 100 用户/每秒 暂不支持 API 调试
封禁用户 /user/block.json 100 次/每秒，可调频 API 调试 API 调试
解除用户封禁 /user/unblock.json 100 次/每秒，可调频 API 调试 API 调试
获取封禁用户列表 /user/block/query.json 100 次/每秒，可调频 API 调试 API 调试
用户状态 /user/checkOnline.json 100 次/每秒，可调频 API 调试 API 调试
设置用户单聊禁言 /user/chat/fb/set.json 100 次/每秒，可调频 API 调试 API 调试
查询单聊禁言用户列表 /user/chat/fb/querylist.json 100 次/每秒，可调频 API 调试 API 调试
设置用户标签 /user/tag/set.json 100 次/每秒，可调频 API 调试 API 调试
批量设置用户标签 /user/tag/batch/set.json 10 次/每秒，可调频 API 调试 API 调试
获取用户标签 /user/tags/get.json 100 次/每秒，可调频 API 调试 API 调试

用户黑/白名单服务
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
添加用户到黑名单 /user/blacklist/add.json 100 次/每秒，可调频 API 调试 API 调试
从黑名单中移除用户 /user/blacklist/remove.json 100 次/每秒，可调频 API 调试 API 调试
获取某用户的黑名单列表 /user/blacklist/query.json 100 次/每秒，可调频 API 调试 API 调试
为用户开启白名单 /user/whitesetting/set.json 100 次/每秒 API 调试 API 调试
查询用户白名单服务状态 /user/whitesetting/query.json 100 次/每秒 API 调试 API 调试
添加用户到白名单 /user/whitelist/add.json 100 次/每秒，可调频 API 调试 API 调试
从用户白名单中移除用户 /user/whitelist/remove.json 100 次/每秒，可调频 API 调试 API 调试
获取用户的白名单列表 /user/whitelist/query.json 100 次/每秒，可调频 API 调试 API 调试

用户信息托管
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
设置用户信息 /user/profile/set.json 100 次/每秒，可调频 暂不支持 API 调试
清理用户资料 /user/profile/clean.json 100 次/每秒，可调频 暂不支持 API 调试
批量查询用户资料 /user/profile/batch/query.json 100 次/每秒，可调频 暂不支持 API 调试
分页获取应用全部用户列表 /user/profile/query.json 100 次/每秒 暂不支持 API 调试

好友管理（信息托管）
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
添加好友 /friend/add.json 100 次/每秒，可调频 暂不支持 API 调试
解除好友 /friend/delete.json 100 次/每秒，可调频 暂不支持 API 调试
清理好友 /friend/clean.json 100 次/每秒，可调频 暂不支持 API 调试
设置好友自定义属性 /friend/profile/set.json 100 次/每秒，可调频 暂不支持 API 调试
获取用户好友列表 /friend/get.json 100 次/每秒，可调频 暂不支持 API 调试
检查好友关系 /friend/check.json 100 次/每秒，可调频 暂不支持 API 调试
设置添加好友验证等级 /friend/permission/set.json 100 次/每秒，可调频 暂不支持 API 调试
查询添加好友验证等级 /friend/permission/get.json 100 次/每秒，可调频 暂不支持 API 调试

消息管理
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
发送单聊普通消息 /message/private/publish.json 6000 条消息/每分钟，按收件人数量计算条数，可调频 API 调试 API 调试
发送单聊模板消息 /message/private/publish_template.json 6000 条消息/每分钟，按收件人数量计算条数，可调频 API 调试 API 调试
发送单聊状态消息 /statusmessage/private/publish.json 6000 条消息/每分钟，按收件人数量计算条数，可调频 API 调试 API 调试
发送群聊消息 /message/group/publish.json 20 条/每秒，按目标群组数量计算条数，可调频 API 调试 API 调试
发送群聊状态消息 /statusmessage/group/publish.json 20 条/每秒，按目标群组数量计算条数，可调频 API 调试 API 调试
发送超级群消息 /message/ultragroup/publish.json 100 条/每秒，按目标群组数量计算条数；单个频道限 20 条/每秒，可调频 API 调试 API 调试

<!-- 发送聊天室消息 /message/chatroom/publish.json 100 条/每秒，按目标聊天室数量计算条数，可调频 API 调试 API 调试 -->
<!-- 发送全体聊天室广播消息 /message/chatroom/broadcast.json 1 次/每秒，可调频 API 调试 API 调试 -->

设置单群聊消息扩展 /message/expansion/set.json 100 次/每秒，其中群聊消息扩展最多 20 次，可调频 API 调试 API 调试
删除单群聊消息扩展 /message/expansion/delete.json 100 次/每秒，其中群聊消息扩展最多 20 次，可调频 API 调试 API 调试
获取单群聊消息扩展 /message/expansion/query.json 100 次/每秒，可调频 API 调试 API 调试
撤回消息 /message/recall.json 100 次/每秒，可调频 API 调试 API 调试
获取历史消息日志 /message/history.json 100 次/每秒 API 调试 API 调试
删除历史消息日志 /message/history/delete.json 100 次/每秒 API 调试 API 调试
清除消息 /conversation/message/history/clean.json 100 次/每秒，可调频 API 调试 API 调试
会话管理
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
会话置顶 /conversation/top/set.json 100 次/每秒 暂不支持 API 调试
系统通知
提示

下表中频率限制一栏标注「共享」的项目均使用 /push.json 接口，共享该接口频率限额，即 2 次/每小时，3 次/每自然日，可调频。
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
发送系统通知普通消息 /message/system/publish.json 100 条/每秒，按收件人数量计算条数，可调频 API 调试 API 调试
发送系统通知模板消息 /message/system/publish_template.json 100 条/每秒，按收件人数量计算条数，可调频 API 调试 API 调试
撤回单条系统通知 /message/recall.json 100 次/每秒 API 调试 API 调试
发送全量用户落地通知 /message/broadcast.json 2 次/每小时，3 次/每自然日，可调频 API 调试 API 调试
发送在线用户广播 /message/online/broadcast.json 60 次/每分钟 API 调试 API 调试
发送全量用户不落地通知 /push.json 2 次/每小时，3 次/每自然日（共享），可调频 API 调试 API 调试
发送标签用户通知 /push.json 2 次/每小时，3 次/每自然日（共享），可调频 API 调试 API 调试
发送应用包名通知 /push.json 2 次/每小时，3 次/每自然日（共享），可调频 API 调试 API 调试
撤回全量用户落地通知 /message/broadcast/recall.json 2 次/每小时，3 次/每自然日 暂不支持 API 调试
发送指定用户不落地通知 /push/user.json 100 条/每秒，按收件人数量计算条数，可调频 API 调试 API 调试

群组管理
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
创建群组 /group/create.json 100 次/每秒，可调频 API 调试 API 调试
解散群组 /group/dismiss.json 100 次/每秒，可调频 API 调试 API 调试
加入群组 /group/join.json 100 次/每秒，可调频 API 调试 API 调试
退出群组 /group/quit.json 100 次/每秒，可调频 API 调试 API 调试
查询群组成员 /group/user/query.json 100 次/每秒，可调频 API 调试 API 调试
同步用户所在群组 /group/sync.json 100 次/每秒，可调频 暂不支持 API 调试
查询用户所在群组 /user/group/query.json 100 次/每秒，可调频 API 调试 API 调试
刷新群组信息 /group/refresh.json 100 次/每秒，可调频 API 调试 API 调试

群组管理（用户信息托管）
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
创建群组 /entrust/group/create.json 100 次/每秒，可调频 API 调试 API 调试
设置群组资料 /entrust/group/profile/update.json 100 次/每秒，可调频 API 调试 API 调试
批量查询群组资料 /entrust/group/profile/query.json 100 次/每秒，可调频 API 调试 API 调试
加入群组 /entrust/group/join.json 100 次/每秒，可调频 API 调试 API 调试
导入群组 /entrust/group/import.json 100 次/每秒，可调频 API 调试 API 调试
退出群组 /entrust/group/quit.json 100 次/每秒，可调频 API 调试 API 调试
踢出群组 /entrust/group/member/kick.json 100 次/每秒 API 调试 API 调试
指定用户踢出所有群组 /entrust/group/member/kick/all.json 100 次/每秒，可调频 API 调试 API 调试
解散群组 /entrust/group/dismiss.json 100 次/每秒，可调频 API 调试 API 调试
转让群组 /entrust/group/transfer/owner.json 100 次/每秒，可调频 API 调试 API 调试
设置群管理员 /entrust/group/manager/add.json 100 次/每秒，可调频 API 调试 API 调试
移除群管理员 /entrust/group/manager/remove.json 100 次/每秒，可调频 API 调试 API 调试
分页获取群成员信息 /entrust/group/member/query.json 100 次/每秒，可调频 API 调试 API 调试
获取指定群成员信息 /entrust/group/member/specific/query.json 100 次/每秒，可调频 API 调试 API 调试
设置群成员资料 /entrust/group/member/set.json 100 次/每秒，可调频 API 调试 API 调试
设置用户指定群特别关注用户 /entrust/group/member/follow.json 100 次/每秒，可调频 API 调试 API 调试
删除用户指定群组中的特别关注用户 /entrust/group/member/unfollow.json 100 次/每秒，可调频 API 调试 API 调试
查询用户指定群组特别关注成员列表 /entrust/group/member/followed/get.json 100 次/每秒，可调频 API 调试 API 调试
设置用户指定群组名称备注名 /entrust/group/remarkname/set.json 100 次/每秒，可调频 API 调试 API 调试
删除用户指定群组名称备注名 /entrust/group/remarkname/delete.json 100 次/每秒，可调频 API 调试 API 调试
查询用户指定群组名称备注名 /entrust/group/remarkname/query.json 100 次/每秒，可调频 API 调试 API 调试
分页查询应用下群组信息 /entrust/group/query.json 100 次/每秒 API 调试 API 调试
分页查询用户加入的群组 /entrust/joined/group/query.json 100 次/每秒 API 调试 API 调试
群组禁言服务
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
禁言指定群成员 /group/user/gag/add.json 100 次/每秒，可调频 API 调试 API 调试
取消指定群成员禁言 /group/user/gag/rollback.json 100 次/每秒，可调频 API 调试 API 调试
查询群成员禁言列表 /group/user/gag/list.json 100 次/每秒，可调频 API 调试 API 调试
设置群组全体禁言 /group/ban/add.json 100 次/每秒，可调频 API 调试 API 调试
取消群组全体禁言 /group/ban/rollback.json 100 次/每秒，可调频 API 调试 API 调试
查询群组全体禁言 /group/ban/query.json 100 次/每秒，可调频 API 调试 API 调试
加入群组全体禁言白名单 /group/user/ban/whitelist/add.json 100 次/每秒，可调频 API 调试 API 调试
移出群组全体禁言白名单 /group/user/ban/whitelist/rollback.json 100 次/每秒，可调频 API 调试 API 调试
查询群组全体禁言白名单 /group/user/ban/whitelist/query.json 100 次/每秒，可调频 API 调试 API 调试

<!--
超级群管理
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
创建超级群 /ultragroup/create.json 100 次/每秒，可调频 API 调试 API 调试
加入超级群 /ultragroup/join.json 100 次/每秒，可调频 API 调试 API 调试
退出超级群 /ultragroup/quit.json 100 次/每秒，可调频 API 调试 API 调试
解散超级群 /ultragroup/dis.json 100 次/每秒，可调频 API 调试 API 调试
刷新超级群信息 /ultragroup/refresh.json 100 次/每秒，可调频 API 调试 API 调试
创建频道 /ultragroup/channel/create.json 100 次/每秒，可调频 API 调试 API 调试
删除频道 /ultragroup/channel/del.json 100 次/每秒，可调频 API 调试 API 调试
查询频道列表 /ultragroup/channel/get.json 100 次/每秒，可调频 API 调试 API 调试
获取指定超级群消息内容 /ultragroup/msg/get.json 5 次/每秒 暂不支持 API 调试
修改超级群消息 /ultragroup/msg/modify.json 100 次/每分钟 暂不支持 API 调试
搜索超级群消息 /ultragroup/hismsg/query.json 100 次/每分钟 暂不支持 API 调试
设置超级群消息扩展 /ultragroup/message/expansion/set.json 100 次/每秒，可调频 暂不支持 API 调试
删除超级群消息扩展 /ultragroup/message/expansion/delete.json 100 次/每秒，可调频 暂不支持 API 调试
获取超级群消息扩展 /ultragroup/message/expansion/query.json 100 次/每秒，可调频 暂不支持 API 调试
查询用户是否为群成员 /ultragroup/member/exist.json 100 次/每秒，可调频 暂不支持 API 调试
设置群/频道默认免打扰 /ultragroup/notdisturb/set.json 100 次/每秒，可调频 暂不支持 API 调试
查询默认免打扰配置 /ultragroup/notdisturb/get.json 100 次/每秒 暂不支持 API 调试
超级群私有频道
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
变更频道类型 /ultragroup/channel/type/change.json 100 次/每秒，可调频 暂不支持 API 调试
添加私有频道成员 /ultragroup/channel/private/users/add.json 100 次/每秒，可调频 暂不支持 API 调试
删除私有频道成员 /ultragroup/channel/private/users/del.json 100 次/每秒，可调频 暂不支持 API 调试
查询私有频道成员列表 /ultragroup/channel/private/users/get.json 100 次/每秒，可调频 暂不支持 API 调试
查询用户所属的私有频道 /ultragroup/user/channel/query.json 100 次/每秒 暂不支持 API 调试
超级群用户组
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
创建用户组 /ultragroup/usergroup/add.json 100 次/每秒 暂不支持 API 调试
删除用户组 /ultragroup/usergroup/del.json 100 次/每秒 暂不支持 API 调试
查询用户组列表 /ultragroup/usergroup/query.json 100 次/每秒 暂不支持 API 调试
添加用户 /ultragroup/usergroup/user/add.json 100 次/每秒 暂不支持 API 调试
移出用户 /ultragroup/usergroup/user/del.json 100 次/每秒 暂不支持 API 调试
查询用户所属用户组 /ultragroup/user/usergroup/query.json 100 次/每秒 暂不支持 API 调试
绑定频道与用户组 /ultragroup/channel/usergroup/bind.json 100 次/每秒 暂不支持 API 调试
解绑频道与用户组 /ultragroup/channel/usergroup/unbind.json 100 次/每秒 暂不支持 API 调试
查询频道绑定的用户组 /ultragroup/channel/usergroup/query.json 100 次/每秒 暂不支持 API 调试
查询用户组绑定的频道 /ultragroup/usergroup/channel/query.json 100 次/每秒 暂不支持 API 调试
超级群禁言服务
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
禁言指定超级群成员 /ultragroup/userbanned/add.json 100 次/每秒，可调频 API 调试 API 调试
取消指定超级群成员禁言 /ultragroup/userbanned/del.json 100 次/每秒，可调频 API 调试 API 调试
查询超级群成员禁言列表 /ultragroup/userbanned/get.json 100 次/每秒，可调频 API 调试 API 调试
设置超级群全体禁言 /ultragroup/globalbanned/set.json 100 次/每秒，可调频 API 调试 API 调试
查询超级群全体禁言 /ultragroup/globalbanned/get.json 100 次/每秒，可调频 API 调试 API 调试
加入超级群全体禁言白名单 /ultragroup/banned/whitelist/add.json 100 次/每秒，可调频 API 调试 API 调试
移出超级群全体禁言白名单 /ultragroup/banned/whitelist/del.json 100 次/每秒，可调频 API 调试 API 调试
查询超级群全体禁言白名单 /ultragroup/banned/whitelist/get.json 100 次/每秒，可调频 API 调试 API 调试
聊天室管理
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
创建聊天室 /chatroom/create_new.json 100 次/每秒 API 调试 API 调试
设置聊天室销毁类型 /chatroom/destroy/set.json 100 次/每秒 暂不支持 API 调试
销毁聊天室 /chatroom/destroy.json 100 次/每秒，可调频 API 调试 API 调试
查询聊天室信息 /chatroom/get.json 100 次/每秒 API 调试 API 调试
绑定音视频房间 /chatroom/correlation/rtc.json 100 次/每秒 API 调试 API 调试
创建聊天室（已废弃） /chatroom/create.json 100 次/每秒，可调频 暂不支持 API 调试
查询聊天室信息（已废弃） /chatroom/query.json 100 次/每秒，可调频 暂不支持 API 调试
聊天室保活服务
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
保活聊天室 /chatroom/keepalive/add.json 100 次/每秒 API 调试 API 调试
取消保活聊天室 /chatroom/keepalive/remove.json 100 次/每秒 API 调试 API 调试
查询保活聊天室 /chatroom/keepalive/query.json 100 次/每秒 API 调试 API 调试
聊天室用户与禁言管理
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
获取聊天室成员 /chatroom/user/query.json 100 次/每秒，可调频 API 调试 API 调试
查询是否在聊天室中 /chatroom/user/exist.json 100 次/每秒，可调频 API 调试 API 调试
批量查询是否在聊天室中 /chatroom/users/exist.json 100 次/每秒，可调频 API 调试 API 调试
禁言指定聊天室用户 /chatroom/user/gag/add.json 100 次/每秒，可调频 API 调试 API 调试
取消禁言指定聊天室用户 /chatroom/user/gag/rollback.json 100 次/每秒，可调频 API 调试 API 调试
查询聊天室禁言用户列表 /chatroom/user/gag/list.json 100 次/每秒，可调频 API 调试 API 调试
设置聊天室全体禁言 /chatroom/ban/add.json 100 次/每秒，可调频 API 调试 API 调试
取消聊天室全体禁言 /chatroom/ban/rollback.json 100 次/每秒，可调频 API 调试 API 调试
查询聊天室全体禁言列表 /chatroom/ban/query.json 100 次/每秒，可调频 API 调试 API 调试
查询聊天室全体禁言状态 /chatroom/ban/check.json 100 次/每秒，可调频 API 调试 API 调试
加入聊天室全体禁言白名单 /chatroom/user/ban/whitelist/add.json 100 次/每秒，可调频 API 调试 API 调试
移出聊天室全体禁言白名单 /chatroom/user/ban/whitelist/rollback.json 100 次/每秒，可调频 API 调试 API 调试
查询聊天室全体禁言白名单 /chatroom/user/ban/whitelist/query.json 100 次/每秒，可调频 API 调试 API 调试
全局禁言用户 /chatroom/user/ban/add.json 100 次/每秒，可调频 API 调试 API 调试
取消全局禁言用户 /chatroom/user/ban/remove.json 100 次/每秒，可调频 API 调试 API 调试
查询全局禁言用户列表 /chatroom/user/ban/query.json 100 次/每秒，可调频 API 调试 API 调试
封禁聊天室用户 /chatroom/user/block/add.json 100 次/每秒，可调频 API 调试 API 调试
解除封禁聊天室用户 /chatroom/user/block/rollback.json 100 次/每秒，可调频 API 调试 API 调试
查询聊天室封禁用户 /chatroom/user/block/list.json 100 次/每秒，可调频 API 调试 API 调试
聊天室属性（KV）
提示

下表中频率限制一栏标注「共享」的没有独立的接口频率限额。设置单个聊天室属性与批量设置聊天室属性接口共享 100 个属性每秒的限额。删除单个聊天室属性与批量删除聊天室属性接口共享 100 个属性每秒的限额。
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
设置聊天室属性（KV） /chatroom/entry/set.json 100 个属性/每秒（共享），可调频 API 调试 API 调试
批量设置聊天室属性（KV） /chatroom/entry/batch/set.json 100 个属性/每秒（共享） 暂不支持 API 调试
删除聊天室属性（KV） /chatroom/entry/remove.json 100 个属性/每秒（共享），可调频 API 调试 API 调试
批量删除聊天室属性（KV） /chatroom/entry/batch/remove.json 100 个属性/每秒（共享） 暂不支持 API 调试
查询聊天室属性（KV） /chatroom/entry/query.json 100 次/每秒，可调频 API 调试 API 调试
聊天室消息优先级服务
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
添加低级别消息类型 /chatroom/message/priority/add.json 100 次/每秒，可调频 API 调试 API 调试
移除低级别消息类型 /chatroom/message/priority/remove.json 100 次/每秒，可调频 API 调试 API 调试
查询低级别消息类型 /chatroom/message/priority/query.json 100 次/每秒，可调频 API 调试 API 调试
聊天室白名单服务
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
加入聊天室用户白名单 /chatroom/user/whitelist/add.json 100 次/每秒，可调频 API 调试 API 调试
移出聊天室用户白名单 /chatroom/user/whitelist/remove.json 100 次/每秒，可调频 API 调试 API 调试
查询聊天室用户白名单 /chatroom/user/whitelist/query.json 100 次/每秒，可调频 API 调试 API 调试
加入聊天室消息白名单 /chatroom/whitelist/add.json 100 次/每秒，可调频 API 调试 API 调试
移出聊天室消息白名单 /chatroom/whitelist/delete.json 100 次/每秒，可调频 API 调试 API 调试
查询聊天室消息白名单 /chatroom/whitelist/query.json 100 次/每秒，可调频 API 调试 API 调试
内容审核
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
添加消息敏感词 /sensitiveword/add.json 100 次/每秒，可调频 API 调试 API 调试
批量添加消息敏感词 /sensitiveword/batch/add.json 100 次/每秒 暂不支持 API 调试
移除消息敏感词 /sensitiveword/delete.json 100 次/每秒，可调频 API 调试 API 调试
批量移除消息敏感词 /sensitiveword/batch/delete.json 100 次/每秒，可调频 API 调试 API 调试
查询消息敏感词 /sensitiveword/list.json 100 次/每秒，可调频 API 调试 API 调试

推送聚合统计 /stat/getDayPushData 1 次/每秒 API 调试 API 调试
单次推送统计 /stat/getPushIdData 1 次/每秒 API 调试 API 调试
设置用户级推送备注名 /user/remarks/set.json 100 次/每秒，可调频 暂不支持 API 调试
删除用户级推送备注名 /user/remarks/del.json 100 次/每秒，可调频 暂不支持 API 调试
查询用户级推送备注名 /user/remarks/get.json 100 次/每秒，可调频 暂不支持 API 调试
设置群成员推送备注名 /group/remarks/set.json 100 次/每秒，可调频 暂不支持 API 调试
删除群成员推送备注名 /group/remarks/del.json 100 次/每秒，可调频 暂不支持 API 调试
查询群成员推送备注名 /group/remarks/get.json 100 次/每秒，可调频 暂不支持 API 调试


推送与通知管理
功能/文档页面 API URL 频率限制 北极星 API 调试地址 Apifox 调试地址
设置指定会话免打扰 /conversation/notification/set.json 100 次/每秒，可调频 API 调试 API 调试
查询指定会话免打扰 /conversation/notification/get.json 100 次/每秒 API 调试 API 调试
设置指定会话类型免打扰 /conversation/type/notification/set.json 100 次/每秒 暂不支持 API 调试
查询指定会话类型免打扰 /conversation/type/notification/get.json 100 次/每秒 暂不支持 API 调试
设置用户免打扰时段 /user/blockPushPeriod/set.json 100 次/每秒，可调频 暂不支持 API 调试
删除用户免打扰时段 /user/blockPushPeriod/delete.json 100 次/每秒，可调频 暂不支持 API 调试
查询用户免打扰时段 /user/blockPushPeriod/get.json 100 次/每秒，可调频 暂不支持 API 调试
推送 Plus /push/custom.json 20 次/每小时，100 次/每自然日 API 调试 API 调试

最后于 2026 年 1 月 9 日 更新 -->
