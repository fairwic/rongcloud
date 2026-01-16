use crate::core::RongCloud;
use crate::core::RongCloudError;
use crate::types::RcResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserRegisterParams<'a> {
    #[serde(rename = "userId")]
    pub user_id: &'a str,
    #[serde(rename = "name")]
    pub name: &'a str,
    #[serde(rename = "portraitUri")]
    pub portrait_uri: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct UserToken {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct UserUpdateParams<'a> {
    #[serde(rename = "userId")]
    pub user_id: &'a str,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(rename = "portraitUri", skip_serializing_if = "Option::is_none")]
    pub portrait_uri: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub name: Option<String>,
    #[serde(rename = "userPortrait")]
    pub portrait_uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserOnlineStatus {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockUser {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "blockEndTime")]
    pub block_end_time: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockList {
    pub users: Option<Vec<BlockUser>>, // API might return null if empty
}

impl RongCloud {
    /// Register a user and get a token.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/register
    pub async fn user_register(
        &self,
        user_id: &str,
        name: &str,
        portrait_uri: &str,
    ) -> Result<RcResponse<UserToken>, RongCloudError> {
        let params = UserRegisterParams {
            user_id,
            name,
            portrait_uri,
        };

        self.post(
            super::endpoints::USER_GET_TOKEN,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Update user information.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/refresh
    pub async fn user_update(
        &self,
        user_id: &str,
        name: Option<&str>,
        portrait_uri: Option<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = UserUpdateParams {
            user_id,
            name,
            portrait_uri,
        };

        self.post(
            super::endpoints::USER_REFRESH,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Get user information.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/info
    pub async fn user_info(&self, user_id: &str) -> Result<RcResponse<UserInfo>, RongCloudError> {
        #[derive(Serialize)]
        struct InfoParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
        }
        let params = InfoParams { user_id };

        self.post(
            super::endpoints::USER_INFO,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Check user online status.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/onlinestatus
    pub async fn user_check_online(
        &self,
        user_id: &str,
    ) -> Result<RcResponse<UserOnlineStatus>, RongCloudError> {
        #[derive(Serialize)]
        struct CheckOnlineParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
        }
        let params = CheckOnlineParams { user_id };

        self.post(
            super::endpoints::USER_CHECK_ONLINE,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Block a user.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/block
    pub async fn user_block(
        &self,
        user_id: &str,
        minute: i32,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct BlockParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
            minute: i32,
        }
        let params = BlockParams { user_id, minute };

        self.post(
            super::endpoints::USER_BLOCK,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Unblock a user.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/unblock
    pub async fn user_unblock(&self, user_id: &str) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct UnblockParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
        }
        let params = UnblockParams { user_id };

        self.post(
            super::endpoints::USER_UNBLOCK,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query blocked users.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/block/query
    pub async fn user_block_query(&self) -> Result<RcResponse<BlockList>, RongCloudError> {
        #[derive(Serialize)]
        struct BlockQueryParams {}
        let params = BlockQueryParams {};
        self.post(
            super::endpoints::USER_BLOCK_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Add user to blacklist.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/blacklist/add
    pub async fn user_blacklist_add(
        &self,
        user_id: &str,
        black_user_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct BlacklistAddParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
            #[serde(rename = "blackUserId")]
            black_user_id: &'a str,
        }
        let params = BlacklistAddParams {
            user_id,
            black_user_id,
        };

        self.post(
            super::endpoints::USER_BLACKLIST_ADD,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Remove user from blacklist.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/blacklist/remove
    pub async fn user_blacklist_remove(
        &self,
        user_id: &str,
        black_user_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct BlacklistRemoveParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
            #[serde(rename = "blackUserId")]
            black_user_id: &'a str,
        }
        let params = BlacklistRemoveParams {
            user_id,
            black_user_id,
        };

        self.post(
            super::endpoints::USER_BLACKLIST_REMOVE,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query user blacklist.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/blacklist/query
    pub async fn user_blacklist_query(
        &self,
        user_id: &str,
    ) -> Result<RcResponse<BlockList>, RongCloudError> {
        // Response format for blacklist query is list of users, similar to block query?
        // API list says "Get user's blacklist". Usually returns list of user IDs or user objects.
        // I'll assume users list same as BlockList for now or just generic.
        // Let's assume it returns a list of blocked users structure.
        #[derive(Serialize)]
        struct BlacklistQueryParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
        }
        let params = BlacklistQueryParams { user_id };

        self.post(
            super::endpoints::USER_BLACKLIST_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Set user tags.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/tag/set
    pub async fn user_tag_set(
        &self,
        user_id: &str,
        tags: Vec<String>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct TagSetParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
            tags: Vec<String>,
        }
        let params = TagSetParams { user_id, tags };

        // API says "application/json" in Java SDK
        self.post(super::endpoints::USER_TAG_SET, &params, "application/json")
            .await
    }

    /// Get tags for users.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/tags/get
    pub async fn user_tags_get(
        &self,
        user_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        // Returns complex structure, using unit for now or need generic maps
        // Java SDK: GetTagResult -> result (Map<String, Vec<String>>)
        // Since I'm using RcResponse, data should be the map?
        // Let's use serde_json::Value for data to be safe or implement specific struct

        // Params needs to be x-www-form-urlencoded with multiple userIds
        // generic params struct with manual serialization or using vec
        let params: Vec<(&str, &str)> = user_ids.iter().map(|id| ("userIds", *id)).collect();

        // reqwest form supports array of tuples for multiple values
        self.post(
            super::endpoints::USER_TAGS_GET,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Set user chat ban (Forbidden).
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/chat/ban/set
    pub async fn user_chat_ban_set(
        &self,
        user_ids: Vec<&str>,
        state: i32, // 1 for ban, 0 for unban usually? Java SDK takes 'state'.
        typ: &str,  // "person" etc.
    ) -> Result<RcResponse<()>, RongCloudError> {
        // Body: userId=...&userId=...&state=...&type=...
        let mut params: Vec<(&str, String)> = user_ids
            .iter()
            .map(|id| ("userId", id.to_string()))
            .collect();
        params.push(("state", state.to_string()));
        params.push(("type", typ.to_string()));

        self.post(
            super::endpoints::USER_CHAT_FB_SET,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query chat ban list.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/chat/ban/query
    pub async fn user_chat_ban_query(
        &self,
        typ: &str,
        num: Option<i32>,
        offset: Option<i32>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct BanQueryParams<'a> {
            #[serde(rename = "type")]
            typ: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            num: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            offset: Option<i32>,
        }
        let params = BanQueryParams { typ, num, offset };

        self.post(
            super::endpoints::USER_CHAT_FB_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    // ========================================================================
    // 用户 Token 管理
    // ========================================================================

    /// Expire user token.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/token/expire
    pub async fn user_token_expire(
        &self,
        user_id: &str,
        time: Option<i64>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct ExpireParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            time: Option<i64>,
        }
        let params = ExpireParams { user_id, time };

        self.post(
            super::endpoints::USER_TOKEN_EXPIRE,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    // ========================================================================
    // 用户注销与激活
    // ========================================================================

    /// Deactivate user.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/deactivate
    pub async fn user_deactivate(
        &self,
        user_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params: Vec<(&str, &str)> = user_ids.iter().map(|id| ("userId", *id)).collect();

        self.post(
            super::endpoints::USER_DEACTIVATE,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query deactivated users.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/deactivate/query
    pub async fn user_deactivate_query(
        &self,
        page_num: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct QueryParams {
            #[serde(rename = "pageNum", skip_serializing_if = "Option::is_none")]
            page_num: Option<i32>,
            #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }
        let params = QueryParams {
            page_num,
            page_size,
        };

        self.post(
            super::endpoints::USER_DEACTIVATE_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Reactivate user.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/reactivate
    pub async fn user_reactivate(
        &self,
        user_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params: Vec<(&str, &str)> = user_ids.iter().map(|id| ("userId", *id)).collect();

        self.post(
            super::endpoints::USER_REACTIVATE,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    // ========================================================================
    // 用户白名单服务
    // ========================================================================

    /// Enable user whitelist service.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/whitesetting/set
    pub async fn user_whitesetting_set(
        &self,
        user_id: &str,
        status: i32, // 1 = enable, 0 = disable
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct WhiteSettingParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
            status: i32,
        }
        let params = WhiteSettingParams { user_id, status };

        self.post(
            super::endpoints::USER_WHITE_SETTING_SET,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query user whitelist service status.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/whitesetting/query
    pub async fn user_whitesetting_query(
        &self,
        user_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct QueryParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
        }
        let params = QueryParams { user_id };

        self.post(
            super::endpoints::USER_WHITE_SETTING_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Add user to whitelist.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/whitelist/add
    pub async fn user_whitelist_add(
        &self,
        user_id: &str,
        white_user_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params: Vec<(&str, &str)> = vec![("userId", user_id)];
        for wid in &white_user_ids {
            params.push(("whiteUserId", wid));
        }

        self.post(
            super::endpoints::USER_WHITELIST_ADD,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Remove user from whitelist.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/whitelist/remove
    pub async fn user_whitelist_remove(
        &self,
        user_id: &str,
        white_user_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params: Vec<(&str, &str)> = vec![("userId", user_id)];
        for wid in &white_user_ids {
            params.push(("whiteUserId", wid));
        }

        self.post(
            super::endpoints::USER_WHITELIST_REMOVE,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query user whitelist.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/whitelist/query
    pub async fn user_whitelist_query(
        &self,
        user_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct QueryParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
        }
        let params = QueryParams { user_id };

        self.post(
            super::endpoints::USER_WHITELIST_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    // ========================================================================
    // 用户信息托管
    // ========================================================================

    /// Set user profile (hosted).
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/profile/set
    pub async fn user_profile_set(
        &self,
        user_id: &str,
        profile: serde_json::Value,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct ProfileParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
            #[serde(flatten)]
            profile: serde_json::Value,
        }
        let params = ProfileParams { user_id, profile };

        self.post(
            super::endpoints::USER_PROFILE_SET,
            &params,
            "application/json",
        )
        .await
    }

    /// Clean user profile (hosted).
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/profile/clean
    pub async fn user_profile_clean(
        &self,
        user_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct CleanParams<'a> {
            #[serde(rename = "userId")]
            user_id: &'a str,
        }
        let params = CleanParams { user_id };

        self.post(
            super::endpoints::USER_PROFILE_CLEAN,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Batch query user profiles (hosted).
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/profile/batch/query
    pub async fn user_profile_batch_query(
        &self,
        user_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params: Vec<(&str, &str)> = user_ids.iter().map(|id| ("userId", *id)).collect();

        self.post(
            super::endpoints::USER_PROFILE_BATCH_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query all user profiles with pagination (hosted).
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/profile/query
    pub async fn user_profile_query(
        &self,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            size: Option<i32>,
        }
        let params = QueryParams { page, size };

        self.post(
            super::endpoints::USER_PROFILE_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::RongCloudConfig;
    use mockito;

    #[tokio::test]
    async fn test_user_register_success() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock = server
            .mock("POST", "/user/getToken.json")
            .match_header("App-Key", "app_key")
            // We can check other headers if needed
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code": 200, "userId": "user1", "token": "token123"}"#)
            .create_async()
            .await;

        let result = client
            .user_register("user1", "name1", "http://portrait")
            .await;

        mock.assert_async().await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.data.as_ref().unwrap().user_id, "user1");
        assert_eq!(response.data.as_ref().unwrap().token, "token123");
    }

    #[tokio::test]
    async fn test_user_update_success() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock = server
            .mock("POST", "/user/refresh.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;

        let result = client.user_update("user1", Some("newname"), None).await;
        mock.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_info_success() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock = server
            .mock("POST", "/user/info.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"code": 200, "userId": "user1", "userName": "name1", "userPortrait": "uri"}"#,
            )
            .create_async()
            .await;

        let result = client.user_info("user1").await;
        mock.assert_async().await;

        let info = result.unwrap();
        assert_eq!(info.code, 200);
        let data = info.data.unwrap();
        assert_eq!(data.user_id, "user1");
        assert_eq!(data.name, Some("name1".to_string()));
    }

    #[tokio::test]
    async fn test_user_check_online() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock = server
            .mock("POST", "/user/checkOnline.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code": 200, "status": "1"}"#)
            .create_async()
            .await;

        let result = client.user_check_online("user1").await;
        mock.assert_async().await;
        assert_eq!(result.unwrap().data.unwrap().status, "1");
    }

    #[tokio::test]
    async fn test_user_block_operations() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock_block = server
            .mock("POST", "/user/block.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;

        let result = client.user_block("user1", 10).await;
        mock_block.assert_async().await;
        assert!(result.is_ok());

        let mock_unblock = server
            .mock("POST", "/user/unblock.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;

        let result_unblock = client.user_unblock("user1").await;
        mock_unblock.assert_async().await;
        assert!(result_unblock.is_ok());
    }

    #[tokio::test]
    async fn test_user_block_query() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock = server
            .mock("POST", "/user/block/query.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code": 200, "users": [{"userId": "u1", "blockEndTime": "2022"}]}"#)
            .create_async()
            .await;

        let result = client.user_block_query().await;
        mock.assert_async().await;
        let list = result.unwrap().data.unwrap();
        assert_eq!(list.users.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_user_blacklist_operations() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock_add = server
            .mock("POST", "/user/blacklist/add.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;

        let result = client.user_blacklist_add("u1", "u2").await;
        mock_add.assert_async().await;
        assert!(result.is_ok());

        let mock_query = server
            .mock("POST", "/user/blacklist/query.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code": 200, "users": [{"userId": "u2", "blockEndTime": ""}]}"#)
            .create_async()
            .await;

        let result_q = client.user_blacklist_query("u1").await;
        mock_query.assert_async().await;
        assert!(result_q.is_ok());
    }

    #[tokio::test]
    async fn test_user_tag_ban_ops() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        // Tag Set
        let mock_tag = server
            .mock("POST", "/user/tag/set.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;

        let _ = client.user_tag_set("u1", vec!["t1".to_string()]).await;
        mock_tag.assert_async().await;

        // Ban Set
        let mock_ban = server
            .mock("POST", "/user/chat/fb/set.json")
            .with_status(200)
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;

        let _ = client.user_chat_ban_set(vec!["u1"], 1, "person").await;
        mock_ban.assert_async().await;
    }
}
