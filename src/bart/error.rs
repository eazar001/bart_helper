use core::fmt;
use core::fmt::{Display, Formatter};
use std::error::Error;
use lambda_runtime::error::LambdaErrorExt;


#[derive(Debug)]
pub enum BartError {
    NoConnection,
    InvalidStation,
    BadParse,
    BadPriceConversion,
    BadCalculation,
    MissingSlot,
    BadRegex,
    BadIntent,
    MissingIntent
}

impl Display for BartError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for BartError { }

impl LambdaErrorExt for BartError {
    fn error_type(&self) -> &str {
        match self {
            BartError::NoConnection => "NoConnection",
            BartError::InvalidStation => "InvalidStation",
            BartError::BadParse => "BadParse",
            BartError::BadPriceConversion => "BadPriceConversion",
            BartError::BadCalculation => "BadCalculation",
            BartError::MissingSlot => "MissingSlot",
            BartError::BadRegex => "BadRegex",
            BartError::BadIntent => "BadIntent",
            BartError::MissingIntent => "MissingIntent"
        }
    }
}
