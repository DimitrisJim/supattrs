#![allow(dead_code, unused_imports, unused_macros )]
extern crate supattrs;
use supattrs::rename;

// ok, for a simple function we have rename rename it to 'Foo'
#[rename]
fn test_run(){
    assert_eq!(2,2);
}

#[rename]
mod mymod {
    fn do_nothing() {}
}

#[rename]
struct Foo{
    i: i32,
}

#[rename]
trait TestTrait {
    fn do_nothing(i: i32) -> i32;
}

#[rename]
impl TestTrait for Foo {
    fn do_nothing(i: i32) -> i32 {i}
}

#[rename]
const MY_CONSTANT: i32 = 10292;

#[rename]
static LIVING: &str = "LIVE FOREVER";


#[rename]
enum BestDays {
    MONDAY,
    WEDNESDAY,
}

#[rename]
type ShinnyFoo = Foo;

#[rename]
use std::collections::HashMap;

#[rename]
extern crate alloc;

#[rename]
union MyUnion {
    f1: u32,
    f2: f32,
}

#[rename]
macro_rules ! dumb {
    () => ()
}

#[rename]
#[rename]
static JUST: f32 = 3.21;

// note: left: foreign mod, macro
#[rename]
extern "C" {
    fn my_dumb_func(x: i32) -> i32;
}

// todo: nightly feature (see `trait_alias`), find a way to test it.
/*
#![feature(trait_alias)]
#[rename]
trait SendTest = TestTrait + Send;
*/

// todo: nightly feature (see decl_macro), find a way to test it.
/*
#![feature(decl_macro)]
#[rename]
macro my_macro = {...}
*/

#[rename]
mod f{
    use super::*;
    fn deokd() {}

}