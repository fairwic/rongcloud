use crate::core::RongCloud;
use crate::core::RongCloudError;
use crate::types::RcResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ChatroomModel<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

impl RongCloud {
    /// Create chatroom.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/chatroom/create
    pub async fn chatroom_create(
        &self,
        chatrooms: Vec<ChatroomModel<'_>>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params: Vec<(String, String)> = Vec::new();
        for chatroom in chatrooms {
            params.push((
                format!("chatroom[{}]", chatroom.id),
                chatroom.name.to_string(),
            ));
        }
        self.post(
            super::endpoints::CHATROOM_CREATE,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Destroy chatroom.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/chatroom/destroy
    pub async fn chatroom_destroy(
        &self,
        chatroom_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("chatroomId", chatroom_id)];
        self.post(
            super::endpoints::CHATROOM_DESTROY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query chatroom info.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/chatroom/info
    pub async fn chatroom_query(
        &self,
        chatroom_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("chatroomId", chatroom_id)];
        self.post(
            super::endpoints::CHATROOM_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Query chatroom users.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/chatroom/user/query
    pub async fn chatroom_user_query(
        &self,
        chatroom_id: &str,
        count: i32,
        order: i32,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![
            ("chatroomId", chatroom_id.to_string()),
            ("count", count.to_string()),
            ("order", order.to_string()),
        ];
        self.post(
            super::endpoints::CHATROOM_USER_QUERY,
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Check if user exists in chatroom.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/chatroom/user/exist
    pub async fn chatroom_user_exist(
        &self,
        chatroom_id: &str,
        user_id: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("chatroomId", chatroom_id), ("userId", user_id)];
        self.post(
            super::endpoints::CHATROOM_USER_EXIST,
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
    async fn test_chatroom_ops() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock_create = server
            .mock("POST", "/chatroom/create_new.json")
            .with_status(200)
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;

        let chatrooms = vec![ChatroomModel {
            id: "c1",
            name: "room1",
        }];
        let _ = client.chatroom_create(chatrooms).await;
        mock_create.assert_async().await;
    }
}
