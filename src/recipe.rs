#![warn(clippy::all,clippy::pedantic)]


use serde_json::Result;
use serde::{Serialize,Deserialize};

use std::fs::{self};
use std::path::Path;
use std::ffi::OsStr;


#[derive(Serialize,Deserialize,Debug,Clone,Default)]
pub struct Recipe {
    pub flour:i32,
    pub hydration:i32,
    pub starter:i32,
    pub salt:f32,
    pub yeast:f32,
    pub oil:f32,
    pub name:String,
}

impl Recipe {
    pub fn import(&mut self,filename:String) -> Result<()> {
        let contents:String=fs::read_to_string(filename).expect(
            "Expected file input");
        let recipe:Recipe = serde_json::from_str(&contents)?;
        
        self.flour = recipe.flour;
        self.hydration= recipe.hydration;
        self.starter=recipe.starter;
        self.salt=recipe.salt;
        self.oil = recipe.oil;
        self.name = recipe.name;
        self.yeast=recipe.yeast;
        Ok(())
    }
    pub fn water(&self) -> f32 {
        self.flour as f32 *(self.hydration as f32/100.0)
    }
    pub fn starter(&self) -> f32 {
        self.flour as f32 *(self.starter as f32/100.0)
    }
    pub fn salt(&self) -> f32 {
        self.flour as f32 *(self.salt/100.0)
    }
    pub fn oil(&self) -> f32 {
        self.flour as f32 *(self.oil/100.0)
    }
    pub fn yeast(&self) -> f32 {
        self.flour as f32 *(self.yeast/100.0)
    }

}
fn check_is_json(filename:&String)->bool{
    let extension=Path::new(filename)
        .extension()
        .and_then(OsStr::to_str);
    match extension {
        Some("json") => true,
        _ => false,
    }
}
pub fn import_all_recipes(filepath:String,recipes:&mut Vec<Recipe>) {
    let paths = fs::read_dir(filepath).unwrap();
    for path in paths {
        let name = path.unwrap().path().display().to_string();
        if check_is_json(&name) {
            let mut recipe = Recipe::default();
            recipe.import(name).unwrap();
            recipes.push(recipe.clone());
        }
    }
}