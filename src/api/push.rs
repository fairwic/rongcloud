use crate::core::RongCloud;
use crate::core::RongCloudError;
use crate::types::RcResponse;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Clone, PartialEq)]
// RongCloud might expect ["ios", "android"] or generic strings.
// Documentation says: platform: ["ios", "android"] lowercase usually, or "all" string if broadcast?
// Java SDK uses String array. Let's assume lowercase string literals.
// If valid values are known: ios, android
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Ios,
    Android,
    Web,
    Pc,
}

#[derive(Debug, Serialize)]
pub struct PushModel {
    pub platform: Vec<Platform>,
    pub audience: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Value>,
}

impl RongCloud {
    /// Send push notification.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/push/send
    pub async fn push(&self, payload: &PushModel) -> Result<RcResponse<()>, RongCloudError> {
        self.post(super::endpoints::PUSH, payload, "application/json")
            .await
    }

    /// Send broadcast.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/push/broadcast
    /// Note: The Java SDK uses /push.json for broadcast as well, but with different payload structure.
    /// Here we reuse the generic push method as the endpoint is the same.
    pub async fn push_broadcast(
        &self,
        payload: &PushModel,
    ) -> Result<RcResponse<()>, RongCloudError> {
        self.post(super::endpoints::PUSH, payload, "application/json")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::RongCloudConfig;
    use mockito;
    use serde_json::json;

    #[tokio::test]
    async fn test_push() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock_push = server
            .mock("POST", "/push.json")
            .with_status(200)
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;

        let payload = PushModel {
            platform: vec![Platform::Ios],
            audience: json!({"is_to_all": true}),
            notification: Some(json!({"alert": "hello"})),
            message: None,
        };
        let _ = client.push(&payload).await;
        mock_push.assert_async().await;
    }
}
