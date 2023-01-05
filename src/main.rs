#![warn(clippy::all,clippy::pedantic)]

mod recipe;
use crate::recipe::*;

//use bracket_lib::prelude::*;




fn main() {
    let mut recipes:Vec<Recipe> = vec![];
    import_all_recipes("recipes/".to_string(),&mut recipes);
    for r in recipes {
        println!("{:?} has {}g of water",r,r.water());
    }
}
