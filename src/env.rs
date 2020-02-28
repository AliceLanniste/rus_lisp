
use crate::parser::{AstNode,AstErr};

use std::collections::HashMap;


pub struct Env{
    enviroment: Vec<HashMap<String,AstNode>>
 }
 
 impl Env {
     pub fn new() -> Self {
         Self {
             enviroment: vec![HashMap::new()]
         }
     }
   
     pub fn set(&mut self,name:String,value:AstNode)  {
         let mut map = HashMap::new();
         map.insert(name,value);
         self.enviroment.insert(0,map);
     }
 
     pub fn set_global(&mut self,name:String,value:AstNode)  {
         self.enviroment[0].insert(name,value);
     }
 
     pub fn get(&self,key:String) ->Option<AstNode>{

        match self.enviroment[0].get(&key) {
            Some(v) => Some(v.clone()),
            None => self.enviroment[1].get(&key).cloned(),
        }
     }    
 }


pub fn add(number:&[AstNode]) -> Result<AstNode,AstErr> {
    // number.iter().fold(AstNode::Number(0) |a ,b| {
    //     match(a ,b) {
    //         (AstNode::Number(r1),AstNode::Number(r2))  => Ok(AstNode::Number(r1+r2)),
    //         (_,_) => Err(AstErr::Msg("parser Error".to_string()))
    //     }
    // })
    unimplemented!()
}

pub fn sub(number:&[AstNode]) -> Result<AstNode,AstErr> {
    unimplemented!();
}

pub fn mul(number:&[AstNode]) -> Result<AstNode,AstErr> {
    unimplemented!();
}