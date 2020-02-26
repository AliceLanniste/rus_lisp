pub mod parser;
use parser::AstNode;

pub struct env{
   map: Vec<Hashmap<String,AstNode>>
}

impl env {
    fn new() -> Self {
        Self {
            map: vec![Hashmap::new()]
        }
    }
  
    pub fn set(&mut self,name:String,value:AstNode)  {
        self.env[0].insert(name,value)
    }

    pub fn set_global(&mut self,name:String,valeu:AstNode)  {
        self.env.insert(name,AstNode)
    }

    pub fn get(&self,key:String) ->Option<AstNode>{
        unimplemented!();
    }
}


pub fn add(number:&[AstNode]) -> Result<AstNode,parser::AstErr> {
    unimplemented!();
}

pub fn sub(number:&[AstNode]) -> Result<AstNode,parser::AstErr> {
    unimplemented!();
}

pub fn mul(number:&[AstNode]) -> Result<AstNode,parser::AstErr> {
    unimplemented!();
}