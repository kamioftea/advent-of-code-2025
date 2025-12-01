use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum BootstrapError {
    IoError(std::io::Error),
    ReqwestError(reqwest::Error),
    HtmlParseError(String),
}

impl From<reqwest::Error> for BootstrapError {
    fn from(err: reqwest::Error) -> Self {
        BootstrapError::ReqwestError(err)
    }
}

impl From<std::io::Error> for BootstrapError {
    fn from(err: std::io::Error) -> Self {
        BootstrapError::IoError(err)
    }
}

impl From<String> for BootstrapError {
    fn from(err: String) -> Self {
        BootstrapError::HtmlParseError(err)
    }
}

impl Display for BootstrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootstrapError::IoError(e) => write!(f, "IO Error: {}", e),
            BootstrapError::ReqwestError(e) => write!(f, "Reqwest Error: {}", e),
            BootstrapError::HtmlParseError(e) => write!(f, "HTML Parse Error: {}", e),
        }
    }
}

impl Error for BootstrapError {}