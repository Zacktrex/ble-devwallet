#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]


pub(crate) mod common;
pub(crate) mod fmt;

#[cfg(feature = "security")]
pub mod ble;

pub mod display;