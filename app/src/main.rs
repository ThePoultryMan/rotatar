use rotatar_types::{Args, ArgsError, FrontendError, Parser, ValidArgs};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("Invalid arguments were passed")]
    Args(#[from] ArgsError),
    #[error("The backend exited with an error")]
    BackendError(#[from] rotatar_backend::Error),
    #[error("An error occurred from the frontend")]
    FrontendError(#[from] FrontendError),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args: ValidArgs = Args::parse().try_into()?;

    let config = rotatar_backend::run(&args).await?;
    match args.frontend() {
        #[cfg(feature = "iced-frontend")]
        rotatar_types::Frontend::Iced => iced_frontend::run(args, config).await?,
        #[cfg(feature = "tauri-frontend")]
        rotatar_types::Frontend::Tauri => tauri_frontend::run(config),
    }

    Ok(())
}
