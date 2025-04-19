# Freenet Scaffold

`freenet-scaffold` is a Rust utility crate that simplifies the development of Freenet contracts by
providing tools to implement efficient, mergeable state synchronization.

## Overview

In decentralized systems like Freenet, achieving consistency across nodes without heavy coordination
is challenging. Traditional methods often rely on consensus algorithms, which can be
resource-intensive and less scalable. Freenet addresses this by using a summary-delta
synchronization approach, where each node summarizes its state and exchanges deltas—minimal changes
needed to update another node's state. This method reduces bandwidth usage and allows for efficient,
deterministic merging of states across the network.

The `freenet-scaffold` crate provides the `ComposableState` trait and associated utilities to
implement this approach in Rust. It enables developers to define how their data structures can be
summarized, how deltas are computed, and how to apply these deltas to achieve eventual consistency.

## Getting Started

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
freenet-scaffold = "0.2"
freenet-scaffold-macro = "0.2"
```

The `freenet-scaffold-macro` crate provides the `#[composable]` procedural macro, which derives
implementations of the `ComposableState` trait for structs whose fields also implement
`ComposableState`. This macro re-exports everything from `freenet-scaffold`, so importing
`freenet-scaffold-macro` is typically sufficient.

### Example

Here's a minimal example demonstrating how to use `freenet-scaffold` and `freenet-scaffold-macro`:

```rust
use freenet_scaffold::{ComposableState, util::FastHash};
use freenet_scaffold_macro::composable;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ContractualI32(pub i32);

impl ComposableState for ContractualI32 {
    type ParentState = Test;
    type Summary = i32;
    type Delta = i32;
    type Parameters = Params;

    fn verify(&self, _: &Self::ParentState, _: &Self::Parameters) -> Result<(), String> { Ok(()) }
    fn summarize(&self, _: &Self::ParentState, _: &Self::Parameters) -> Self::Summary { self.0 }
    fn delta(&self, _: &Self::ParentState, _: &Self::Parameters, old: &Self::Summary) -> Option<Self::Delta> {
        let diff = self.0 - *old; if diff == 0 { None } else { Some(diff) }
    }
    fn apply_delta(&mut self, _: &Self::ParentState, _: &Self::Parameters, delta: &Option<Self::Delta>) -> Result<(), String> {
        if let Some(d) = delta { self.0 += *d; } Ok(())
    }
}

#[composable]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Test {
    number: ContractualI32,
    // Add other fields implementing ComposableState
}
```

The `#[composable]` macro automatically generates the necessary summary and delta structures, as
well as the `ComposableState` implementation for the `Test` struct.

## Best Practices

- **Field Order Matters**: When using `#[composable]`, ensure that fields are ordered such that any
  field depending on another appears after it. This is important for cases where one field's
  `apply_delta` relies on another field's state.

## License

This project is licensed under the **GNU Lesser General Public License v2.1** (LGPL-2.1-only).

## Contributing

Pull requests are welcome.

---

For a deeper understanding of the summary-delta synchronization approach and its advantages in
decentralized systems, refer to the article:
[Understanding Freenet's Delta-Sync](https://freenet.org/news/summary-delta-sync/).
