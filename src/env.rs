pub mod parser;
use parser::AstNode;

pub struct env{
   map: Vec<Hashmap<String,AstNode>>
}

impl env {
    pub fn new() -> Self {
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
    number.iter().fold(AstNode::Number(0) |a ,b| {
        match(a ,b) {
            (AstNode::Number(r1),AstNode::Number(r2))  => Ok(AstNode::Number(r1+r2)),
            (_,_) => Err(AstErr::Msg("parser Error".to_string()))
        }
    })
}

pub fn sub(number:&[AstNode]) -> Result<AstNode,parser::AstErr> {
    unimplemented!();
}

pub fn mul(number:&[AstNode]) -> Result<AstNode,parser::AstErr> {
    unimplemented!();
}