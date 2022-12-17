#![no_std]
#![deny(unsafe_code)]

//! Rc5 contains implementations for the following parameters:
//! - Rc5_8_12_4
//! - Rc5_16_16_8
//! - Rc5_32_12_16
//! - Rc5_32_20_16
//!
#[macro_use]
extern crate alloc;

#[macro_use]
mod internal_macro;

/// Contains the errors handled by this crate.
pub mod error;

#[allow(non_snake_case)]
/// Rc5 contains the implementations for each Rc5 variant.
pub mod rc5;
