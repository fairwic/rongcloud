//! 融云客户端配置
//!
//! 支持多区域、自动故障切换、超时配置

use crate::types::Region;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::time::Duration;

/// 融云客户端配置
#[derive(Debug)]
pub struct RongCloudConfig {
    /// App Key
    pub app_key: String,
    /// App Secret
    pub app_secret: String,
    /// 当前使用的区域
    pub region: Region,
    /// 自定义 API URL（如果设置了则覆盖区域 URL）
    custom_api_url: Option<String>,
    /// HTTP 连接超时时间
    pub connect_timeout: Duration,
    /// HTTP 读取超时时间
    pub read_timeout: Duration,
    /// 故障切换阈值：连续失败多少次后切换到备用域名
    pub error_switching_threshold: u32,
    /// 当前错误计数器（线程安全）
    error_counter: AtomicU32,
    /// 当前域名索引（0=主域名, 1=备用域名）
    domain_index: AtomicUsize,
}

impl Clone for RongCloudConfig {
    fn clone(&self) -> Self {
        Self {
            app_key: self.app_key.clone(),
            app_secret: self.app_secret.clone(),
            region: self.region,
            custom_api_url: self.custom_api_url.clone(),
            connect_timeout: self.connect_timeout,
            read_timeout: self.read_timeout,
            error_switching_threshold: self.error_switching_threshold,
            error_counter: AtomicU32::new(self.error_counter.load(Ordering::Relaxed)),
            domain_index: AtomicUsize::new(self.domain_index.load(Ordering::Relaxed)),
        }
    }
}

/// 默认连接超时：30秒
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
/// 默认读取超时：30秒
const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(30);
/// 默认故障切换阈值：1次失败后切换
const DEFAULT_ERROR_THRESHOLD: u32 = 1;

impl RongCloudConfig {
    /// 创建新的配置（使用默认区域：北京）
    ///
    /// # Arguments
    /// * `app_key` - 融云 App Key
    /// * `app_secret` - 融云 App Secret
    pub fn new(app_key: impl Into<String>, app_secret: impl Into<String>) -> Self {
        Self {
            app_key: app_key.into(),
            app_secret: app_secret.into(),
            region: Region::default(),
            custom_api_url: None,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            read_timeout: DEFAULT_READ_TIMEOUT,
            error_switching_threshold: DEFAULT_ERROR_THRESHOLD,
            error_counter: AtomicU32::new(0),
            domain_index: AtomicUsize::new(0),
        }
    }

    /// 设置 API 区域
    pub fn with_region(mut self, region: Region) -> Self {
        self.region = region;
        self
    }

    /// 设置自定义 API URL（覆盖区域 URL）
    ///
    /// 主要用于测试或私有部署场景
    pub fn with_api_url(mut self, api_url: impl Into<String>) -> Self {
        self.custom_api_url = Some(api_url.into());
        self
    }

    /// 设置 HTTP 连接超时时间
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// 设置 HTTP 读取超时时间
    pub fn with_read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = timeout;
        self
    }

    /// 设置故障切换阈值
    ///
    /// 连续失败达到此阈值后，会自动切换到备用域名
    pub fn with_error_threshold(mut self, threshold: u32) -> Self {
        self.error_switching_threshold = threshold;
        self
    }

    /// 获取当前 API URL
    ///
    /// 优先使用自定义 URL，否则根据区域和故障切换状态返回对应 URL
    pub fn api_url(&self) -> &str {
        if let Some(ref custom_url) = self.custom_api_url {
            return custom_url;
        }

        let urls = self.region.urls();
        let index = self.domain_index.load(Ordering::Relaxed) % urls.len();
        urls[index]
    }

    /// 记录一次请求成功
    ///
    /// 重置错误计数器
    pub fn record_success(&self) {
        self.error_counter.store(0, Ordering::Relaxed);
    }

    /// 记录一次请求失败
    ///
    /// 如果连续失败次数达到阈值，切换到备用域名
    pub fn record_error(&self) {
        let count = self.error_counter.fetch_add(1, Ordering::Relaxed) + 1;

        if count >= self.error_switching_threshold {
            // 切换域名
            self.domain_index.fetch_add(1, Ordering::Relaxed);
            // 重置计数器
            self.error_counter.store(0, Ordering::Relaxed);

            log::warn!(
                "Switching to backup domain due to {} consecutive errors",
                count
            );
        }
    }

    /// 获取当前使用的域名索引（用于调试）
    pub fn current_domain_index(&self) -> usize {
        self.domain_index.load(Ordering::Relaxed) % 2
    }

    /// 重置故障切换状态
    pub fn reset_failover(&self) {
        self.error_counter.store(0, Ordering::Relaxed);
        self.domain_index.store(0, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = RongCloudConfig::new("app_key", "app_secret");
        assert_eq!(config.app_key, "app_key");
        assert_eq!(config.app_secret, "app_secret");
        assert_eq!(config.region, Region::Beijing);
        assert!(config.api_url().contains("rong-api.com"));
    }

    #[test]
    fn test_config_with_region() {
        let config = RongCloudConfig::new("key", "secret").with_region(Region::Singapore);
        assert_eq!(config.region, Region::Singapore);
        assert!(config.api_url().contains("sg-light-api.com"));
    }

    #[test]
    fn test_config_with_custom_url() {
        let config = RongCloudConfig::new("key", "secret").with_api_url("http://localhost:8080");
        assert_eq!(config.api_url(), "http://localhost:8080");
    }

    #[test]
    fn test_failover_switching() {
        let config = RongCloudConfig::new("key", "secret").with_error_threshold(2);

        // 初始使用主域名
        assert_eq!(config.current_domain_index(), 0);
        assert!(config.api_url().contains("api.rong-api.com"));

        // 第一次失败，不切换
        config.record_error();
        assert_eq!(config.current_domain_index(), 0);

        // 第二次失败，切换到备用
        config.record_error();
        assert_eq!(config.current_domain_index(), 1);
        assert!(config.api_url().contains("api-b.rong-api.com"));

        // 成功后重置计数器，但不切换回主域名
        config.record_success();
        assert_eq!(config.current_domain_index(), 1);

        // 再次连续失败两次，切换回主域名
        config.record_error();
        config.record_error();
        assert_eq!(config.current_domain_index(), 0);
    }

    #[test]
    fn test_reset_failover() {
        let config = RongCloudConfig::new("key", "secret").with_error_threshold(1);

        config.record_error();
        assert_eq!(config.current_domain_index(), 1);

        config.reset_failover();
        assert_eq!(config.current_domain_index(), 0);
    }
}
