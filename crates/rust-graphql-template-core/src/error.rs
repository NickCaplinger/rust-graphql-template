use thiserror::Error;

// TODO Flesh this out based on your application's needs.
#[derive(Debug, Error)]
pub enum Error {
    // FIXME This is a rather poor error variant provided as an example.
    // You should provide more useful error variants for your library's users!
    #[error("unexpected error")]
    Unknown,
}
