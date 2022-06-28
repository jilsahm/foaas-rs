use clap::Parser;
use foaas_client_rs::{FoaasClient, FoaasClientConfiguration};

#[tokio::main]
async fn main() {
    let config = FoaasClientConfiguration::parse();
    match FoaasClient::new(config.url) {
        Err(what) => print!("{}", what),
        Ok(client) => {
            match client.operation(config.operation).await {
                Err(what) => print!("{}", what),
                Ok(result) => print!("{}", result),
            }
        }
    }
}