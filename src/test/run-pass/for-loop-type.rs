// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


pub fn main() {
    let mut loop_body_did_run = false;
    // test that the type after the pattern allows
    // from_str's type param to be inferred
    for &x: &int in from_str("42").iter() {
        assert_eq!(x, 42);
        loop_body_did_run = true;
    }
    assert!(loop_body_did_run);
}
