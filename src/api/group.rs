use crate::core::RongCloud;
use crate::core::RongCloudError;
use crate::types::RcResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GroupModel<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

impl RongCloud {
    /// Create a group.
    pub async fn group_create(
        &self,
        user_ids: Vec<&str>,
        group_id: &str,
        group_name: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = vec![
            ("groupId", group_id.to_string()),
            ("groupName", group_name.to_string()),
        ];
        for id in user_ids {
            params.push(("userId", id.to_string()));
        }

        self.post(
            super::endpoints::GROUP_CREATE,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Sync user groups.
    pub async fn group_sync(
        &self,
        user_id: &str,
        groups: Vec<GroupModel<'_>>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params: Vec<(String, String)> = vec![("userId".to_string(), user_id.to_string())];
        for group in groups {
            params.push((format!("group[{}]", group.id), group.name.to_string()));
        }

        self.post(
            super::endpoints::GROUP_SYNC,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Update group.
    pub async fn group_update(
        &self,
        group_id: &str,
        group_name: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("groupId", group_id), ("groupName", group_name)];
        self.post(
            super::endpoints::GROUP_REFRESH,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Join group.
    pub async fn group_join(
        &self,
        user_ids: Vec<&str>,
        group_id: &str,
        group_name: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = vec![
            ("groupId", group_id.to_string()),
            ("groupName", group_name.to_string()),
        ];
        for id in user_ids {
            params.push(("userId", id.to_string()));
        }
        self.post(
            super::endpoints::GROUP_JOIN,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Quit group.
    pub async fn group_quit(
        &self,
        user_ids: Vec<&str>,
        group_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = vec![("groupId", group_id.to_string())];
        for id in user_ids {
            params.push(("userId", id.to_string()));
        }
        self.post(
            super::endpoints::GROUP_QUIT,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Dismiss group.
    pub async fn group_dismiss(
        &self,
        user_id: &str,
        group_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("userId", user_id), ("groupId", group_id)];
        self.post(
            super::endpoints::GROUP_DISMISS,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query group users.
    pub async fn group_user_query(&self, group_id: &str) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("groupId", group_id)];
        self.post(
            super::endpoints::GROUP_USER_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query user groups.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/user/group/query
    pub async fn user_group_query(&self, user_id: &str) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("userId", user_id)];
        self.post(
            super::endpoints::USER_GROUP_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    // ========================================================================
    // 群组禁言服务
    // ========================================================================

    /// Gag group member.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/group/user/gag/add
    pub async fn group_user_gag_add(
        &self,
        group_id: &str,
        user_ids: Vec<&str>,
        minute: i32,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = vec![
            ("groupId", group_id.to_string()),
            ("minute", minute.to_string()),
        ];
        for id in user_ids {
            params.push(("userId", id.to_string()));
        }
        self.post(
            super::endpoints::GROUP_USER_GAG_ADD,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Ungag group member.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/group/user/gag/rollback
    pub async fn group_user_gag_rollback(
        &self,
        group_id: &str,
        user_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = vec![("groupId", group_id.to_string())];
        for id in user_ids {
            params.push(("userId", id.to_string()));
        }
        self.post(
            super::endpoints::GROUP_USER_GAG_ROLLBACK,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// List gagged group members.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/group/user/gag/list
    pub async fn group_user_gag_list(
        &self,
        group_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("groupId", group_id)];
        self.post(
            super::endpoints::GROUP_USER_GAG_LIST,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Ban entire group (全体禁言).
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/group/ban/add
    pub async fn group_ban_add(&self, group_id: &str) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("groupId", group_id)];
        self.post(
            super::endpoints::GROUP_BAN_ADD,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Unban entire group.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/group/ban/rollback
    pub async fn group_ban_rollback(
        &self,
        group_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("groupId", group_id)];
        self.post(
            super::endpoints::GROUP_BAN_ROLLBACK,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query group ban status.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/group/ban/query
    pub async fn group_ban_query(&self, group_id: &str) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("groupId", group_id)];
        self.post(
            super::endpoints::GROUP_BAN_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Add user to group ban whitelist.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/group/user/ban/whitelist/add
    pub async fn group_user_ban_whitelist_add(
        &self,
        group_id: &str,
        user_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = vec![("groupId", group_id.to_string())];
        for id in user_ids {
            params.push(("userId", id.to_string()));
        }
        self.post(
            super::endpoints::GROUP_USER_BAN_WHITELIST_ADD,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Remove user from group ban whitelist.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/group/user/ban/whitelist/rollback
    pub async fn group_user_ban_whitelist_rollback(
        &self,
        group_id: &str,
        user_ids: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = vec![("groupId", group_id.to_string())];
        for id in user_ids {
            params.push(("userId", id.to_string()));
        }
        self.post(
            super::endpoints::GROUP_USER_BAN_WHITELIST_ROLLBACK,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query group ban whitelist.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/group/user/ban/whitelist/query
    pub async fn group_user_ban_whitelist_query(
        &self,
        group_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("groupId", group_id)];
        self.post(
            super::endpoints::GROUP_USER_BAN_WHITELIST_QUERY,
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
    async fn test_group_ops() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock_create = server
            .mock("POST", "/group/create.json")
            .with_status(200)
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;
        let _ = client.group_create(vec!["u1"], "g1", "group1").await;
        mock_create.assert_async().await;
    }
}
