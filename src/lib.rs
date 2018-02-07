//! A Rust crate for generating identicons.
//!
//! Identicons are deterministic yet unpredictable icons that can be used as
//! avatars or other visual identifiers.
//!
//! * Deterministic: given the same input, you'll always get the same identicon
//! back out.
//!
//! * Unpredictable: similar-but-just-barely-different inputs give back
//! radically different identicons.

#![deny(missing_docs)]

extern crate iron;
extern crate rand;
extern crate router;
extern crate iron_tera;
extern crate tera;
#[macro_use]
extern crate lazy_static;
extern crate ctrlc;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod icons;
pub mod server;
