#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Uninitialized Ws2811")]
    Uninitialized,

    #[error("Error with VideoCore request")]
    VideoCoreError,

    #[error("Failed at VideoCore request")]
    VideoCoreFailed,

    #[error("Partial error with VideoCore request")]
    VideoCorePartialError,

    #[error("Vc memory uninitialized")]
    VcMemUninitialized,

    #[error("SMI uninitialized")]
    SmiUninitialized,

    #[error("Invalid LED index")]
    InvalidLed,

    #[error("Invalid channel number")]
    InvalidChannel,
}
