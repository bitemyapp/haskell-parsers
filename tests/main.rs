extern crate haskell_parsers;

use haskell_parsers::*;

// mod combine;

// mod common;
// use common::*;

pub fn main() {
    // hpcombine::parse_file("haskell-src/SimpleModule.hs");
    let result = hpnom::parse_file("tests/haskell-src/SimpleModule.hs");
    println!("{:?}", result);
    let result = hpnom::module(b"module Blah where");
    println!("{:?}", result);
    let result = hpnom::module(b"module Blah.Woo where");
    println!("{:?}", result);
    let result = hpnom::module_name(b"Blah.Woo");
    println!("{:?}", result);
}
