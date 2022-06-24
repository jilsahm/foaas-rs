#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

pub use configuration::FoaasConfiguration;

mod api;
mod configuration;


pub async fn run(configuration: FoaasConfiguration) {
    api::create_server(&configuration).await;
}