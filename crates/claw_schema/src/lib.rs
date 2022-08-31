//! A serializable representation of the contents of `project.json`.
//!
//! For the main struct, look at [`Project`].

#![cfg_attr(debug_assertions, allow(unused))]

mod block;
mod costume;
mod project;
mod target;

pub use self::{block::*, costume::*, project::*, target::*};
