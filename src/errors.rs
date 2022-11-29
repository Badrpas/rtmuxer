
use std::*;


#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("Something went wrong while executing command.")]
    GenericCommandError,
    
    #[error("Error while running: {0}")]
    FailedCommandError(#[from] std::io::Error),

}

