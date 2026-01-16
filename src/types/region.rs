//! 融云 API 区域配置
//!
//! 支持多数据中心切换和自动故障转移

/// 融云 API 区域枚举
///
/// 每个区域包含主域名和备用域名，支持自动故障切换
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Region {
    /// 北京数据中心（默认）
    #[default]
    Beijing,
    /// 新加坡数据中心
    Singapore,
    /// 新加坡备用数据中心
    SingaporeBackup,
    /// 北美数据中心
    NorthAmerica,
    /// 沙特数据中心
    SaudiArabia,
}

impl Region {
    /// 获取主域名 URL
    pub fn primary_url(&self) -> &'static str {
        match self {
            Self::Beijing => "https://api.rong-api.com",
            Self::Singapore => "https://api.sg-light-api.com",
            Self::SingaporeBackup => "https://api.sg-b-light-api.com",
            Self::NorthAmerica => "https://api.us-light-api.com",
            Self::SaudiArabia => "https://api.sau-light-api.com",
        }
    }

    /// 获取备用域名 URL
    pub fn backup_url(&self) -> &'static str {
        match self {
            Self::Beijing => "https://api-b.rong-api.com",
            Self::Singapore => "https://api-b.sg-light-api.com",
            Self::SingaporeBackup => "https://api-b.sg-b-light-api.com",
            Self::NorthAmerica => "https://api-b.us-light-api.com",
            Self::SaudiArabia => "https://api-b.sau-light-api.com",
        }
    }

    /// 获取所有可用的服务器 URL（主域名在前）
    pub fn urls(&self) -> [&'static str; 2] {
        [self.primary_url(), self.backup_url()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_urls() {
        let beijing = Region::Beijing;
        assert_eq!(beijing.primary_url(), "https://api.rong-api.com");
        assert_eq!(beijing.backup_url(), "https://api-b.rong-api.com");
    }

    #[test]
    fn test_region_default() {
        let default_region = Region::default();
        assert_eq!(default_region, Region::Beijing);
    }

    #[test]
    fn test_all_regions_have_urls() {
        let regions = [
            Region::Beijing,
            Region::Singapore,
            Region::SingaporeBackup,
            Region::NorthAmerica,
            Region::SaudiArabia,
        ];

        for region in regions {
            assert!(!region.primary_url().is_empty());
            assert!(!region.backup_url().is_empty());
            assert!(region.primary_url().starts_with("https://"));
            assert!(region.backup_url().starts_with("https://"));
        }
    }
}
