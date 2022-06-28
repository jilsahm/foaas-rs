use std::sync::Arc;

use reqwest::{Client, IntoUrl, Url};

use crate::{operation::Operation, content_type::ContentType, error::FoaasError};

#[derive(Clone)]
pub struct FoaasClient {
    client: Arc<Client>,
    url: Url,
    content_type: ContentType,
    shoutcloud: bool,
    i18n: Option<String>,
}

impl FoaasClient {

    pub fn new<U>(fqdn: U) -> Result<Self, FoaasError> 
    where
        U: IntoUrl
    {
        let mut client = reqwest::ClientBuilder::new().user_agent("foaas-client-rs");

        if cfg!(not(test)) {
            client = client.https_only(true);
        }
            
        let client = client.build().map_err(|e| FoaasError::ClientConfigurationError(e.to_string()))?;

        Ok(Self { 
            client: Arc::new(client),
            url: fqdn.into_url()?,
            content_type: ContentType::Json,
            shoutcloud: false,
            i18n: None,
        })
    }

    pub fn shout(&self) -> Self {
        let mut clone = self.clone();
        clone.shoutcloud = true;
        clone
    }

    pub fn i18n(&self, language: &str) -> Self {
        let mut clone = self.clone();
        clone.i18n = Some(language.to_string());
        clone
    }

    async fn send(&self, operation: Operation) -> Result<String, FoaasError> {
        let mut request = self.client.get(format!("{}{}", self.url, operation.uri()))
            .header("Accept", self.content_type.to_header_value());

        if self.shoutcloud {
            request = request.query(&[("shoutcloud", "true")]);
        }

        if let Some(code) = self.i18n.as_ref() {
            request = request.query(&[("i18n", code)]);
        }

        let bytes = request
            .send()
            .await?
            .bytes()
            .await?;

        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    pub async fn operation(&self, operation: Operation) -> Result<String, FoaasError> {
        self.send(operation).await
    }

    pub async fn version(&self) -> Result<String, FoaasError> {
        self.send(Operation::Version).await
    }

    pub async fn operations(&self) -> Result<String, FoaasError> {
        self.send(Operation::Operations).await
    }

    pub async fn absolutely(&self, company: &str, from: &str) -> Result<String, FoaasError> {
        self.send(Operation::Absolutely { company: company.to_string(), from: from.to_string(), }).await
    }
}

#[cfg(test)] 
mod tests {
    use wiremock::{matchers::{header, method, path}, Mock, MockServer, ResponseTemplate};

    use crate::FoaasClient;


    #[tokio::test]
    async fn absolutely() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/absolutely/company/from"))
            .and(header("Accept", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"message":"Absolutely fucking Not, company, No Fucking Way!","subtitle":"- from"}"#))
            .mount(&mock_server)
            .await;

        let client = FoaasClient::new(mock_server.uri()).expect("valid test client");
        let result = client.absolutely("company", "from").await;
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(r#"{"message":"Absolutely fucking Not, company, No Fucking Way!","subtitle":"- from"}"#, result.unwrap());
    }
}