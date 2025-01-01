// Linking Consumers with Providers
// https://patterns.contextgeneric.dev/consumer-provider-link.html

// In the previous chapter, we learned about how provider traits allow multiple overlapping implementations to be defined.
// However, if everything is implemented only as provider traits, it would be much more tedious
// having to determine which provider to use, at every time when we need to use the trait.

// To overcome this, we would need have both provider traits and consumer traits,
// and have some ways to choose a provider when implementing a consumer trait.

//  Implementing Consumer Traits

mod string_formatter;

// The simplest way to link a consumer trait with a provider is by
// implementing the consumer trait to call a chosen provider. Consider the StringFormatter example
// of the previous chapter, we would implement CanFormatString for a Person context as follows:
use crate::string_formatter::{FormatStringWithDisplay, StringFormatter};
use core::fmt::{self, Display};

// consumer trait
pub trait CanFormatString {
    fn format_string(&self) -> String;
}

#[derive(Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

//  implement CanFormatString for Person by calling FormatStringWithDisplay
impl CanFormatString for Person {
    fn format_string(&self) -> String {
        FormatStringWithDisplay::format_string(self)
        // Alternatively, we can call
        // FormatStringWithDebug::format_string(self)
    }
}

fn test_format_string_consumer() {
    let person = Person {
        first_name: "John".into(),
        last_name: "Smith".into(),
    };

    assert_eq!(person.format_string(), "John Smith");
}

// To recap the previous chapter, we have a consumer trait CanFormatString and
// a provider trait StringFormatter. There are two example providers that implement StringFormatter
// - FormatStringWithDisplay which formats strings using Display,
// and FormatStringWithDebug which formats strings using Debug.

// In addition to that, we implement CanFormatString for the Person context
// by forwarding the call to FormatStringWithDisplay.

// By doing so, we effectively "bind" the StringFormatter provider for the Person context to FormatStringWithDisplay.
// With that, any time a consumer code calls person.format_string(),
// it would automatically format the context using Display.

// Thanks to the decoupling of providers and consumers, a context like Person can freely choose between multiple providers,
// and link them with relative ease. Similarly, the provider trait allows multiple context-generic providers
// such as FormatStringWithDisplay and FormatStringWithDebug to co-exist.

// Blanket Consumer Trait Implementation

fn main() {
    test_format_string_consumer();
}
