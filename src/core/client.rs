//! 融云客户端核心实现
//!
//! 负责 HTTP 请求发送、签名生成、自动故障切换

use super::config::RongCloudConfig;
use super::error::RongCloudError;
use crate::util::{current_timestamp, generate_nonce, generate_signature};
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// SDK 版本
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");
/// SDK User-Agent
pub const SDK_USER_AGENT: &str = concat!("rc-rust-sdk/", env!("CARGO_PKG_VERSION"));

/// 融云客户端
pub struct RongCloud {
    /// 客户端配置
    pub config: RongCloudConfig,
    /// HTTP 客户端
    http_client: Client,
}

impl RongCloud {
    /// 创建新的融云客户端
    ///
    /// # Arguments
    /// * `config` - 客户端配置
    pub fn new(config: RongCloudConfig) -> Self {
        let http_client = Client::builder()
            .connect_timeout(config.connect_timeout)
            .timeout(config.read_timeout)
            .build()
            .unwrap_or_default();

        Self {
            config,
            http_client,
        }
    }

    /// 发送 POST 请求
    ///
    /// 内部方法，包含签名生成和故障切换逻辑
    pub(crate) async fn post<R, B>(
        &self,
        path: &str,
        body: &B,
        content_type: &str,
    ) -> Result<R, RongCloudError>
    where
        R: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.config.api_url(), path);
        let nonce = generate_nonce();
        let timestamp = current_timestamp();
        let signature = generate_signature(&self.config.app_secret, &nonce, &timestamp);

        let request_builder = self
            .http_client
            .post(&url)
            .header("App-Key", &self.config.app_key)
            .header("Nonce", nonce)
            .header("Timestamp", timestamp)
            .header("Signature", signature)
            .header("Content-Type", content_type)
            .header("User-Agent", SDK_USER_AGENT);

        let response = if content_type.contains("json") {
            request_builder.json(body).send().await
        } else {
            request_builder.form(body).send().await
        };

        // 处理请求错误（网络错误等）
        let response = match response {
            Ok(resp) => resp,
            Err(e) => {
                // 记录失败，可能触发域名切换
                self.config.record_error();
                return Err(RongCloudError::Http(e));
            }
        };

        let status = response.status();
        let text = response.text().await?;

        // HTTP 状态码检查
        if !status.is_success() {
            // 服务端错误可能需要切换域名
            if status.is_server_error() {
                self.config.record_error();
            }
            return Err(RongCloudError::Api {
                code: status.as_u16() as i32,
                msg: text,
            });
        }

        // 记录成功，重置错误计数
        self.config.record_success();

        // 解析响应
        let result: R = serde_json::from_str(&text).map_err(|e| {
            log::error!("Failed to parse response: {}", text);
            RongCloudError::Serialization(e)
        })?;

        Ok(result)
    }

    /// 获取当前使用的 API URL
    pub fn current_api_url(&self) -> &str {
        self.config.api_url()
    }

    /// 重置故障切换状态
    ///
    /// 将域名切换回主域名，并重置错误计数
    pub fn reset_failover(&self) {
        self.config.reset_failover();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Region;

    #[test]
    fn test_sdk_version() {
        assert!(!SDK_VERSION.is_empty());
        assert!(SDK_USER_AGENT.starts_with("rc-rust-sdk/"));
    }

    #[test]
    fn test_client_creation() {
        let config = RongCloudConfig::new("app_key", "app_secret");
        let client = RongCloud::new(config);

        assert_eq!(client.config.app_key, "app_key");
    }

    #[test]
    fn test_client_with_region() {
        let config = RongCloudConfig::new("key", "secret").with_region(Region::Singapore);
        let client = RongCloud::new(config);

        assert!(client.current_api_url().contains("sg-light-api"));
    }
}
