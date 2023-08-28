use thiserror::Error;

// TODO Flesh this out based on your API's needs.
#[derive(Debug, Error)]
pub enum Error {
    #[error("authentication failed")]
    AuthenticationFailure,
    #[error("something failed internally")]
    Internal,
}
