use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("error making http request")]
    Http(#[from] reqwest::Error),
    #[error("error getting environment variable")]
    Env(#[from] std::env::VarError),
    #[error("error parsing URL")]
    Url(#[from] url::ParseError),
}
