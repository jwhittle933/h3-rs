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

## Design
There are many things that will diverge in the Rust code from C.

Many of the C functions come in pairs to operate on the variants within the types, i.e., `_faceIjkToCellBoundary` and  `_faceIjkPentToCellBoundary`. Both of these methods can be an implementation of the `FaceIJK<T>`, where `T` is either `Hexagon` or `Pentagon`. For example,
```rust
struct Hexagon;
struct Pentagon;
pub struct FaceIJK<T>{
    _m: PhantomData<T>
}

impl Into<CellBoundary> for FaceIJK<Hexagon> {
    fn into(&self) -> CellBoundary { CellBoundary{} }
}

impl Into<CellBoundary> for FaceIJK<Pentagon> {
    fn into(&self) -> CellBoundary { CellBoundary{} }
}
```

Another issue is the integer types. Many structures will use `int` fields and functions will accept `int` arguments. For example, `getHexagonAreaAvgKm2`, `getHexagonAreaAvgM2`, `getHexagonEdgeLengthAvgKm`, `getHexagonEdgeLengthAvgM`, and `getNumCells` all accept the resolution as an `int`. But each function returns an error if `res < 0`. This indicates that where resolution is used (and it's used all over the place), it can be unsigned. Additionally, these same functions use the resolution value to index an array, which would indicate that `usize` is the right choice.