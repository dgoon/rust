// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    return
        { return () } //~ ERROR the type of this value must be known in this context
    () //~^ ERROR the type of this value must be known in this context
//~^^ ERROR notation; the first type parameter for the function trait is neither a tuple nor unit
//~^^^ ERROR overloaded calls are experimental
    ;
}