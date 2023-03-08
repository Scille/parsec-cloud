# Proper Rust <-> Javascript number conversion in the bindings

From [ISSUE-3941](https://github.com/Scille/parsec-cloud/issues/3941)

Currently we only support `i32` & `u32` types as lower common denominator for number.

However it would be much better to support the whole `i8/i16/i32/i64/u8/u16/u32/u64` family.
Or, at least, unsigned integer and 64bits given how common they are.

However this have some challenges:

- Javascript default number is 64bits floating number, so a naive conversion bring rounding errors :/
- Conversion works differently between platform: compiling Rust to wasm [seems to natively support all the number types](https://webassembly.github.io/spec/core/syntax/types.html#syntax-numtype) while using neon bindings means more manual conversion typically using [BigInt](https://developer.mozilla.org/fr/docs/Web/JavaScript/Reference/Global_Objects/BigInt)
- Interaction between wasm and javascript is unclear right now, we need to know precisely what happen when passing a big number to/from rust (a BigInt is returned ? a custom type specific to wasm ?)

So what we want to do:

- Replace in the api generator the `int` by sized typed: `u32`, `i64`, `u64` etc. (see how `OnClientEventCallback` is implemented)
- Add Rust-to-Js conversion code in the generator that, in case conversion is not possible (i.e. too big number or negative number for an unsigned type), throws an exception
- Add Js-to-Rust conversion code in the generator. This conversion should not failed and use [Number.MAX_SAFE_INTEGER](https://developer.mozilla.org/fr/docs/Web/JavaScript/Reference/Global_Objects/Number/MAX_SAFE_INTEGER] to determine if the conversion should return a regular number or a BigInt
- Add test in the bindings code to ensure this is handled correctly. Typically:
  - Introduce dummy functions `test_u8_roundtrip`/`test_i8_roundtrip` etc. functions in the bindings crates.
  - For Electron, the compiled output is a node module, so a node script can be use to do the test.
  - For Web bindings, `wasm-pack test` has us covered. The trick is to add inline Javascript in the test body and use [js_sys::eval](https://docs.rs/js-sys/latest/js_sys/fn.eval.html)
