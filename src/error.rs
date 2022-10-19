use thiserror::Error;

#[derive(Error, Debug)]
pub enum FishFishError {
    #[error("http request failed")]
    HttpError(#[from] reqwest::Error),
    #[error("failed to parse response json")]
    ResponseParseFailed(reqwest::Error),
    #[error("failed to initialize http client")]
    ClientInitializationError(reqwest::Error),
    #[error("token is not valid")]
    InvalidTokenError,
    #[error("failed to stringify input parameters")]
    SerializationError,
    #[error("failed to refresh token")]
    TokenRefreshError(reqwest::Error),
    #[error("can't call an authenticated endpoint without a token")]
    NotAuthenticatedError,
    #[error("time went backwards")]
    TimeWentBackwardsError,
    #[error("api returned forbidden")]
    ForbiddenError,
}