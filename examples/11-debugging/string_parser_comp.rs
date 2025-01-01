use anyhow::Error;
use cgp::prelude::*;
use serde::Deserialize;

// Component definitions
#[cgp_component {
    name: StringParserComponent,
    provider: StringParser,
    context: Context,
    }]
pub trait CanParseFromString: Sized {
    fn parse_from_string(raw: &str) -> Result<Self, Error>;
}

// Context Generic default implementation for StringParser
pub struct ParseFromJsonString;
impl<Context> StringParser<Context> for ParseFromJsonString
where
    Context: for<'a> Deserialize<'a>,
{
    fn parse_from_string(json_str: &str) -> Result<Context, Error> {
        Ok(serde_json::from_str(json_str)?)
    }
}
