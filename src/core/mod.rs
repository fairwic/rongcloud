//! 核心模块
//!
//! 包含客户端、配置、错误类型

mod client;
mod config;
mod error;

pub use client::{RongCloud, SDK_USER_AGENT, SDK_VERSION};
pub use config::RongCloudConfig;
pub use error::RongCloudError;
