use thiserror::Error;

use crate::{config::ConfigError, graphics::GraphicsError, UpdateError};

#[derive(Error, Debug)]
pub enum StateError {
    #[error("update: {source}")]
    Update {
        #[from]
        source: UpdateError,
    },
    #[error("config: {source}")]
    Config {
        #[from]
        source: ConfigError,
    },
    #[error("graphics: {source}")]
    Graphics {
        #[from]
        source: GraphicsError,
    },
    #[error("setup: {0}")]
    Setup(String),
}
