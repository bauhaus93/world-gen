use thiserror::Error;

use crate::GraphicsError;

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("resource: {0}")]
    Resource(String),
    #[error("graphics: {source}")]
    Graphics {
        #[from]
        source: GraphicsError,
    },
    #[error("internal: {0}")]
    Internal(String),
}
