use core::fmt;
use core::fmt::{Display, Formatter};
use std::error::Error;


#[derive(Debug)]
enum BartError {
    NoConnection,
    InvalidStation,
    BadParse,
    BadPriceConversion,
    BadCalculation,
    MissingSlot,
    BadRegex,
    BadIntent,
    MissingIntent,
}

impl Display for BartError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for BartError { }
