//! 融云 API 错误码常量
//!
//! 包含所有融云服务端返回的错误码定义

/// 成功
pub const SUCCESS: i32 = 200;

/// 超出测试用户限制，请申请扩大配额
pub const TEST_USERS_EXCEEDED: i32 = 20008;

/// 签名错误，请确认 App Key 和 App Secret
pub const SIGN_ERROR: i32 = 20000;

/// 超出频率限制
pub const FREQUENCY_EXCEEDED: i32 = 20001;

/// 参数校验失败，长度/数量超限
pub const PARAM_VALIDATION_FAILED: i32 = 20002;

/// userId 超出最大长度（64字节）
pub const USER_ID_TOO_LONG: i32 = 20003;

/// 封禁时长无效，minute 须 >= 1 且 <= 43200
pub const BAN_TIMEOUT_INVALID: i32 = 20004;

/// 参数类型错误
pub const PARAM_TYPE_ERROR: i32 = 20006;

/// 地址格式无效
pub const ADDRESS_INVALID: i32 = 20007;

/// 参数缺失
pub const PARAM_MISSING: i32 = 1002;

/// 用户不存在
pub const USER_NOT_FOUND: i32 = 1001;

/// 服务内部错误
pub const INTERNAL_ERROR: i32 = 500;

/// 服务不可用
pub const SERVICE_UNAVAILABLE: i32 = 503;

/// 未授权
pub const UNAUTHORIZED: i32 = 401;

/// 请求过于频繁
pub const TOO_MANY_REQUESTS: i32 = 429;

/// 群组不存在
pub const GROUP_NOT_FOUND: i32 = 1003;

/// 用户已在群组中
pub const USER_ALREADY_IN_GROUP: i32 = 22406;

/// 用户不在群组中
pub const USER_NOT_IN_GROUP: i32 = 22408;

/// 聊天室不存在
pub const CHATROOM_NOT_FOUND: i32 = 1004;

/// 消息发送失败
pub const MESSAGE_SEND_FAILED: i32 = 1005;

/// 判断错误码是否表示成功
#[inline]
pub fn is_success(code: i32) -> bool {
    code == SUCCESS
}

/// 判断错误码是否表示客户端错误（参数问题）
#[inline]
pub fn is_client_error(code: i32) -> bool {
    matches!(
        code,
        PARAM_MISSING
            | PARAM_TYPE_ERROR
            | PARAM_VALIDATION_FAILED
            | USER_ID_TOO_LONG
            | BAN_TIMEOUT_INVALID
            | ADDRESS_INVALID
    )
}

/// 判断错误码是否表示服务端错误（需要重试）
#[inline]
pub fn is_server_error(code: i32) -> bool {
    matches!(code, INTERNAL_ERROR | SERVICE_UNAVAILABLE)
}

/// 判断错误码是否表示需要切换域名重试
#[inline]
pub fn is_retryable(code: i32) -> bool {
    is_server_error(code) || code == TOO_MANY_REQUESTS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_success() {
        assert!(is_success(SUCCESS));
        assert!(!is_success(PARAM_MISSING));
    }

    #[test]
    fn test_is_client_error() {
        assert!(is_client_error(PARAM_MISSING));
        assert!(is_client_error(USER_ID_TOO_LONG));
        assert!(!is_client_error(SUCCESS));
        assert!(!is_client_error(INTERNAL_ERROR));
    }

    #[test]
    fn test_is_retryable() {
        assert!(is_retryable(INTERNAL_ERROR));
        assert!(is_retryable(SERVICE_UNAVAILABLE));
        assert!(is_retryable(TOO_MANY_REQUESTS));
        assert!(!is_retryable(PARAM_MISSING));
    }
}
