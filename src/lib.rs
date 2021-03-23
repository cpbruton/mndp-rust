//! MikroTik Neighbor Discovery Protocol (MNDP) library and discovery tool.
//!
//!

#![warn(missing_docs)]

mod neighbor;
mod protocol;

// pub extern crate bytes;
pub extern crate macaddr;

pub use crate::neighbor::{Neighbor, Builder, Unpack};
pub use crate::protocol::{Packet, MndpType, TypeValue, SOLICIT};

