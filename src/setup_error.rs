use ::std::error::Error;

use ::std::fmt;
use ::std::io;

/// Represents the error returned on the setup request reply.
#[derive(Debug)]
pub enum SetupError {
    /// The connection was refused.
    Failed(String),

    /// Further authentication is needed.
    Authenticate(String),

    /// An I/O error.
    Io(io::Error),
}

impl From<io::Error> for SetupError {
    fn from(e: io::Error) -> SetupError {
        SetupError::Io(e)
    }
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetupError::Failed(ref e) => write!(f, "Connection refused: {}", e),
            SetupError::Authenticate(ref e) => write!(f, "Authentication error: {}", e),
            SetupError::Io(ref e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl Error for SetupError {
    fn description(&self) -> &str {
        match *self {
            SetupError::Failed(_) => "Connection refused",
            SetupError::Authenticate(_) => "Authentication error",
            SetupError::Io(_) => "I/O error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SetupError::Io(ref e) => Some(e),
            _ => None,
        }
    }
}
