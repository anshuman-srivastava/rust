// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that lifetime resolver enforces the lifetime name scoping
// rules correctly in various scenarios.

struct Foo<'a> {
    x: &'a isize
}

impl<'a> Foo<'a> {
    // &'a is inherited:
    fn m1(&self, arg: &'a isize) { }
    fn m2(&'a self) { }
    fn m3(&self, arg: Foo<'a>) { }

    // &'b is not:
    fn m4(&self, arg: &'b isize) { } //~ ERROR undeclared lifetime
    fn m5(&'b self) { } //~ ERROR undeclared lifetime
    fn m6(&self, arg: Foo<'b>) { } //~ ERROR undeclared lifetime
}

fn bar<'a>(x: &'a isize) {
    // &'a is visible to code:
    let y: &'a isize = x;

    // &'a is not visible to *items*:
    type X = Option<&'a isize>; //~ ERROR undeclared lifetime
    enum E {
        E1(&'a isize) //~ ERROR undeclared lifetime
    }
    struct S {
        f: &'a isize //~ ERROR undeclared lifetime
    }
    fn f(a: &'a isize) { } //~ ERROR undeclared lifetime

    // &'a CAN be declared on functions and used then:
    fn g<'a>(a: &'a isize) { } // OK
    fn h(a: Box<for<'a> FnOnce(&'a isize)>) { } // OK
}

// Test nesting of lifetimes in fn type declarations
fn fn_types(a: &'a isize, //~ ERROR undeclared lifetime
            b: Box<for<'a> FnOnce(&'a isize,
                                  &'b isize, //~ ERROR undeclared lifetime
                                  Box<for<'b> FnOnce(&'a isize,
                                                     &'b isize)>,
                                  &'b isize)>, //~ ERROR undeclared lifetime
            c: &'a isize) //~ ERROR undeclared lifetime
{
}

pub fn main() {}
