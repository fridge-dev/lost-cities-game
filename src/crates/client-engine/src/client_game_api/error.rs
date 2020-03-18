use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use tonic::Code;
use std::borrow::Cow;

#[derive(Debug)]
pub enum ClientGameError {
    NotFound,
    UserInvalidArg,
    BackendFault,
    BackendTimeout,
    BackendUnknown,
    MalformedResponse(/* message */ Cow<'static, str>),
}

impl Error for ClientGameError {}

impl Display for ClientGameError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            ClientGameError::BackendFault => f.write_str("Calling backend failed. CRAP."),
            ClientGameError::BackendTimeout => f.write_str("Timeout while calling backend."),
            ClientGameError::BackendUnknown => f.write_str("Unknown backend failure. Should probably handle this branch before it gets to this point."),
            ClientGameError::UserInvalidArg => f.write_str("User fricked up."),
            ClientGameError::NotFound => f.write_str("Crap, where'd it go?"),
            ClientGameError::MalformedResponse(msg) => f.write_str(&format!("Server gave us a payload that ain't make sense: {}", msg)),
        }
    }
}

impl From<tonic::Status> for ClientGameError {
    fn from(status: tonic::Status) -> Self {
        match status.code() {
            Code::InvalidArgument => ClientGameError::UserInvalidArg,
            Code::AlreadyExists => ClientGameError::UserInvalidArg,
            Code::NotFound => ClientGameError::NotFound,
            _ => ClientGameError::BackendUnknown
        }
    }
}
