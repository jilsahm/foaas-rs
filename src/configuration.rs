use std::net::SocketAddr;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct FoaasConfiguration {

    #[clap(
        help = "the socket address the server will bound on",
        default_value = "127.0.0.1:8080",
    )]
    pub socket_address: SocketAddr,
}