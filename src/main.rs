#![warn(clippy::all,clippy::pedantic)]

mod recipe;

use crate::recipe::*;

use bracket_lib::prelude::*;

struct State {
    recipes:Vec<Recipe>,
    index:usize,
}
impl State {
    fn new() -> Self {
        let mut recipes:Vec<Recipe> = vec![];
        import_all_recipes("recipes/".to_string(),&mut recipes);
        
        State {
            recipes,
            index:0,
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx:&mut BTerm) {
        let mut line:i32 = 10;

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::N=>self.index+=1,
                VirtualKeyCode::P=>{
                    if self.index==0 {
                        self.index = self.recipes.len()-1;
                    }else{
                        self.index-=1;
                    }
                },
                VirtualKeyCode::Right=>self.index+=1,
                VirtualKeyCode::Left=>{
                    if self.index==0 {
                        self.index = self.recipes.len()-1;
                    }else{
                        self.index-=1;
                    }
                },
                VirtualKeyCode::Escape=>ctx.quitting=true,

                _=>{}
            }
        }
        if self.index > self.recipes.len()-1 {
            self.index=0;
        }
        
        ctx.cls();
        ctx.print(1,1,&format!("{}/{}",self.index+1,self.recipes.len()));
        
        ctx.print_centered(5,&format!("{}",self.recipes[self.index].name));
        ctx.print_centered(6,"--------------------------------------");
        ctx.print_centered(7,&format!("{}g of flour",self.recipes[self.index].flour));
        ctx.print_centered(8,&format!("{}g of water",self.recipes[self.index].water().to_string()));
        ctx.print_centered(9,&format!("{:.4}g of salt",self.recipes[self.index].salt().to_string()));
        if self.recipes[self.index].starter() > 0.0 {
            ctx.print_centered(line,&format!("{}g of starter",self.recipes[self.index].starter().to_string()));
            line+=1;
        }
        if self.recipes[self.index].yeast() > 0.0 {
            ctx.print_centered(line,&format!("{:.4}g of yeast",self.recipes[self.index].yeast().to_string()));
            line+=1;
        }
        if self.recipes[self.index].oil() > 0.0 {
            ctx.print_centered(line,&format!("{:.4}g of oil",self.recipes[self.index].oil().to_string()));
            line+=1;
        }
        ctx.print_centered(line+5,"Press (N)next or (P)revious");
        ctx.print_centered(line+6,"Or Left or Right");
        
    }
}


fn main() -> BError{
    
    let context = BTermBuilder::simple80x50()
        .with_title("Recipe calculator")
        .with_vsync(true)
        .build()?;
    main_loop(context,State::new())
}
