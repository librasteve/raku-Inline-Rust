<<<<<<< HEAD
# raku Dan::Polars

This is a new notion for a new raku module to bind to Polars via Raku NativeCall / Rust FFI.

The following capabilities are planned:
- Polars objects (Series, DataFrames) as shadows
- Polars object methods (ie. auto-generate & export, access via .fallback)
- Polars lazy APIs via raku lazy semantics
- handle map & apply (with raku callbacks)
- raku Dan features (accessors, dtypes)
- broad datatype support & mapping
- concurrency
  
 **WORK IN PROGRESS**
 
 Installation
 
 make pl_so ...
 ```
 cargo new pl_so && cd pl_so
 vi Cargo.toml, add polars = "0.21.1"
 vi src/main.rs (write some code)
 cargo build
 cargo run
 ```
  
First step is to get a Dockerfile up...
- rakudo + rust + polars minimal (no python!)

Notes from Polars Discord

potter420 — 05/01/2022
https://raku-advent.blog/2019/12/13/day-4-a-little-rr/
Raku Advent Calendar
tmtvl
Day 13 – A Little R&R
A Little R&R Introduction Raku is a really nice language. Versatile, expressive, fast, dwimmy. The only problem I sometimes have with it is that it can be a little slow. Fortunately that can ea…

[20:09]
according to this blog, one can make a FFI binding between Raku and Rust
[20:11]
But, @ritchie46  prolly too busy keeping rust and python lib of polars updated. So additional external effort may be needed

ritchie46 — Yesterday at 14:56
Yeap.. 😅
[14:59]
can an FFI capable languge be bound to polars that way reasonably effectively and is there any example/documentation/cheat sheet/advice I can use to avoid reinventing wheels

Yes, definitely look at the python implementation as the reference implementation. The interop goes very well. There are also bindings to nodejs which also may be helpful. The work of @universalmind303 proves that the port is definitely possible and a lot less work than starting from scratch (Trust me, I've got a lot of time in this ;))

Some other notes:
https://news.ycombinator.com/item?id=27051573#27053712
https://www.youtube.com/watch?v=OtIU7HsHCE8&t=2731s
https://arrow.apache.org
=======
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
>>>>>>> 229cef1a528ee05443afa48eafa4b3a9e832409e
