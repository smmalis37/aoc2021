#![feature(
    associated_type_defaults,
    type_alias_impl_trait,
    array_windows,
    destructuring_assignment,
    mixed_integer_ops,
    label_break_value
)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![forbid(unsafe_code)]
#![allow(
    clippy::wildcard_imports,
    clippy::default_trait_access,
    clippy::cast_lossless,
    clippy::unreadable_literal
)]

pub mod days;
pub mod solver;
pub(crate) mod util;
