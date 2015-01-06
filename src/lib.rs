#![no_std]
#![feature(phase)]
#![feature(globs)]

#[phase(plugin, link)]
extern crate core;
extern crate alloc;

mod intrusive;
mod aligned_ptr_pun;
mod easy_unsafe_ref;

pub mod red_black;

#[test]
fn it_works() {
}


// for deriving and macros
#[doc(hidden)]
#[cfg(not(test))]
mod std {
  pub use core::clone;
  pub use core::cmp;
  pub use core::kinds;
  pub use core::option;
  pub use core::fmt;
  pub use core::hash;
}
