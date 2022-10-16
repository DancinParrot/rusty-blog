use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Server error")]
    GatewayError,
    #[error("Unknown error occured.")]
    Unknown,
}