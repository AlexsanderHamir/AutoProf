use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ProfileParsingError {
    FileReadError(std::io::Error),
    EmptyFile,
    InvalidFormat(String),
    IncompleteHeader(String),
    IncompleteBody(String),
}

impl fmt::Display for ProfileParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfileParsingError::FileReadError(err) => write!(f, "Failed to read profile file: {}", err),
            ProfileParsingError::EmptyFile => write!(f, "Profile file is empty"),
            ProfileParsingError::InvalidFormat(msg) => write!(f, "Invalid profile format: {}", msg),
            ProfileParsingError::IncompleteHeader(msg) => write!(f, "Incomplete header: {}", msg),
            ProfileParsingError::IncompleteBody(msg) => write!(f, "Incomplete body: {}", msg),
        }
    }
}

impl Error for ProfileParsingError {}

impl From<std::io::Error> for ProfileParsingError {
    fn from(err: std::io::Error) -> Self {
        ProfileParsingError::FileReadError(err)
    }
}

#[derive(Debug)]
pub struct FunctionProfileData {
    pub function_name: String,
    pub flat: f64,
    pub flat_percentage: f64,
    pub sum_percentage: f64,
    pub cum: f64,
    pub cum_percentage: f64,
}

impl FunctionProfileData {
    pub fn new(function_name: String, flat: f64, flat_percentage: f64, sum_percentage: f64, cum: f64, cum_percentage: f64) -> Self {
        Self {
            function_name,
            flat,
            flat_percentage,
            sum_percentage,
            cum,
            cum_percentage,
        }
    }
}
