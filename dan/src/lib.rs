use std::ffi::*; //{CStr, CString,}
use std::collections::HashMap;
use std::cell::RefCell;
use std::fs::File;
use std::path::{Path};
use libc::c_char;

use polars::prelude::*;//{CsvReader, DataType, Field, Schema, DataFrame,};
use polars::prelude::{Result as PolarResult};
use polars::frame::DataFrame;

pub fn df_load_csv(spath: &str) -> PolarResult<DataFrame> {
    let fpath = Path::new(spath);
    let file = File::open(fpath).expect("Cannot open file.");

    CsvReader::new(file)
    .has_header(true)
    .finish()
}
#[no_mangle]
pub extern "C" fn df_read_csv(string: *const c_char) {
    let df = df_load_csv(&str_in(string)).unwrap();
    println!{"{}", df.head(Some(5))};

    let c = df.column("petal.length").unwrap();
    println!{"{}", c};

    let x = df
            .groupby(["variety"])
            .unwrap()
            .select(["petal.length"])
            .sum();

    println!{"{:?}", x};
}


#[no_mangle]
pub extern "C" fn df_ret_csv(string: *const c_char) -> *const DataFrame {
    let df = df_load_csv(&str_in(string)).unwrap();
    &df as *const _ 
}
#[no_mangle]
pub extern "C" fn df_head( df: DataFrame ) {
//pub extern "C" fn df_head( df: <*const DataFrame> ) {
    println!{"{}", df.head(Some(2))};
}


fn str_in(i_string: *const c_char) -> String {
    unsafe {
        CStr::from_ptr(i_string).to_string_lossy().into_owned()
    }
}
#[no_mangle]
pub extern "C" fn str2rust(string: *const c_char) {
    println!("{:?}", str_in(string));
}

//viz. https://metacpan.org/pod/FFI::Platypus::Lang::Rust#returning-strings
fn str_out(o_string: &str) -> *const u8 {
    thread_local! {
        static KEEP: RefCell<Option<CString>> = RefCell::new(None);
    }
 
    let c_string = CString::new(o_string).unwrap();
    let ptr = c_string.as_ptr();
    KEEP.with(|k| {
        *k.borrow_mut() = Some(c_string);
    });
 
    ptr
}
#[no_mangle]
pub extern "C" fn str2raku() -> *const u8 {
    str_out("yo")
}

#[no_mangle]
pub extern "C" fn add(a:i32, b:i32) -> i32 {
    a+b 
}
