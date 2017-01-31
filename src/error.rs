//! Error types

use lettre::email::error::Error as EmailError;
use lettre::transport::smtp::error::Error as SmtpError;
use reqwest::Error as ReqwestError;
use std::io::Error as IoError;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum DomoError {
    Config(IoError),
    Email(EmailError),
    Http(ReqwestError),
    Smtp(SmtpError),
    Domoticz,
}

impl fmt::Display for DomoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DomoError::Config(ref err) => write!(f, "Config erro: {}", err),
            DomoError::Email(ref err) => write!(f, "Email error: {}", err),
            DomoError::Http(ref err) => write!(f, "Http error: {}", err),
            DomoError::Smtp(ref err) => write!(f, "Smtp error: {}", err),
            DomoError::Domoticz => write!(f, "Domoticz error"),
        }
    }
}

impl error::Error for DomoError {
    fn description(&self) -> &str {
        match *self {
            DomoError::Config(ref err) => err.description(),
            DomoError::Email(ref err) => err.description(),
            DomoError::Http(ref err) => err.description(),
            DomoError::Smtp(ref err) => err.description(),
            DomoError::Domoticz => "Domoticz error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DomoError::Config(ref err) => Some(err),
            DomoError::Email(ref err) => Some(err),
            DomoError::Http(ref err) => Some(err),
            DomoError::Smtp(ref err) => Some(err),
            DomoError::Domoticz => None,
        }
    }
}

impl From<IoError> for DomoError {
    fn from(err: IoError) -> DomoError {
        DomoError::Config(err)
    }
}

impl From<EmailError> for DomoError {
    fn from(err: EmailError) -> DomoError {
        DomoError::Email(err)
    }
}

impl From<ReqwestError> for DomoError {
    fn from(err: ReqwestError) -> DomoError {
        DomoError::Http(err)
    }
}

impl From<SmtpError> for DomoError {
    fn from(err: SmtpError) -> DomoError {
        DomoError::Smtp(err)
    }
}
