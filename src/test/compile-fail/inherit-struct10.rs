// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test struct inheritance.
#![feature(struct_inherit)]

virtual struct UnitLikeVirtual;       //~ ERROR unit-like or tuple structs cannot be virtual
virtual struct TupleLikeVirtual(int); //~ ERROR unit-like or tuple structs cannot be virtual

virtual struct RecordLikeVirtual { i: int }
struct UnitLike : RecordLikeVirtual;       //~ ERROR unit-like or tuple structs cannot inherit
struct TupleLike : RecordLikeVirtual(int); //~ ERROR unit-like or tuple structs cannot inherit

fn main() {}
