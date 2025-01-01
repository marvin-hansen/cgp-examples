use anyhow::Error;
use cgp::prelude::*;
use serde::Serialize;

#[cgp_component {
    name: StringFormatterComponent,
    provider: StringFormatter,
    context: Context,
    }]
pub trait CanFormatToString {
    fn format_to_string(&self) -> Result<String, Error>;
}



// Context Generic default implementation for StringFormatter
pub struct FormatAsJsonString;
impl<Context> StringFormatter<Context> for FormatAsJsonString
where
    Context: Serialize,
{
    fn format_to_string(context: &Context) -> Result<String, Error> {
        Ok(serde_json::to_string(context)?)
    }
}