pub enum CustomError {
    IoError(std::io::Error),
    LoadGameDataError(serde_json::error::Error),
    UnsupportedRawLevelError,
}
