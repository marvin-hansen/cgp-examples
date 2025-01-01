use crate::shared::{
    CanFormatToString, CanParseFromString, HasComponents, Person, StringFormatter, StringParser,
};
use anyhow::Error;
use serde::{Deserialize, Serialize};

// In the previous chapter, we learned to make use of the HasComponents trait to define a blanket implementation
// for a consumer trait like CanFormatString, so that a context would automatically delegate
// the implementation to a provider trait like StringFormatter.

// Similar to the previous chapter, we define CanFormatToString for formatting a context into string,
// and CanParseFromString for parsing a context from a string. Notice that CanParseFromString
// also has an additional Sized constraint, as by default the Self type in Rust traits do not implement Sized,
// to allow traits to be used in dyn trait objects

// Next, we also define the provider traits as follows:

impl<Context> CanFormatToString for Context
where
    Context: HasComponents,
    Context::Components: StringFormatter<Context>,
{
    fn format_to_string(&self) -> Result<String, Error> {
        Context::Components::format_to_string(self)
    }
}

impl<Context> CanParseFromString for Context
where
    Context: HasComponents,
    Context::Components: StringParser<Context>,
{
    fn parse_from_string(raw: &str) -> Result<Context, Error> {
        Context::Components::parse_from_string(raw)
    }
}

// Similar to the previous chapter, we make use of blanket implementations and HasComponents
// to link the consumer traits CanFormatToString and CanParseFromString with their respective provider traits,
// StringFormatter and StringParser.
//
// We can then implement context-generic providers for the given provider traits,
// such as to format and parse the context as JSON if the context implements Serialize and Deserialize:

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

// The provider FormatAsJsonString implements StringFormatter for any Context type that implements Serialize,
// and uses serde_json::to_string to format the context as JSON.
// Similarly, the provider ParseFromJsonString implements StringParser for any Context that implements Deserialize,
// and parse the context from a JSON string.

// Linking Multiple Providers to a Concrete Context

// With the providers implemented, we can now define a concrete context like Person, and link it with the given providers.
// However, since there are multiple providers, we need to first define an aggregated provider called PersonComponents,
// which would implement both StringFormatter and StringParser by delegating the call to the actual providers.

// aggregated provider
pub struct PersonComponents;

// Set the associated type to PersonComponents that aggregates all other providers
impl HasComponents for Person {
    type Components = PersonComponents;
}

// Select implementation for StringFormatter and StringParser
impl StringFormatter<Person> for PersonComponents {
    fn format_to_string(context: &Person) -> Result<String, Error> {
        FormatAsJsonString::format_to_string(context)
    }
}

impl StringParser<Person> for PersonComponents {
    fn parse_from_string(raw: &str) -> Result<Person, Error> {
        ParseFromJsonString::parse_from_string(raw)
    }
}

// We first define Person struct with auto-derived implementations of Serialize and Deserialize.
// We also auto-derive Debug and Eq for use in tests later on.

// We then define a dummy struct PersonComponents, which would be used to aggregate the providers for Person.
// Compared to the previous chapter, we implement HasComponents for Person with PersonComponents as the provider.

// We then implement the provider traits StringFormatter and StringParser for PersonComponents,
// with the actual implementation forwarded to FormatAsJsonString and ParseFromJsonString.

pub(crate) fn test_aggregated_provider() {
    let person = Person {
        first_name: "John".into(),
        last_name: "Smith".into(),
    };
    let person_str = r#"{"first_name":"John","last_name":"Smith"}"#;

    assert_eq!(person.format_to_string().unwrap(), person_str);

    assert_eq!(Person::parse_from_string(person_str).unwrap(), person);
}

// Inside the test that follows, we verify that the wiring indeed automatically implements CanFormatToString and CanParseFromString
// for Person, with the JSON implementation used.
