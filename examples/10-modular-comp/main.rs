// Single import of person component and related traits
use crate::person_component::{Person, CanParseFromString, CanFormatToString};

mod string_formatter_comp;
mod string_parser_comp;
mod person_component;

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