#![feature(
    type_alias_impl_trait,
    mixed_integer_ops,
    label_break_value,
    int_roundings
)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]
#![forbid(unsafe_code)]
#![allow(
    clippy::wildcard_imports,
    clippy::default_trait_access,
    clippy::unreadable_literal,
    clippy::similar_names,
    clippy::blanket_clippy_restriction_lints,
    clippy::missing_docs_in_private_items,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::single_char_lifetime_names,
    clippy::missing_inline_in_public_items,
    clippy::shadow_reuse,
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::pattern_type_mismatch,
    clippy::separated_literal_suffix,
    clippy::else_if_without_else,
    // TODO fix these!
    clippy::unwrap_used,
    clippy::unreachable,
    clippy::default_numeric_fallback
)]

pub mod days;
pub mod solver;
pub(crate) mod util;
