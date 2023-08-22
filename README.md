# raku-Inline-Rust

**WORK IN PROGRESS**

### Description

Since Rust is a compiled language and Raku connects to a Rust ```cdylib``` directly via the core [NativeCall](https://docs.raku.org/language/nativecall) capability, this raku module is more of a "how to" set of examples.

It has been dubbed Inline::Rust to follow the naming convention of similar Raku interface modules (but no actual Raku code is needed to make these examples work since NativeCall works out-of-the-box).

The use case provided is for a Raku programme to access Rust library functions (e.g. for performance and re-usability).

### Source Material

The raku examples are inspired by the [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/) and they have been offered back for inclusion into this guide.

### Getting Started
 
```
docker run -it librasteve/rakudo:rusty
cd ~ && git clone https://github.com/p6steve/raku-Inline-Rust.git
cd raku-Inline-Rust/ffi-omnibus && cargo build
cd .. && raku ffi-omnibus.raku
```

### Do It Yourself

To make a new cdylib, you can do this:

```
 cargo new --lib your-name && cd your-name
 vi Cargo.toml, add [dependencies] and [lib] sections (see example)
 vi src/lib.rs, write some Rust code (see example)
 cargo build
 cd .. && vi your-name.raku, write some Raku code (see example)
 raku your-name.raku
 ```
