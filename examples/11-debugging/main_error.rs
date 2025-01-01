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

mod dep_error;
mod string_formatter_comp;
mod string_parser_comp;

fn main() {
    dep_error::test_dep_error();
}
