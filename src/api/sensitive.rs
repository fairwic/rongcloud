use crate::core::RongCloud;
use crate::core::RongCloudError;
use crate::types::RcResponse;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum SensitiveType {
    Replace = 0,
    Block = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensitiveWordModel {
    #[serde(rename = "type")]
    pub typ: SensitiveType,
    pub word: String,
    #[serde(rename = "replaceWord", skip_serializing_if = "Option::is_none")]
    pub replace_word: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AddSensitiveWordsModel {
    pub words: Vec<SensitiveWordModel>,
}

impl RongCloud {
    /// Add sensitive word.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/sensitiveword/add
    pub async fn sensitive_word_add(
        &self,
        word: &str,
        bind_type: SensitiveType,
        replace_word: Option<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = vec![("word".to_string(), word.to_string())];
        if bind_type == SensitiveType::Replace {
            if let Some(rw) = replace_word {
                params.push(("replaceWord".to_string(), rw.to_string()));
            }
        }

        self.post(
            "/sensitiveword/add.json",
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Batch add sensitive words.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/sensitiveword/batch/add
    pub async fn sensitive_word_batch_add(
        &self,
        words: Vec<SensitiveWordModel>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = AddSensitiveWordsModel { words };
        self.post("/sensitiveword/batch/add.json", &params, "application/json")
            .await
    }

    /// List sensitive words.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/sensitiveword/list
    pub async fn sensitive_word_list(
        &self,
        typ: Option<SensitiveType>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = Vec::new();
        if let Some(t) = typ {
            let val = t as u8;
            params.push(("type".to_string(), val.to_string()));
        }
        self.post(
            "/sensitiveword/list.json",
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Remove sensitive word.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/sensitiveword/delete
    pub async fn sensitive_word_remove(
        &self,
        word: &str,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let params = vec![("word".to_string(), word.to_string())];
        self.post(
            "/sensitiveword/delete.json",
            &params,
            "application/x-www-form-urlencoded",
        )
        .await
    }

    /// Batch remove sensitive words.
    ///
    /// See: https://doc.rongcloud.cn/imserver/server/v1/sensitiveword/batch/delete
    pub async fn sensitive_word_batch_remove(
        &self,
        words: Vec<&str>,
    ) -> Result<RcResponse<()>, RongCloudError> {
        let mut params = Vec::new();
        for word in words {
            params.push(("words".to_string(), word.to_string()));
        }
        self.post(
            "/sensitiveword/batch/delete.json",
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
    async fn test_sensitive_word_ops() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let config = RongCloudConfig::new("app_key", "app_secret").with_api_url(url);
        let client = RongCloud::new(config);

        let mock_add = server
            .mock("POST", "/sensitiveword/add.json")
            .with_status(200)
            .with_body(r#"{"code": 200}"#)
            .create_async()
            .await;
        let _ = client
            .sensitive_word_add("bad", SensitiveType::Block, None)
            .await;
        mock_add.assert_async().await;
    }
}
