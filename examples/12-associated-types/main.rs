
// Associated Types
// https://patterns.contextgeneric.dev/associated-types.html

// In the first part of this book, we have learned about how CGP makes use of Rust's trait system
// to wire up components using blanket implementations. Since CGP works within Rust's trait system,
// we can make use of advanced Rust features together with CGP to form new design patterns.

// In this chapter, we will learn about how to make use of associated types with CGP
// to define context-generic providers that are generic over multiple abstract types.

// Building Authentication Components

// Supposed that we want to build a simple authentication system using bearer tokens with expiry time.
// To build such system, we would need to fetch the expiry time of a valid token,
// and ensure that the time is not in the past. A naive attempt of implementing
// the authentication would be as follows:

fn main() {}
