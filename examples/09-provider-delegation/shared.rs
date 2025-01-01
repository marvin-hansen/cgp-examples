// Our person struct
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

pub trait HasComponents {
    type Components;
}

use anyhow::Error;
use serde::{Deserialize, Serialize};

pub trait CanFormatToString {
    fn format_to_string(&self) -> Result<String, Error>;
}

pub trait CanParseFromString: Sized {
    fn parse_from_string(raw: &str) -> Result<Self, Error>;
}

pub trait StringFormatter<Context> {
    fn format_to_string(context: &Context) -> Result<String, Error>;
}

pub trait StringParser<Context> {
    fn parse_from_string(raw: &str) -> Result<Context, Error>;
}
