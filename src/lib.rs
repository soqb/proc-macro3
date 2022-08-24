#[cfg(proc_macro)]
extern crate proc_macro;
#[cfg(proc_macro)]
pub use proc_macro::*;
#[cfg(feature = "proc_macro2")]
extern crate proc_macro2;
#[cfg(feature = "proc_macro2")]
pub use proc_macro2::*;

// do not allow use of this library without the "proc_macro2" flag outside of proc_macro contexts
#[cfg(not(any(proc_macro, feature = "proc_macro2"))]
core::compile_error!("this crate must be used either with the \"proc_macro2\" feature flag or in a proc_macro context");