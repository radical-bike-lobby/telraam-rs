use thiserror::Error;

use crate::response::Status;

#[derive(Error, Debug)]
pub enum Error {
    /// An error occured on the request
    #[error("status_code:{}:{}", .0.status_code, .0.message)]
    Non200Response(Status),
}
