//! 融云 Rust SDK
//!
//! 提供融云 IM 服务端 API 的 Rust 实现
//!
//! # 快速开始
//!
//! ```rust,no_run
//! use rongcloud::{RongCloud, RongCloudConfig, Region};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = RongCloudConfig::new("app_key", "app_secret")
//!         .with_region(Region::Beijing);
//!     let client = RongCloud::new(config);
//!
//!     // 注册用户
//!     let result = client.user_register("user1", "张三", "http://avatar.url").await;
//! }
//! ```

// 核心模块
pub mod core;
// 类型定义
pub mod types;
// 消息类型
pub mod messages;
// API 模块
pub mod api;
// 工具函数
mod util;

// 重新导出常用类型
pub use core::{RongCloud, RongCloudConfig, RongCloudError};
pub use types::{ConversationType, RcResponse, Region};
