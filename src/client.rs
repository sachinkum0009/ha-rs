use reqwest::{Client, Error};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use crate::utils::Entity;

pub struct HaClient {
    base_url: String,
    token: String,
    client: Client,
}

impl HaClient {
    pub fn new(base_url: String, token: String) -> Self {
        let client = Client::new();
        HaClient { base_url, token, client }
    }

    pub async fn get_state(&self, entity_id: &str) -> Result<Entity, Error> {
        let url = format!("{}/api/states/{}", self.base_url, entity_id);

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let entity = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .json::<Entity>()
            .await?;

        Ok(entity)
    }

    pub async fn set_state(&self, entity_id: &str, turn_on: bool) -> Result<String, Error> {
        // Extract domain from entity_id (e.g., "switch" from "switch.smart_plug")
        let domain = entity_id.split('.').next().unwrap_or("switch");
        let service = if turn_on { "turn_on" } else { "turn_off" };
        let url = format!("{}/api/services/{}/{}", self.base_url, domain, service);

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let body = serde_json::json!({
            "entity_id": entity_id
        });

        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}
