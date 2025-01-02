// Debugging Techniques
// https://patterns.contextgeneric.dev/debugging-techniques.html

// By leveraging impl-side dependencies, CGP providers are able to include additional dependencies
// that are not specified in the provider trait. We have already seen this in action in the previous chapter,
// for example, where the provider FormatAsJsonString is able to require Context to implement Serialize,
//  while that is not specified anywhere in the provider trait StringFormatter.

// In fact, because the provider constraints are not enforced in DelegateComponent,
//  the delegation would always be successful, even if some provider constraints are not satisfied.
// In other words, the impl-side provider constraints are enforced lazily in CGP,
// and compile-time errors would only arise when we try to use a consumer trait against a concrete context.

mod string_formatter_comp;
mod string_parser_comp;

use crate::string_formatter_comp::{
    CanFormatToString, FormatAsJsonString, StringFormatterComponent,
};
use crate::string_parser_comp::{CanParseFromString, ParseFromJsonString, StringParserComponent};
use cgp::prelude::*;
use serde::{Deserialize, Serialize};
// Unsatisfied Dependency Errors
//
// To demonstrate how such error would arise, we would reuse the same example Person context
// as the previous chapter. Consider if we made a mistake and forgot to implement Serialize for Person:

// Note: We forgot to derive Serialize here. The code compiles with cargo build,
// but fails to run due to lazy evaluation of the trait bounds.
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

pub struct PersonComponents;

impl HasComponents for Person {
    type Components = PersonComponents;
}

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
