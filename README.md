# static-algebra
N-dimensional, arbitrary-type linear algebra library for stable Rust.

## Design goals

This crate aims to provide the most generic possible form of vector and matrix structures, supporting the broadest types that operations allow and an arbitrary number of dimensions. It also aspires to the maximum possible level of static analysis by the stable Rust compiler on vector and matrix operations. This is implemented using template metaprogramming to replace runtime `usize` bounds and indices with a new integer arithmetic built from pure types, enabling the compiler to reason recursively about sequence data structures.

In theory, this means a high level of static optimization, but this crate is not yet developed enough to be properly benchmarked.

`static-algebra` is `no_std` and uses no `unsafe`, panic, macros, nightly features, or required dependencies.

#### Goals
- Vector operations
- Matrix operations
- Broadly generic type and dimension support
- Flexible vector and matrix views for efficient operations, at least as capable as `nalgebra`'s views
- Practical user-facing APIs for common use cases, i.e. graphics and game engines
- Maximum release build performance

#### Non-goals
- Dynamically-sized structures
- Low compile time / binary size
- Intuitive compiler error messages
- Debug build performance

### No `unsafe`

`#![deny(unsafe_code)]` is declared in `lib.rs`.

### No panics

The crate does not use any operations that could panic, including no `unwrap`s and no index-out-of-bounds panics. Optionals are used at runtime whenever an index operation could potentially be out-of-bounds.

### No macros

No macros are used in the crate.

### No nightly

The crate supports stable Rust 2024 edition.

### `no_std`

No dependence on `std` library.

### No required dependencies

Default features include the `num-traits` crate to define mathematical properties for provided data structures. This can be disabled to remove the depdency by specifying `default-features = false` on this crate in your `Cargo.toml`.

## Similar crates

This crate shares features and goals with a few existing crates:
- `nalgebra`: Good crate. Still need to look into what this crate could do that `nalgebra` can't, besides indexing into structures with compiler checks.
- `optimath`: Requires nightly Rust features, uses `unsafe` liberally, only implements operations for `&Vector`, unmaintained since 2021.
