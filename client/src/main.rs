use foaas_client_rs::FoaasClient;

#[tokio::main]
async fn main() {
    let client = FoaasClient::new();

    print!("{}", client.absolutely("a", "b").await);
}