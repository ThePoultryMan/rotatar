use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrontendError {
    #[error("An error origination from the iced frontend")]
    Iced,
}
