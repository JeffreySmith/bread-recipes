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
        
        if check_is_json(&filename)  {
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
        }
        Ok(())
    }

}
pub fn check_is_json(filename:&String)->bool{
    let extension=Path::new(filename)
        .extension()
        .and_then(OsStr::to_str);
    match extension {
        Some("json") => true,
        _ => false,
    }
}