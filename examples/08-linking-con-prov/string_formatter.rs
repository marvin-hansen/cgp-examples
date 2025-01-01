use std::fmt::{Debug, Display};

pub trait StringFormatter<Context> {
    fn format_string(context: &Context) -> String;
}
pub struct FormatStringWithDisplay;

pub struct FormatStringWithDebug;

impl<Context> StringFormatter<Context> for FormatStringWithDisplay
where
    Context: Display,
{
    fn format_string(context: &Context) -> String {
        format!("{}", context)
    }
}

impl<Context> StringFormatter<Context> for FormatStringWithDebug
where
    Context: Debug,
{
    fn format_string(context: &Context) -> String {
        format!("{:?}", context)
    }
}
