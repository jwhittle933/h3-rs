## h3-rs
`h3-rs` is a WIP Rust port of Uber's libh3. For a write up on the motivation and creation of the system, you can read their [engineering article](https://eng.uber.com/h3/).

### Motivation
There are currently 3 Rust libraries for Uber's H3. All 3 use `bindgen`.
- [libh3-sys](https://github.com/rustyconover/libh3-sys)
- [libh3](https://github.com/rustyconover/libh3)
- [h3ron](https://github.com/nmandery/h3ron)

While C bindings work fine, a native Rust solution would be the optimal choice.

### Strategy
The first phase is to get the core functionality of H3 ported to Rust with minimal changes to the system. This includes source code, tests, and the binaries. There will be natural updates that can take place along the way, however. Many of the C functions that accept pointers can be converted to `impl` blocks that return values, and the conversion functions can be moved to `Into` and `TryInto` implementations.

The second phase will be refactoring the code that still bears the marks of its parent into idiomatic Rust. Tests and binaries will be updated as well.