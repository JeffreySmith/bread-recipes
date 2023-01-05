#![warn(clippy::all,clippy::pedantic)]

mod recipe;
use crate::recipe::*;

use bracket_lib::prelude::*;

struct State {
    recipes:Vec<Recipe>,

}
impl State {
    fn new() -> Self {
        State {
            recipes:vec![],
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx:&mut BTerm) {
    
        ctx.cls();
        ctx.print(1,1,"Hello!");
    }
}


fn main() -> BError{
    let mut recipes:Vec<Recipe> = vec![];
    import_all_recipes("recipes/".to_string(),&mut recipes);
    for r in recipes {
        println!("{:?} has {}g of water and {}g of salt",r,r.water(),r.salt());
    }
    let mut context = BTermBuilder::simple80x50()
        .with_title("Recipe calculator")
        .with_vsync(true)
        .build()?;
    main_loop(context,State::new())
}
