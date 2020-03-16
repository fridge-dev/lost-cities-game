use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub enum ClientGameError {
    BackendFault,
    BackendTimeout,
    BackendUnknown,
}

impl Error for ClientGameError {}

impl Display for ClientGameError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            ClientGameError::BackendFault => f.write_str("Calling backend failed. CRAP."),
            ClientGameError::BackendTimeout => f.write_str("Timeout while calling backend."),
            ClientGameError::BackendUnknown => f.write_str("Unknown backend failure. Should probably handle this branch before it gets to this point."),
        }
    }
}
