use crate::core::RongCloud;
use crate::core::RongCloudError;
use crate::types::RcResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct FriendAddParams<'a> {
    #[serde(rename = "userId")]
    user_id: &'a str,
    #[serde(rename = "targetId")]
    target_id: &'a str,
}

#[derive(Debug, Serialize)]
struct FriendDeleteParams<'a> {
    #[serde(rename = "userId")]
    user_id: &'a str,
    #[serde(rename = "targetIds")]
    target_ids: String, // Comma separated
}

#[derive(Debug, Serialize)]
struct FriendCleanParams<'a> {
    #[serde(rename = "userId")]
    user_id: &'a str,
}

#[derive(Debug, Serialize)]
struct FriendProfileSetParams<'a> {
    #[serde(rename = "userId")]
    user_id: &'a str,
    #[serde(rename = "targetId")]
    target_id: &'a str,
    #[serde(rename = "remarkName", skip_serializing_if = "Option::is_none")]
    remark_name: Option<&'a str>,
    #[serde(rename = "friendExtProfile", skip_serializing_if = "Option::is_none")]
    ext_profile: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct FriendGetListParams<'a> {
    #[serde(rename = "userId")]
    user_id: &'a str,
    #[serde(rename = "pageToken", skip_serializing_if = "Option::is_none")]
    page_token: Option<&'a str>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i32>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    order: Option<i32>,
}

#[derive(Debug, Serialize)]
struct CheckParams<'a> {
    #[serde(rename = "userId")]
    user_id: &'a str,
    #[serde(rename = "targetIds")]
    target_ids: String,
}

impl RongCloud {
    /// Add a friend.
    pub async fn friend_add(
        &self,
        user_id: &str,
        target_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = FriendAddParams { user_id, target_id };
        self.post(
            "/friend/add.json",
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Delete friends.
    pub async fn friend_delete(
        &self,
        user_id: &str,
        target_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let target_ids_str = target_ids.join(",");
        let params = FriendDeleteParams {
            user_id,
            target_ids: target_ids_str,
        };
        self.post(
            "/friend/delete.json",
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Clean all friends.
    pub async fn friend_clean(&self, user_id: &str) -> Result<RcResponse<()>, RongCloudError> {
        let params = FriendCleanParams { user_id };
        self.post(
            "/friend/clean.json",
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Set friend profile.
    pub async fn friend_profile_set(
        &self,
        user_id: &str,
        target_id: &str,
        remark_name: Option<&str>,
        ext_profile: Option<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = FriendProfileSetParams {
            user_id,
            target_id,
            remark_name,
            ext_profile,
        };
        self.post(
            "/friend/profile/set.json",
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Get friend list.
    pub async fn friend_get_list(
        &self,
        user_id: &str,
        page_token: Option<&str>,
        size: Option<i32>,
        order: Option<i32>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = FriendGetListParams {
            user_id,
            page_token,
            size,
            order,
        };
        self.post(
            "/friend/get.json",
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Check friend relationship.
    pub async fn friend_check(
        &self,
        user_id: &str,
        target_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let target_ids_str = target_ids.join(",");
        let params = CheckParams {
            user_id,
            target_ids: target_ids_str,
        };
        self.post(
            "/friend/check.json",
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Set permission for adding friends.
    pub async fn friend_permission_set(
        &self,
        user_ids: Vec<&str>,
        permission_type: i32,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct PermSetParams {
            #[serde(rename = "userIds")]
            user_ids: String,
            #[serde(rename = "permissionType")]
            permission_type: i32,
        }
        let params = PermSetParams {
            user_ids: user_ids.join(","),
            permission_type,
        };
        self.post(
            "/friend/permission/set.json",
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Get permission for adding friends.
    pub async fn friend_permission_get(
        &self,
        user_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        #[derive(Serialize)]
        struct PermGetParams {
            #[serde(rename = "userIds")]
            user_ids: String,
        }
        let params = PermGetParams {
            user_ids: user_ids.join(","),
        };
        self.post(
            "/friend/permission/get.json",
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
    async fn test_friend_ops() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock_add = server
            .mock("POST", "/friend/add.json")
            .with_status(200)
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;
        let _ = client.friend_add("u1", "u2").await;
        mock_add.assert_async().await;

        let mock_del = server
            .mock("POST", "/friend/delete.json")
            .with_status(200)
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;
        let _ = client.friend_delete("u1", vec!["u2"]).await;
        mock_del.assert_async().await;
    }
}
