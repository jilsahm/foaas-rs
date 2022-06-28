use thiserror::Error;

#[derive(Debug, Error)]
pub enum FoaasError {

    #[error("failed to configure FOAAS client because: {0}")]
    ClientConfigurationError(String),

    #[error("failed to call FOAAS because: {0}")]
    IoError(#[from] reqwest::Error),
}