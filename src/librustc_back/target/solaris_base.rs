// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use target::TargetOptions;
use std::default::Default;

pub fn opts() -> TargetOptions {
    TargetOptions {
        linker: "cc".to_string(),
        dynamic_linking: true,
        executables: true,
        has_rpath: true,
        is_like_solaris: true,
        archive_format: "gnu".to_string(),
        exe_allocation_crate: super::maybe_jemalloc(),

        .. Default::default()
    }
}
