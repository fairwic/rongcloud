# Rongcloud Rust SDK

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2024-blue.svg)](https://www.rust-lang.org/)

融云（RongCloud）IM 服务端 API 的 Rust 实现。

## ✨ 特性

- 🦀 **纯 Rust 实现** - 高性能、内存安全的异步 API 客户端
- 🌍 **多地域支持** - 支持北京、新加坡、北美等多个数据中心节点
- 📦 **丰富的消息类型** - 文本、图片、语音、文件、通知等多种内置消息类型
- 🔐 **完整的用户管理** - 注册、封禁、黑名单、白名单等完整用户功能
- 💬 **多会话类型** - 单聊、群聊、聊天室、系统通知全覆盖
- 🚀 **异步优先** - 基于 Tokio 的全异步设计

## 📦 安装

将以下内容添加到你的 `Cargo.toml` 文件中：

```toml
[dependencies]
rongcloud = { git = "https://github.com/your-username/rongcloud-rust-sdk" }
```

## 🚀 快速开始

```rust
use rongcloud::{RongCloud, RongCloudConfig, Region};

#[tokio::main]
async fn main() {
    // 创建配置
    let config = RongCloudConfig::new("your_app_key", "your_app_secret")
        .with_region(Region::Beijing);

    // 创建客户端
    let client = RongCloud::new(config);

    // 注册用户并获取 Token
    let result = client.user_register("user_001", "张三", "https://example.com/avatar.jpg").await;

    match result {
        Ok(response) => {
            println!("用户 Token: {}", response.data.token);
        }
        Err(e) => {
            eprintln!("注册失败: {:?}", e);
        }
    }
}
```

## 📚 功能模块

### 用户管理

| 功能         | 方法                | 描述                 |
| ------------ | ------------------- | -------------------- |
| 注册用户     | `user_register`     | 注册用户并获取 Token |
| 更新用户信息 | `user_update`       | 修改用户名称和头像   |
| 获取用户信息 | `user_info`         | 查询用户详细信息     |
| 封禁用户     | `user_block`        | 封禁指定用户         |
| 解除封禁     | `user_unblock`      | 解除用户封禁         |
| 查询封禁列表 | `user_block_query`  | 获取已封禁的用户列表 |
| 检查在线状态 | `user_check_online` | 检查用户是否在线     |

### 黑白名单管理

| 功能       | 方法                    | 描述               |
| ---------- | ----------------------- | ------------------ |
| 添加黑名单 | `user_blacklist_add`    | 将用户添加到黑名单 |
| 移除黑名单 | `user_blacklist_remove` | 从黑名单中移除用户 |
| 查询黑名单 | `user_blacklist_query`  | 获取黑名单列表     |
| 添加白名单 | `user_whitelist_add`    | 将用户添加到白名单 |
| 移除白名单 | `user_whitelist_remove` | 从白名单中移除用户 |
| 查询白名单 | `user_whitelist_query`  | 获取白名单列表     |

### 消息发送

```rust
use rongcloud::api::message::{PrivateMessage, GroupMessage, SystemMessage};
use rongcloud::messages::TextMessage;

// 发送单聊消息
let text = TextMessage::new("Hello World!", "附加内容");
let msg = PrivateMessage::new("sender_id", text.object_name(), text.to_json())
    .to_user("receiver_id")
    .push_content("你收到了一条新消息");

client.send_private_message(&msg).await?;

// 发送群聊消息
let group_msg = GroupMessage::new("sender_id", "RC:TxtMsg", content)
    .to_group("group_id")
    .is_mentioned(1);  // @所有人

client.send_group_message(&group_msg).await?;

// 发送系统通知
let sys_msg = SystemMessage::new("system", "RC:TxtMsg", content)
    .to_user("user_id");

client.send_system_message(&sys_msg).await?;
```

### 群组管理

| 功能       | 方法                      | 描述             |
| ---------- | ------------------------- | ---------------- |
| 创建群组   | `group_create`            | 创建新群组       |
| 解散群组   | `group_dismiss`           | 解散指定群组     |
| 加入群组   | `group_join`              | 用户加入群组     |
| 退出群组   | `group_quit`              | 用户退出群组     |
| 查询群成员 | `group_user_query`        | 获取群组成员列表 |
| 禁言群成员 | `group_user_gag_add`      | 禁言指定群成员   |
| 取消禁言   | `group_user_gag_rollback` | 取消群成员禁言   |
| 全体禁言   | `group_ban_add`           | 设置群组全体禁言 |

### 聊天室管理

| 功能       | 方法                     | 描述           |
| ---------- | ------------------------ | -------------- |
| 创建聊天室 | `chatroom_create`        | 创建新聊天室   |
| 销毁聊天室 | `chatroom_destroy`       | 销毁指定聊天室 |
| 查询聊天室 | `chatroom_get`           | 获取聊天室信息 |
| 保活聊天室 | `chatroom_keepalive_add` | 设置聊天室保活 |

### 好友管理

| 功能         | 方法            | 描述                 |
| ------------ | --------------- | -------------------- |
| 添加好友     | `friend_add`    | 添加好友关系         |
| 删除好友     | `friend_delete` | 解除好友关系         |
| 获取好友列表 | `friend_get`    | 查询用户好友列表     |
| 检查好友关系 | `friend_check`  | 检查两用户是否为好友 |

### 敏感词管理

| 功能       | 方法                      | 描述           |
| ---------- | ------------------------- | -------------- |
| 添加敏感词 | `sensitiveword_add`       | 添加消息敏感词 |
| 批量添加   | `sensitiveword_batch_add` | 批量添加敏感词 |
| 删除敏感词 | `sensitiveword_delete`    | 移除敏感词     |
| 查询敏感词 | `sensitiveword_list`      | 获取敏感词列表 |

## 📱 消息类型

SDK 内置多种标准消息类型：

| 消息类型 | 类名                  | ObjectName   |
| -------- | --------------------- | ------------ |
| 文本消息 | `TextMessage`         | `RC:TxtMsg`  |
| 图片消息 | `ImageMessage`        | `RC:ImgMsg`  |
| 语音消息 | `VoiceMessage`        | `RC:VcMsg`   |
| 文件消息 | `FileMessage`         | `RC:FileMsg` |
| 通知消息 | `NotificationMessage` | `RC:NtfMsg`  |

### 自定义消息

```rust
use serde::Serialize;

#[derive(Serialize)]
struct CustomMessage {
    #[serde(rename = "type")]
    msg_type: String,
    data: String,
}

impl CustomMessage {
    fn object_name() -> &'static str {
        "App:CustomMsg"
    }
}
```

## 🌍 多地域配置

```rust
use rongcloud::{RongCloudConfig, Region};

// 北京节点（默认）
let config = RongCloudConfig::new("app_key", "app_secret")
    .with_region(Region::Beijing);

// 新加坡节点
let config = RongCloudConfig::new("app_key", "app_secret")
    .with_region(Region::Singapore);

// 北美节点
let config = RongCloudConfig::new("app_key", "app_secret")
    .with_region(Region::NorthAmerica);
```

## ⚠️ 错误处理

```rust
use rongcloud::RongCloudError;

match client.user_register("user_id", "name", "avatar_url").await {
    Ok(response) => {
        if response.code == 200 {
            println!("成功: {:?}", response.data);
        } else {
            println!("业务错误码: {}", response.code);
        }
    }
    Err(RongCloudError::RequestFailed(msg)) => {
        eprintln!("请求失败: {}", msg);
    }
    Err(RongCloudError::SerializationError(e)) => {
        eprintln!("序列化错误: {}", e);
    }
    Err(e) => {
        eprintln!("其他错误: {:?}", e);
    }
}
```

## 🔧 配置项

| 配置       | 方法                   | 说明                |
| ---------- | ---------------------- | ------------------- |
| 地域       | `with_region()`        | 设置数据中心节点    |
| 超时       | `with_timeout()`       | 设置请求超时时间    |
| 自定义域名 | `with_custom_domain()` | 使用自定义 API 域名 |

## 🛠️ 开发

```bash
# 运行测试
cargo test

# 检查代码
cargo clippy

# 格式化
cargo fmt
```

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🔗 相关链接

- [融云官方文档](https://doc.rongcloud.cn/imserver/server/v1/overview)
- [融云开发者平台](https://developer.rongcloud.cn/)

## 📊 API 覆盖率

本 SDK 覆盖了融云服务端 API 的主要功能，包括：

- ✅ 用户管理服务
- ✅ 用户黑/白名单服务
- ✅ 好友管理（信息托管）
- ✅ 消息管理
- ✅ 群组管理
- ✅ 群组禁言服务
- ✅ 聊天室管理
- ✅ 内容审核（敏感词）
- ✅ 推送服务

更多 API 正在持续开发中...
