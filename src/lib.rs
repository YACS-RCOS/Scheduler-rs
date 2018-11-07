extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate rayon;


//extern crate cassowary;
//use cassowary::{ Solver, Variable };
//use cassowary::WeightedRelation::*;
//use cassowary::strength::{ WEAK, MEDIUM, STRONG, REQUIRED };

pub mod model;
use model::*;
pub mod solver;
use solver::*;
