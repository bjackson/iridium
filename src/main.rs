#![feature(extern_prelude)]
extern crate num;
#[macro_use]
extern crate num_derive;

pub mod vm;
pub mod instruction;


fn main() {
    println!("Hello, world!");
}
