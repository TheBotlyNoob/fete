#![no_std]
#![feature(const_mut_refs)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
#![doc = include_str!("../README.md")]

pub mod cpu;
pub mod opcode;

pub use cpu::Cpu;
