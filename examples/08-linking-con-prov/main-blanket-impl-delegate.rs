// Blanket Consumer Trait Implementation
//
// In the previous section, we manually implemented CanFormatString for Person with an explicit call
// to FormatStringWithDisplay. Although the implementation is relatively short, it can become tedious
// if we make heavy use of provider traits, which would require us to repeat the same pattern for every trait.

pub struct FormatStringWithDisplay;

impl<Context> StringFormatter<Context> for FormatStringWithDisplay
where
    Context: Display,
{
    fn format_string(context: &Context) -> String {
        format!("{}", context)
    }
}

// To simplify this further, we can make use of blanket implementations to automatically delegate
// the implementation of all consumer traits to one chosen provider.
// We would define the blanket implementation for CanFormatString as follows:

pub trait HasComponents {
    type Components;
}

pub trait CanFormatString {
    fn format_string(&self) -> String;
}

pub trait StringFormatter<Context> {
    fn format_string(context: &Context) -> String;
}

impl<Context> CanFormatString for Context
where
    Context: HasComponents,
    Context::Components: StringFormatter<Context>,
{
    fn format_string(&self) -> String {
        // Delegate to the associated type
        Context::Components::format_string(self)
    }
}

// First of all, we define a new HasComponents trait that contains an associated type Components.
// The Components type would be specified by a context to choose a provider that it would use
// to forward all implementations of consumer traits. Following that, we add a blanket implementation
// for CanFormatString, which would be implemented for any Context that implements HasComponents,
// provided that Context::Components implements StringFormatter<Context>.
//
// To explain in simpler terms - if a context has a provider that implements a provider trait for that context,
// then the consumer trait for that context is also automatically implemented.
//
// With the new blanket implementation in place, we can now implement HasComponents for the Person context,
// and it would now help us to implement CanFormatString for free:

use core::fmt::{self, Debug, Display};

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

impl HasComponents for Person {
    // Specialize associated type into concrete type that provides the format string functionality.
    type Components = FormatStringWithDisplay;
}

// Compared to before, the implementation of HasComponents is much shorter
// than implementing CanFormatString directly, since we only need to specify the provider type without
// any function definition.

// At the moment, because the Person context only implements one consumer trait,
// we can set FormatStringWithDisplay directly as Person::Components.
// However, if there are other consumer traits that we would like to use with Person,
// we would need to define Person::Components with a separate provider that implements multiple provider traits.
// This will be covered in the next chapter, which we would talk about how to
// link multiple providers of different provider traits together.

fn main() {
    let person = Person {
        first_name: "John".into(),
        last_name: "Smith".into(),
    };
    // Because Person implements HasComponents, we can now call person.format_string()
    assert_eq!(person.format_string(), "John Smith");
}

// Component System

// You may have noticed that the trait for specifying the provider for a context is called HasComponents
// instead of HasProviders. This is to generalize the idea of a pair of consumer trait and provider trait working together,
// forming a component.

// In context-generic programming, we use the term component to refer to a consumer-provider trait pair.
// The consumer trait and the provider trait are linked together through blanket implementations
// and traits such as HasComponents. These constructs working together to form the basis for a component system for CGP.
