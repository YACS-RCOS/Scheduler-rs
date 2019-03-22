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
pub use serde;
#[macro_use]
extern crate serde_derive;
pub use serde::{Serialize, Deserialize};

/// ## The model module
/// This module contains the data model and structures for storing schedules
/// and parts of schedules.
pub mod model;

/// ## The solver module
/// This module contains methods for solving producing valid schedules based on
/// lists of Scheduleables. (See module level documentation)
pub mod solver;

/// ## The Private Models used in computation
pub mod models;

/// ## Solver module, with implementation using private library instead
/// of public serialization stuff
pub mod solve;
