#![feature(slice_patterns)]

pub(crate) mod lex;
mod test_lex;
pub mod json;
pub mod parse_error;
pub mod parser;
mod test_syntax;
