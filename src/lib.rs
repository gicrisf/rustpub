pub mod cli;
pub mod cmd;
pub mod epub;
// pub mod error;

#[macro_use]
extern crate error_chain;

pub enum RustpubParser {
    ReadabiliPy,
    ReadabilityJs,
    ReadabilityRs,
}
