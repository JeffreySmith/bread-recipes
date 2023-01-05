#![warn(clippy::all,clippy::pedantic)]

mod recipe;
use crate::recipe::*;

//use bracket_lib::prelude::*;




fn main() {
    println!("Hello, world!");
    let mut recipe = Recipe::default();
    recipe.import("recipes/focaccia.json".to_string()).unwrap();
}
