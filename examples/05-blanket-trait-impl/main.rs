// In the previous chapter, we have an implementation of CanGreet for Person that makes use of
// HasName to retrieve the person's name to be printed. However, the implementation is context-specific
// to the Person context, and cannot be reused for other contexts.

// we want to be able to define context-generic implementations of Greet that works with any context type
// that also implements HasName. For this, the blanket trait implementations pattern is one basic way
// which we can use for defining context-generic implementations

trait HasName {
    fn name(&self) -> &str;
}

trait CanGreet {
    fn greet(&self);
}

impl<Context> CanGreet for Context
where
    Context: HasName,
{
    fn greet(&self) {
        println!("Hello, {}!", self.name());
    }
}

// The above example shows a blanket trait implementation of CanGreet for any Context type that implements HasName.
// With that, contexts like Person do not need to explicitly implement CanGreet, if they already implement HasName:

struct Person {
    name: String,
}

impl HasName for Person {
    fn name(&self) -> &str {
        &self.name
    }
}

// Extension Traits
//
// The use of blanket trait implementation is commonly found in many Rust libraries today.
//  For example, Itertools provides a blanket implementation for any context that implements Iterator.
// Another example is StreamExt, which is implemented for any context that implements Stream.
//
// Traits such as Itertools and StreamExt are sometimes known as extension traits.
// This is because the purpose of the trait is to extend the behavior of existing types,
//  without having to own the type or base traits.

// Overriding Blanket Implementations
//
// Extension Traits containing blanket implementation are usually not meant to be implemented
// manually by individual contexts. They are usually meant to serve as convenient methods
// that extends the functionality of another trait.

// However, Rust's trait system does not completely prevent us from overriding the blanket implementation.
//
// Supposed that we have a VipPerson context that we want to implement a different way of greeting the VIP person.
// We could override the implementation as follows:

struct VipPerson {
    name: String, /* other fields */
}

impl CanGreet for VipPerson {
    fn greet(&self) {
        println!("A warm welcome to you, {}!", self.name);
    }
}

// Conflicting Implementations
// In the previous example, we are able to define a custom provider for VipPerson,
// but with an important caveat: that VipPerson does not implement HasName.
//
//   If we try to define a custom provider for contexts that already implement HasName,
// such as for Person, the compilation would fail:

// impl CanGreet for Person {
//     fn greet(&self) {
//         println!("Hi, {}!", self.name());
//     }
// }
// ^^^^ conflicting implementation for `Person`

// The reason for the conflict is because Rust trait system requires all types
// to have unambiguous implementation of any given trait.
// f Rust were to allow ambiguous override of blanket implementations, such as what we tried with Person,
// it would have resulted in inconsistencies in the compiled code,

fn main() {
    // we call person.greet() without having a context-specific implementation of CanGreet for Person.
    let person = Person {
        name: "Alice - Context Generic".to_owned(),
    };
    person.greet();

    // Here we have a context-specific implementation of CanGreet for VipPerson attached to the VipPerson context
    let vip_person = VipPerson {
        name: "Alice - VIP".to_owned(),
    };
    vip_person.greet();
}

// Limitations of Blanket Implementations
//
// Due to potential conflicting implementations, the use of blanket implementations offer limited customizability,
// in case if a context wants to have a different implementation.
// Although a context many define its own context-specific provider to override the blanket provider, it would face other limitations such as not being able to implement other traits that may cause a conflict.
//
// In practice, we consider that blanket implementations allow for a singular context-generic provider to be defined.
// In future chapters, we will look at how to relax the singular constraint,
// to make it possible to allow multiple context-generic or context-specific providers to co-exist.
