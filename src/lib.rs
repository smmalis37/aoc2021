#![feature(associated_type_defaults, type_alias_impl_trait, array_windows)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![forbid(unsafe_code)]
#![allow(
    clippy::wildcard_imports,
    clippy::default_trait_access,
    clippy::cast_lossless
)]

pub mod days;
pub mod solver;
pub(crate) mod util;
