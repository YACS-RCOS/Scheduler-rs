#![warn(missing_docs)]
#![warn(missing_copy_implementations)]
// ^ Make the compiler warn us if we are missing Copy traits or
// documentation

//! # Scheduler-rs
//! *A crate for solving course schedules.*
//!
//! *Part of [yacs](https://yacs.io/)*
//!

/// We use serde for Serialization / Deserialization
#[macro_use]
pub extern crate serde;
pub use serde::{Serialize, Deserialize};

/// ## The model module
/// This module contains the data model and structures for storing schedules
/// and parts of schedules.
pub mod model;

/// ## The solver module
/// This module contains methods for solving producing valid schedules based on
/// lists of Scheduleables. (See module level documentation)
pub mod solver;