#![feature(crate_in_paths)]
//! Thumb CPU Emulator.

extern crate byteorder;

pub mod cpu;
pub mod instr;

pub use crate::cpu::Cpu;
