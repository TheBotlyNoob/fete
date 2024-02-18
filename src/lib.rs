// #![no_std]
#![feature(const_mut_refs)]
#![warn(clippy::pedantic, clippy::nursery)]
#![doc = include_str!("../README.md")]

pub mod bus;
pub mod cpu;
pub mod opcode;
pub mod rom;

#[cfg(any(test, fete_doctest))]
pub mod testing;
