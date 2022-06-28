use clap::Parser;
use reqwest::Url;

use crate::{content_type::ContentType, operation::Operation};

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct FoaasClientConfiguration {

    #[clap(
        long,
        default_value = "https://foaas.com",
    )]
    pub url: Url,

    #[clap(
        long = "out",
        default_value = "json"
    )]
    pub content_type: ContentType, 

    #[clap(subcommand)]
    pub operation: Operation,
}