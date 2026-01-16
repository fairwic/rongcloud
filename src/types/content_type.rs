/// HTTP 请求内容类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    /// 表单编码格式
    FormUrlEncoded,
    /// JSON 格式
    Json,
}

impl ContentType {
    /// 获取 HTTP Content-Type 头值
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::FormUrlEncoded => "application/x-www-form-urlencoded",
            ContentType::Json => "application/json",
        }
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
