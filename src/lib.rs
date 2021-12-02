#![feature(associated_type_defaults, array_windows)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::default_trait_access,
    clippy::cast_lossless
)]

pub mod days;
pub mod solver;
pub(crate) mod util;
