//@run-rustfix
//@aux-build:proc_macros.rs
#![feature(let_chains)]
#![allow(
    clippy::blocks_in_if_conditions,
    clippy::if_same_then_else,
    clippy::ifs_same_cond,
    clippy::needless_else,
    clippy::no_effect,
    clippy::nonminimal_bool,
    unused
)]
#![warn(clippy::needless_if)]

extern crate proc_macros;
use proc_macros::external;
use proc_macros::with_span;

fn no_side_effects() -> bool {
    true
}

fn has_side_effects(a: &mut u32) -> bool {
    *a = 1;
    true
}

fn main() {
    // Lint
    if (true) {}
    // Do not remove the condition
    if no_side_effects() {}
    let mut x = 0;
    if has_side_effects(&mut x) {}
    assert_eq!(x, 1);
    // Do not lint
    if (true) {
    } else {
    }
    if {
        return;
    } {}
    // Do not lint if `else if` is present
    if (true) {
    } else if (true) {
    }
    // Do not lint if any `let` is present
    if let true = true {}
    if let true = true && true {}
    if true && let true = true {}
    if {
        if let true = true && true { true } else { false }
    } && true
    {}
    external! { if (true) {} }
    with_span! {
        span
        if (true) {}
    }
}
