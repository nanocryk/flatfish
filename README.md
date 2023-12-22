# Flatfish

[![flatfish crate](https://img.shields.io/crates/v/flatfish.svg)](https://crates.io/crates/flatfish)
[![flatfish documentation](https://docs.rs/flatfish/badge.svg)](https://docs.rs/flatfish)

Provide a `ff!` macro to write Fully Qualified Syntax without nesting
turbofishes.

Which can be usefull in very generic code when the chain of traits and
associated types can be long and verbose, or in macros generating chains of
traits.

Syntax is

```rust,ignore
ff!(Type | Trait1::Item | Trait2::Item ...)
```

which desugars to

```rust,ignore
<... <<T as Trait1>::Item as Trait2>::Item ...>
```

Last item can be any associated item: type, function or constant.