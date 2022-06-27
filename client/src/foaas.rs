use reqwest::Client;

use crate::operation::Operation;

pub struct FoaasClient {
    client: Client,
}

impl FoaasClient {

    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            .user_agent("foaas-client-rs")
            .https_only(true)
            .build()
            .expect("valid client config");

        Self { client, }
    }

    async fn send(&self, operation: Operation<'_>) -> String {
        let bytes = self.client.get(format!("https://foaas.com{}", operation.uri()))
            .header("Accept", "application/json")
            .send()
            .await
            .expect("valid REST call")
            .bytes()
            .await
            .expect("valid response body");

        String::from_utf8_lossy(&bytes).to_string()
    }

    pub async fn absolutely(&self, company: &str, from: &str) -> String {
        self.send(Operation::Absolutely { from, company, }).await
    }
}