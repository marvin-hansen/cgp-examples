// Provider Delegation
// https://patterns.contextgeneric.dev/provider-delegation.html

mod aggregate_provider;
mod delgate_provider;

mod shared;

fn main() {
    aggregate_provider::test_aggregated_provider();

    delgate_provider::test_delegate_provider();
}
