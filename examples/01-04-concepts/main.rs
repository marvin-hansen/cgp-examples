// Context
// https://patterns.contextgeneric.dev/context.html

// In CGP, we use the term context to refer to a type that provide certain functionalities,
//  or dependencies. The most common kind of functionality a context may provide is a method.

struct MyContext {}

impl MyContext {
    fn hello(&self) {
        println!("Hello World!");
    }
}

// Consumer
// https://patterns.contextgeneric.dev/consumer.html#consumer

// In CGP, a consumer is a piece of code that consumes certain functionalities from a context.
// There are several ways which a consumer may consume a functionality.
// At its most basic, if a consumer has access to the concrete type of a context,
// it can access any methods defined by an impl block of that context.

struct Person {
    name: String,
}

impl Person {
    fn name(&self) -> &str {
        &self.name
    }
}

#[allow(unused)]
fn greet(person: &Person) {
    println!("Hello, {}!", person.name());
}

// In the above example, we have a greet function that prints a greeting to a person
// using the method Person::name. In other words, we say that
// the greet function is a consumer to the Person::name method.

// Context-Generic Consumers
// The greet function in our previous example can only work with the Person struct.
// However, if we inspect the implementation of greet, we can see that it is possible to generalize
// greet to work with any type that has a name.
//
// To generalize greet, we first need to define a trait that acts as an interface for getting a name:
#[allow(unused)]
trait HasName {
    fn name(&self) -> &str;
}

//Provider
// https://patterns.contextgeneric.dev/provider.html

// In CGP, a provider is a piece of code that implements certain functionality for a context.
// At its most basic, a provider is consist of an impl block for a trait.

impl HasName for Person {
    fn name(&self) -> &str {
        &self.name
    }
}

// In the above example, we implement the HasName for the Person struct.
// The block impl HasName for Person is a provider of the HasName trait for the Person context.

// For this example, the impl block is a context-specific provider for the Person context.
// Furthermore, due to the restrictions of Rust's trait system,
// there can be at most one provider of HasName for the Person context.
// Another common restriction is that the provider has to be defined in the same crate as either the trait or the context.
//
// The asymetry between what can be done with a provider, as compared to a consumer,
// is often a source of complexity in many Rust programs. As we will learn in later chapters,
// one of the goals of CGP is to break this asymetry, and make it easy to implement context-generic providers.

// Providers as Consumers
// Although we have providers and consumers as distinct concepts,
// it is common to have code that serve as both providers and consumers.

trait CanGreet {
    fn greet(&self);
}

impl CanGreet for Person {
    fn greet(&self) {
        println!("Hello, {}!", self.name());
    }
}

fn main() {
    // We can then use the hello method anywhere that we have a value of type MyContext;
    // Notice, that have bound the method hello to the context MyContext, thus making it context-specific.
    let my_context = MyContext {};
    my_context.hello();

    //  However, the implementation below is context-specific to the Person context, and cannot be reused for other contexts.
    let person = Person {
        name: "Alice".to_owned(),
    };
    person.greet();

    // The next example shows how to define context-generic implementations of Greet that works with any context type
}
