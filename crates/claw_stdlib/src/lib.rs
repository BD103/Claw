#![doc = include_str!("../README.md")]

mod opcode;
mod types;

pub use self::{opcode::OpCode, types::Type};
