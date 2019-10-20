#![no_std]

#![feature(const_fn)]

use nb;

mod registers;
mod ax5031;

pub use ax5031::Ax5031;
pub use registers::{PowerMode, Modulation, FramingMode, Encoding};

pub use registers::ControlRegister;

