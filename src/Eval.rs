
use crate::env::{Env,add};
use crate::parser::{AstNode,AstErr};



pub fn eval(exp:AstNode,enviroment:&Env) -> Result<AstNode,AstErr> {
        match exp {
           AstNode::Number(_) | AstNode::Bool(_) => Ok(exp.clone()),
           AstNode::List(list) => eval_list(list,enviroment),
           _ => unimplemented!()
        }
    }



fn eval_list(list:Vec<AstNode>,enviroment:&Env) -> Result<AstNode,AstErr> {
  unimplemented!()
}


pub fn make_env() -> Env {
    let mut enviroment =Env::new();
    enviroment.set_global("+".to_string(),AstNode::Func(add));

    enviroment
}
