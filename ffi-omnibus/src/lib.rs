use libc::c_char;
use libc::size_t;
use std::ffi::*; //{CStr, CString,}
use std::iter;
use std::slice;
use std::convert::From;
use std::collections::HashMap;

// Rust FFI Omnibus: Integers
// http://jakegoulding.com/rust-ffi-omnibus/integers/

#[no_mangle]
pub extern "C" fn addition(a:i32, b:i32) -> i32 {
    a+b 
}

// Rust FFI Omnibus: String Arguments
// http://jakegoulding.com/rust-ffi-omnibus/string-arguments/

#[no_mangle]
pub extern "C" fn how_many_characters(s: *const c_char) -> u32 {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as u32
}

// Rust FFI Omnibus: String Return Values
// http://jakegoulding.com/rust-ffi-omnibus/string-return-values/

#[no_mangle]
pub extern "C" fn theme_song_generate(length: u8) -> *mut c_char {
    let mut song = String::from("💣 ");
    song.extend(iter::repeat("na ").take(length as usize));
    song.push_str("Batman! 💣");

    let c_str_song = CString::new(song).unwrap();
    c_str_song.into_raw()
}

#[no_mangle]
pub extern "C" fn theme_song_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

// Rust FFI Omnibus: Slice Arguments
// http://jakegoulding.com/rust-ffi-omnibus/slice-arguments/

#[no_mangle]
pub extern "C" fn sum_of_even(n: *const u32, len: size_t) -> u32 {
    let numbers = unsafe {
        assert!(!n.is_null());

        slice::from_raw_parts(n, len as usize)
    };

    numbers
        .iter()
        .filter(|&v| v % 2 == 0)
        .sum()
}

// Rust FFI Omnibus: Tuples 
// http://jakegoulding.com/rust-ffi-omnibus/tuples/

// A Rust function that accepts a tuple
fn flip_things_around_rust(tup: (u32, u32)) -> (u32, u32) {
    let (a, b) = tup;
    (b + 1, a - 1)
}

// A struct that can be passed between C and Rust
#[repr(C)]
pub struct Tuple {
    x: u32,
    y: u32,
}

// Conversion functions
impl From<(u32, u32)> for Tuple {
    fn from(tup: (u32, u32)) -> Tuple {
        Tuple { x: tup.0, y: tup.1 }
    }
}

impl From<Tuple> for (u32, u32) {
    fn from(tup: Tuple) -> (u32, u32) {
        (tup.x, tup.y)
    }
}

// The exported C method
#[no_mangle]
//pub extern "C" fn flip_things_around(tup: Tuple) {
pub extern "C" fn flip_things_around(tup: Tuple) -> Tuple {
    flip_things_around_rust(tup.into()).into()
}

// Rust FFI Omnibus: Objects
// http://jakegoulding.com/rust-ffi-omnibus/objects/

pub struct ZipCodeDatabase {
    population: HashMap<String, u32>,
}

impl ZipCodeDatabase {
    fn new() -> ZipCodeDatabase {
        ZipCodeDatabase {
            population: HashMap::new(),
        }
    }

    fn populate(&mut self) {
        for i in 0..100_000 {
            let zip = format!("{:05}", i);
            self.population.insert(zip, i);
        }
    }

    fn population_of(&self, zip: &str) -> u32 {
        self.population.get(zip).cloned().unwrap_or(0)
    }
}

#[no_mangle]
pub extern "C" fn zip_code_database_new() -> *mut ZipCodeDatabase {
    Box::into_raw(Box::new(ZipCodeDatabase::new()))
}

#[no_mangle]
pub extern "C" fn zip_code_database_free(ptr: *mut ZipCodeDatabase) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn zip_code_database_populate(ptr: *mut ZipCodeDatabase) {
    let database = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    database.populate();
}

#[no_mangle]
pub extern "C" fn zip_code_database_population_of(
    ptr: *const ZipCodeDatabase,
    zip: *const c_char,
) -> u32 {
    let database = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let zip = unsafe {
        assert!(!zip.is_null());
        CStr::from_ptr(zip)
    };
    let zip_str = zip.to_str().unwrap();
    database.population_of(zip_str)
}

