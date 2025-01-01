// Component Macros
// https://patterns.contextgeneric.dev/component-macros.html

// At this point, we have covered all basic building blocks of defining CGP components.
// In summary, a CGP component is consist of the following building blocks:

// * A consumer trait.
// * A provider trait.
// * A component name type.
// * blanket implementation of the consumer trait using HasComponents.
// * blanket implementation of the provider trait using DelegateComponent.

// cgp_component Macro

// With the repetitive pattern, it makes sense that we should be able to just define the consumer trait,
// and make use of Rust macros to generate the remaining code.

// Example Use
// To illustrate how cgp_component and delegate_components can be used,
// we revisit the code for CanFormatToString, CanParseFromString, and PersonContext from the previous chapter,
// and look at how the macros can simplify the same code.
//
// Following is the full code after simplification using cgp:
use anyhow::Error;
use cgp::prelude::*;
use serde::{Deserialize, Serialize};

// Component definitions
#[cgp_component {
    name: StringFormatterComponent,
    provider: StringFormatter,
    context: Context,
    }]
pub trait CanFormatToString {
    fn format_to_string(&self) -> Result<String, Error>;
}

#[cgp_component {
    name: StringParserComponent,
    provider: StringParser,
    context: Context,
    }]
pub trait CanParseFromString: Sized {
    fn parse_from_string(raw: &str) -> Result<Self, Error>;
}

// Provider implementations
pub struct FormatAsJsonString;

impl<Context> StringFormatter<Context> for FormatAsJsonString
where
    Context: Serialize,
{
    fn format_to_string(context: &Context) -> Result<String, Error> {
        Ok(serde_json::to_string(context)?)
    }
}

pub struct ParseFromJsonString;

impl<Context> StringParser<Context> for ParseFromJsonString
where
    Context: for<'a> Deserialize<'a>,
{
    fn parse_from_string(json_str: &str) -> Result<Context, Error> {
        Ok(serde_json::from_str(json_str)?)
    }
}

// Concrete context and wiring

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
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

// As we can see, the new code is significantly simpler and more readable than before.
// Using cgp_component, we no longer need to explicitly define the provider traits StringFormatter and StringParser,
// and the blanket implementations can be omitted. We also make use of delegate_components! on PersonComponents
// to delegate StringFormatterComponent to FormatAsJsonString, and StringParserComponent to ParseFromJsonString.

fn test_cgp_macro() {
    let person = Person {
        first_name: "John".into(),
        last_name: "Smith".into(),
    };
    let person_str = r#"{"first_name":"John","last_name":"Smith"}"#;

    assert_eq!(person.format_to_string().unwrap(), person_str);

    assert_eq!(Person::parse_from_string(person_str).unwrap(), person);
}

fn main() {
    test_cgp_macro();
}
