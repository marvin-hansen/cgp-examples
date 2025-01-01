use crate::aggregate_provider::{FormatAsJsonString, ParseFromJsonString};
use crate::shared::{CanFormatToString, CanParseFromString, Person, StringFormatter, StringParser};
use anyhow::Error;
use serde::Serialize;
// Blanket Provider Implementation

// Although the previous example works, the boilerplate for forwarding multiple implementations
// by PersonComponents seems a bit tedious and redundant. The main differences between the two implementation b
// oilerplate is that we want to choose FormatAsJsonString as the provider for StringFormatter,
// and ParseFromJsonString as the provider for StringParser.

// Similar to how we can use HasComponents with blanket implementations to link a consumer with a provider,
// we can reduce the boilerplate required by using similar pattern to link a provider with another provider:

pub trait DelegateComponent<Name> {
    type Delegate;
}

pub struct StringFormatterComponent;

pub struct StringParserComponent;

impl<Context, Component> StringFormatter<Context> for Component
where
    Component: DelegateComponent<StringFormatterComponent>,
    Component::Delegate: StringFormatter<Context>,
{
    fn format_to_string(context: &Context) -> Result<String, Error> {
        Component::Delegate::format_to_string(context)
    }
}

impl<Context, Component> StringParser<Context> for Component
where
    Component: DelegateComponent<StringParserComponent>,
    Component::Delegate: StringParser<Context>,
{
    fn parse_from_string(raw: &str) -> Result<Context, Error> {
        Component::Delegate::parse_from_string(raw)
    }
}

// The DelegateComponent is similar to the HasComponents trait, but it is intended to be implemented
// by providers instead of concrete contexts. It also has an extra generic Name type that is used
// to differentiate which component the provider delegation is intended for.

// To make use of the Name parameter, we first need to assign names to the CGP components that we have defined.
// We first define the dummy struct StringFormatterComponent to be used as the name for StringFormatter,
// and StringParserComponent to be used as the name for StringParser.
// In general, we can choose any type as the component name. However by convention, we choose to add a
// -Component postfix to the name of the provider trait to be used as the name of the component.

// We then define a blanket implementation for StringFormatter, which is implemented for
// a provider type Component with the following conditions: if the provider implements
// DelegateComponent<StringFormatterComponent>, and if the associated type Delegate also implements StringFormatter<Context>,
// then Component also implements StringFormatter<Context> by delegating the implementation to Delegate.

// Following the same pattern, we also define a blanket implementation for StringParser.
// The main difference here is that the name StringParserComponent is used as the type argument to DelegateComponent.
//  In other words, different blanket provider implementations make use of different Name types for DelegateComponent,
// allowing different Delegate to be used depending on the Name.

// Using DelegateComponent

// It may take a while to fully understand how the blanket implementations with DelegateComponent and HasComponents work.
// But since the same pattern will be used everywhere, it would hopefully become clear as we see more examples.
// It would also help to see how the blanket implementation is used,
// by going back to the example of implementing the concrete context Person.

pub struct PersonComponents;

impl DelegateComponent<StringFormatterComponent> for PersonComponents {
    type Delegate = FormatAsJsonString;
}

impl DelegateComponent<StringParserComponent> for PersonComponents {
    type Delegate = ParseFromJsonString;
}

// Instead of implementing the provider traits, we now only need to implement
// DelegateComponent<StringFormatterComponent> and DelegateComponent<StringParserComponent>
// for PersonComponents. Rust's trait system would then automatically make use of the blanket implementations
// to implement CanFormatToString and CanParseFromString for Person.
//
// As we will see in the next chapter, we can make use of macros to further simplify the component delegation,
//  making it as simple as one line to implement such delegation.

// Switching Provider Implementations

// With the given examples, some readers may question why is there a need to define multiple providers for the JSON implementation,
// when we can just define one provider struct and implement both provider traits for it.

// The use of two providers in this chapter is mainly used as demonstration on how to delegate
// and combine multiple providers. In practice, as the number of CGP components increase,
// we would also quickly run into the need have multiple provider implementations
// and choosing between different combination of providers.

// Even with the simplified example here, we can demonstrate how a different provider for StringFormatter may be needed.
// Supposed that we want to format the context as prettified JSON string, we can define a separate provider
// FormatAsPrettifiedJsonString as follows:

pub struct FormatAsPrettifiedJsonString;

impl<Context> StringFormatter<Context> for FormatAsPrettifiedJsonString
where
    Context: Serialize,
{
    fn format_to_string(context: &Context) -> Result<String, Error> {
        Ok(serde_json::to_string_pretty(context)?)
    }
}

// Then, the only change we made would be to replace FormatAsJsonString with FormatAsPrettifiedJsonString
// in the delegate:

// impl DelegateComponent<StringParserComponent> for StringParserComponent {
//     type Delegate = ParseFromJsonString;
// }

// This code is uncommented because it would raise an conflicting implementation error.

// Compared to before, the only line change is to set the Delegate of DelegateComponent<StringFormatterComponent> to FormatAsPrettifiedJsonString instead of FormatAsJsonString. With that, we can now easily choose between whether to pretty print a Person context as JSON.
//
// Beyond having a prettified implementation, it is also easy to think of other kinds of generic implementations,
// such as using Debug or Display to format strings, or use different encodings such as XML to format the string.
// With CGP, we can define generalized component interfaces that are applicable to a wide range of implementations.
// We can then make use of DelegateComponent to easily choose which implementation we want to use for different concrete contexts.

pub(crate) fn test_delegate_provider() {
    let person = Person {
        first_name: "John".into(),
        last_name: "Smith".into(),
    };
    let person_str = r#"{"first_name":"John","last_name":"Smith"}"#;

    assert_eq!(person.format_to_string().unwrap(), person_str);

    assert_eq!(Person::parse_from_string(person_str).unwrap(), person);
}
