use crate::string_formatter_comp::{FormatAsJsonString, StringFormatterComponent};
use crate::string_parser_comp::{ParseFromJsonString, StringParserComponent};
use cgp::prelude::*;
use serde::{Deserialize, Serialize};

// Re-exports underlying traits, for the convenience of a single import.
pub use crate::string_formatter_comp::CanFormatToString;
pub use crate::string_parser_comp::CanParseFromString;

// Concrete  type

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

// Aggregate component type
pub struct PersonComponents;

impl HasComponents for Person {
    // Define associated type as PersonComponents
    type Components = PersonComponents;
}

// Wire components to implementations
delegate_components! {
    PersonComponents {
        StringFormatterComponent: FormatAsJsonString,
        StringParserComponent: ParseFromJsonString,
    }
}
