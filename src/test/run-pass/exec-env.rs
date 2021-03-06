// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-fast (exec-env not supported in fast mode)
// exec-env:TEST_EXEC_ENV=22

use std::os;

pub fn main() {
    assert_eq!(os::getenv("TEST_EXEC_ENV"), Some(~"22"));
}
