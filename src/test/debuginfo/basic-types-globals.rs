// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Caveats - gdb prints any 8-bit value (meaning rust I8 and u8 values)
// as its numerical value along with its associated ASCII char, there
// doesn't seem to be any way around this. Also, gdb doesn't know
// about UTF-32 character encoding and will print a rust char as only
// its numerical value.

// ignore-windows: FIXME #13256
// ignore-android: FIXME(#10381)
// min-lldb-version: 310

// compile-flags:-g
// gdb-command:rbreak zzz
// gdb-command:run
// gdb-command:finish
// gdb-command:print 'basic-types-globals::B'
// gdb-check:$1 = false
// gdb-command:print 'basic-types-globals::I'
// gdb-check:$2 = -1
// gdb-command:print 'basic-types-globals::C'
// gdb-check:$3 = 97
// gdb-command:print/d 'basic-types-globals::I8'
// gdb-check:$4 = 68
// gdb-command:print 'basic-types-globals::I16'
// gdb-check:$5 = -16
// gdb-command:print 'basic-types-globals::I32'
// gdb-check:$6 = -32
// gdb-command:print 'basic-types-globals::I64'
// gdb-check:$7 = -64
// gdb-command:print 'basic-types-globals::U'
// gdb-check:$8 = 1
// gdb-command:print/d 'basic-types-globals::U8'
// gdb-check:$9 = 100
// gdb-command:print 'basic-types-globals::U16'
// gdb-check:$10 = 16
// gdb-command:print 'basic-types-globals::U32'
// gdb-check:$11 = 32
// gdb-command:print 'basic-types-globals::U64'
// gdb-check:$12 = 64
// gdb-command:print 'basic-types-globals::F32'
// gdb-check:$13 = 2.5
// gdb-command:print 'basic-types-globals::F64'
// gdb-check:$14 = 3.5
// gdb-command:continue

#![allow(unused_variables)]

static B: bool = false;
static I: int = -1;
static C: char = 'a';
static I8: i8 = 68;
static I16: i16 = -16;
static I32: i32 = -32;
static I64: i64 = -64;
static U: uint = 1;
static U8: u8 = 100;
static U16: u16 = 16;
static U32: u32 = 32;
static U64: u64 = 64;
static F32: f32 = 2.5;
static F64: f64 = 3.5;

fn main() {
    _zzz();

    let a = (B, I, C, I8, I16, I32, I64, U, U8, U16, U32, U64, F32, F64);
}

fn _zzz() {()}
