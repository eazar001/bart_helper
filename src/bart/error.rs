use core::fmt;
use core::fmt::{Display, Formatter};
use std::error::Error;
use lambda_runtime::error::{LambdaErrorExt, HandlerError};
use BartError::*;


#[derive(Debug)]
pub enum BartError {
    NoConnection(String),
    InvalidStation(String),
    BadParse(String),
    BadPriceConversion,
    BadCalculation,
    MissingSlot,
    BadRegex,
    BadIntent,
    MissingIntent
}

impl Display for BartError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InvalidStation(e) => write!(f, "{:?} is an invalid station", e),
            BadParse(e) => write!(f, "Bad parse error: {:?}", e),
            NoConnection(e) => write!(f, "Network connection error: {:?}", e),
            _ => write!(f, "{:?}", self)
        }
    }
}

impl Error for BartError { }

impl LambdaErrorExt for BartError {
    fn error_type(&self) -> &str {
        match self {
            NoConnection(_) => "bart::error::NoConnection",
            InvalidStation(_) => "bart::error::InvalidStation",
            BadParse(_) => "bart::error::BadParse",
            BadPriceConversion => "bart::error::BadPriceConversion",
            BadCalculation => "bart::error::BadCalculation",
            MissingSlot => "bart::error::MissingSlot",
            BadRegex => "bart::error::BadRegex",
            BadIntent => "bart::error::BadIntent",
            MissingIntent => "bart::error::MissingIntent"
        }
    }
}

impl From<BartError> for HandlerError {
    fn from(e: BartError) -> Self {
        HandlerError::new(e)
    }
}
