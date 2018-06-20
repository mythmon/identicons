//! A server that serves up identicons.

#![deny(missing_docs)]

extern crate serde;
#[macro_use]
extern crate lazy_static;

/// Helpers for rendering templates
pub mod templ;