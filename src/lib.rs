#[cfg(feature = "proc-macro2")]
#[doc(hidden)]
pub use proc_macro2 as _proc_macro2_impl;
#[cfg(not(feature = "proc-macro2"))]
#[doc(hidden)]
pub mod _proc_macro2_impl {}

/// creates a module with any name (by default `proc_macro`) that
/// re-exports either all of `proc_macro` or all of `proc_macro2` items.
/// 
/// should not be used in a `proc-macro` context if the `proc_macro2` feature is set
#[macro_export]
macro_rules! import {
    ($name:ident) => {
        mod $name {
            use $crate::_proc_macro2_impl::*;
            #[cfg(proc_macro)]
            extern crate proc_macro as _proc_macro_impl;
            pub use _proc_macro_impl::*;
        }
    };
    () => { import!(proc_macro) };
}
