mod string_formatter_comp;
mod string_parser_comp;

use crate::string_formatter_comp::{
    CanFormatToString, FormatAsJsonString, StringFormatterComponent,
};
use crate::string_parser_comp::{CanParseFromString, ParseFromJsonString, StringParserComponent};
use cgp::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// Concrete  type
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

// Static check that statically verifies all dependencies are present in the callsite.
#[allow(dead_code)] // Somehow clippy doesn't see its usage below.
pub trait CanUsePerson:
    Sized + Serialize + for<'a> Deserialize<'a> + Debug + CanFormatToString + CanParseFromString
{
}
// Blanket implementation of check trait ensures the compiler enforces all checks.
impl CanUsePerson for Person {}

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

fn main() {
    let person = Person {
        first_name: "John".into(),
        last_name: "Smith".into(),
    };
    // Because Person implements HasComponents, we can now call person.format_to_string()
    let person_str = r#"{"first_name":"John","last_name":"Smith"}"#;

    assert_eq!(person.format_to_string().unwrap(), person_str);

    assert_eq!(Person::parse_from_string(person_str).unwrap(), person);
}
