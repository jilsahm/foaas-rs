use clap::Parser;
use foaas_rs::{run, FoaasConfiguration};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let configuration = FoaasConfiguration::parse();
    run(configuration).await;    
}

