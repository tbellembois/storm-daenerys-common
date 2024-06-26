use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum CommonError {
    NotFound,
    InvalidQueryParamater(String),
    MissingQueryParamater(String),
    InvalidDirectoryName,
    InternalServerError(String),
    Timeout(String),
    UserNotFoundInGroup,
    UnexpectedResponse(String),
    CanNotModifyNonRestrictedDirectory(String),
}

// Implement std::fmt::Display for CommonError
impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommonError::NotFound => write!(f, "not found"),
            CommonError::InvalidQueryParamater(p) => write!(f, "invalid query parameter {}", p),
            CommonError::MissingQueryParamater(p) => write!(f, "missing query parameter {}", p),
            CommonError::InvalidDirectoryName => write!(f, "invalid directory name"),
            CommonError::InternalServerError(e) => write!(f, "internal server error {}", e),
            CommonError::Timeout(t) => write!(f, "timeout {}", t),
            CommonError::UserNotFoundInGroup => write!(f, "user not found in group"),
            CommonError::UnexpectedResponse(e) => write!(f, "unexpected response {}", e),
            CommonError::CanNotModifyNonRestrictedDirectory(s) => {
                write!(f, "can not modify the non restricted directory {}", s)
            }
        }
    }
}

// Implement std::fmt::Debug for CommonError
impl fmt::Debug for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
