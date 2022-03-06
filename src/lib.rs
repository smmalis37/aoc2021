#![feature(
    type_alias_impl_trait,
    mixed_integer_ops,
    label_break_value,
    int_roundings
)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::if_then_some_else_none
)]
#![forbid(unsafe_code)]
#![allow(
    clippy::wildcard_imports,
    clippy::default_trait_access,
    clippy::unreadable_literal
)]

pub mod days;
pub mod solver;
pub(crate) mod util;
