extern crate scheduler_rs;
extern crate serde_json;

use scheduler_rs::model::*;
use scheduler_rs::solver::solve;

use serde_json::from_str;

#[test]
fn simple_test() {
    let class1: Scheduleable = from_str(r#"
    {
    }
    "#).unwrap();
}