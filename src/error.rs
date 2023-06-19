use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExcalidockerError {
    #[error(
        "File '{}' has unsupported extension. File should be 'yaml' or 'yml'",
        path
    )]
    FileIncorrectExtension { path: String },
    #[error("Failed to open '{}'. Details: {}", path, msg)]
    FileNotFound { path: String, msg: String },
    #[error("Failed to read '{}'. Details: {}", path, msg)]
    FileFailedRead { path: String, msg: String },
    #[error("Failed to download '{}'. Details: {}", path, msg)]
    RemoteFileFailedRead { path: String, msg: String },
    #[error("Failed to parse provided docker-compose '{}'. Details: {}", path, msg)]
    InvalidDockerCompose { path: String, msg: String },
}
