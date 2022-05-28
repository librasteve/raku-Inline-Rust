# raku-Inline-Rust

**WORK IN PROGRESS**

### Description

Since Rust is a compiled language and Raku connects to a Rust cdylib directly via the core [NativeCall](https://docs.raku.org/language/nativecall) capability, this raku module is more of a "how to" set of examples.

It has been dubbed Inline::Rust to follow the naming convention of similar Raku interface modules (but no actual Raku code is needed to make these examples work since NativeCall works out-of-the-box).

### Source Material

The raku examples are inspired by the [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/) and they have been offered back for inclusion into this guide.


### Getting Started
 
```
docker run -it p6steve/raku-dan:polars-2022.02-arm64   
(or edit the FROM --platform=linux/amd64 p6steve/rakudo:ubuntu-latest-amd64 and docker build... if you need Intel x86)
cd ~ && git clone https://github.com/p6steve/raku-Inline-Rust.git
cd raku-Inline-Rust
raku ffi-omnibus.raku
```
