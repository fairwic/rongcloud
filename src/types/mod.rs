//! 类型定义模块
//!
//! 包含会话类型、区域、错误码、响应模型

mod content_type;
mod conversation;
pub mod error_codes;
mod region;
mod response;

pub use content_type::ContentType;
pub use conversation::ConversationType;
pub use region::Region;
pub use response::RcResponse;
