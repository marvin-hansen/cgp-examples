mod string_formatter_comp;
mod string_parser_comp;

use crate::string_formatter_comp::{
    CanFormatToString, FormatAsJsonString, StringFormatterComponent,
};
use crate::string_parser_comp::{CanParseFromString, ParseFromJsonString, StringParserComponent};
use cgp::prelude::*;
use serde::{Deserialize, Serialize};

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

// Note, even though each component resides in a separate file,
// in practice you may want to move larger components into a single crate.

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
