use thiserror::Error;

use crate::{config::ConfigError, file::FileError, graphics::GraphicsError};

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("graphics: {source}")]
    Graphics {
        #[from]
        source: GraphicsError,
    },
    #[error("config: {source}")]
    Config {
        #[from]
        source: ConfigError,
    },
    #[error("file: {source}")]
    File {
        #[from]
        source: FileError,
    },
    #[error("invalid window size: {0}x{1}")]
    InvalidWindowSize(f64, f64),
}
