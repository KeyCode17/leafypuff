#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub mod application;
pub mod domain;
#[cfg(feature = "ffi")]
pub mod ffi;
pub mod infrastructure;

#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();
