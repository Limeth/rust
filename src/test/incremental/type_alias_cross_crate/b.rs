// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:a.rs
// revisions:rpass1 rpass2 rpass3

#![feature(rustc_attrs)]

extern crate a;

#[rustc_dirty(label="TypeckItemBody", cfg="rpass2")]
#[rustc_clean(label="TypeckItemBody", cfg="rpass3")]
pub fn use_X() -> u32 {
    let x: a::X = 22;
    x as u32
}

#[rustc_clean(label="TypeckItemBody", cfg="rpass2")]
#[rustc_clean(label="TypeckItemBody", cfg="rpass3")]
pub fn use_Y() {
    let x: a::Y = 'c';
}

pub fn main() { }
