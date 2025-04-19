# Freenet Scaffold

`freenet-scaffold` is a lightweight utility crate that underpins Freenet contract development. It
exposes the [`ComposableState`
trait]\(https\://github.com/freenet/freenet-scaffold/blob/main/src/lib.rs) and a handful of helpers (e.g.,
a fast non‑cryptographic hash).

The companion crate **`freenet-scaffold-macro`** exposes the `#[composable]` procedural macro that
derives a fully‑featured `ComposableState` implementation for a struct whose fields themselves
implement `ComposableState`. Users of the macro only need to depend on `freenet-scaffold-macro`; it
re‑exports everything from this crate, so a single `use freenet_scaffold::*;` is usually enough.

---

## Why does this exist?

Freenet contracts run on an eventually‑consistent, peer‑to‑peer network. A contract’s state can be
modified concurrently on different peers, then merged. Replacing the entire object each time wastes
bandwidth and makes merges harder.\
`ComposableState` forces each component to describe itself in three sizes:

- **Summary** – a small, hash‑like view that answers “has this part changed?”
- **Delta** – the minimal information needed to turn one state into the next.
- **Full state** – the complete object you would have written by hand.

By composing these pieces recursively, large structures can be updated with tiny messages, and
conflicting edits can be merged deterministically.

For a broader, non‑code overview of why this _summary + delta_ approach scales so well on a
small‑world peer‑to‑peer network, see the project blog post:
[“Understanding Freenet’s Delta‑Sync”](https://freenet.org/news/summary-delta-sync/).

---

## Installing

```toml
# Cargo.toml
[dependencies]
freenet-scaffold = "0.2"
freenet-scaffold-macro = "0.2" # only if you use the derive macro
```

_Both crates are ************\*\*************`no_std`************\*\************* with the
************\*\*************`alloc`************\*\************* feature;
************\*\*************`std`************\*\************* is enabled by default._

---

## Quick start

Below is a minimal, self‑contained example. It shows two domain‑specific structs (`ContractualI32`,
`ContractualString`) that implement `ComposableState` manually, then a composite struct that
delegates the heavy lifting to `#[composable]`.

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

// Repeat for ContractualString …

#[composable]                 // <<< this line does all the work
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Test {
    number: ContractualI32,
    text:   ContractualString,
}
```

The macro expands `Test` into roughly the following:

- A `TestSummary` struct containing the summaries of each field.
- A `TestDelta` struct with optional deltas for each field.
- A `ComposableState` impl that:
  - Delegates `verify`, `summarize`, `delta`, and `apply_delta` down into each field.
  - Generates `None` when no field changed so that upstream code can avoid needless work.
  - Performs compile‑time checks that all fields share the same `ParentState` and `Parameters`
    types.

### Things to remember

- **Field order matters** when using `#[composable]`. If field `B` refers to field `A` inside its
  `apply_delta`, then `A` must appear before `B` in the struct. This is common when one component
  depends on a configuration component that lives earlier in the struct.
- If a field’s delta is `None`, its `apply_delta` will still be called. Use this to react to changes
  in the parent or other fields.

### Complete example in River

For a real‑world contract that exercises `ComposableState` and `#[composable]` with several
inter‑dependent fields, take a look at the River chat application’s state definition:
[`room_state.rs`](https://github.com/freenet/river/blob/main/common/src/room_state.rs). It
demonstrates validation logic, owner/member permissions, and how deltas cascade through nested
components.

---

## Advanced usage

### Implementing your own leaf component

A leaf component is a type that implements `ComposableState` directly instead of via the macro. You
will usually need to do this for primitive values or cryptographic objects.

1. Choose the smallest data you can get away with for `Summary`.
2. Make `Delta` as small as possible. A single bit flag or an enum variant is fine.
3. Write `verify` defensively (check signatures, bounds, invariants).
4. Keep `apply_delta` idempotent and deterministic.

### Fast hashing helper

`util::fast_hash` is an extremely cheap, 64‑bit hash. It is **not** cryptographically secure. Use it
only for lookup keys or change detection where collisions have no security impact. When you need a
secure hash, pull in a real hash function.

---

## Testing pattern

The crate ships with extensive tests that demonstrate a pattern you can copy:

1. Build two versions of the state (`old`, `new`).
2. Call `verify` on both.
3. Produce a delta with `new.delta(&old, …)`.
4. Clone `old`, apply the delta, then assert equality with `new`.

This confirms that your delta logic is lossless and that `apply_delta` maintains invariants.

---

## Relationship between the two crates

| Crate                    | Contains                         | Cargo feature gate |
| ------------------------ | -------------------------------- | ------------------ |
| `freenet-scaffold`       | `ComposableState` trait, helpers | `std` (default)    |
| `freenet-scaffold-macro` | `#[composable]` derive macro     | —                  |

`freenet-scaffold-macro` depends on `freenet-scaffold` and re‑exports everything, so most downstream
crates only need to depend on the macro crate.

---

## License

This project is licensed under the **GNU Lesser General Public License v2.1** (LGPL‑2.1‑only). See
the `LICENSE` file for the full text.

---

## Contributing

Issues and pull requests are welcome. Please run `cargo test --all` before submitting a PR.
