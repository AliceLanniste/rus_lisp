// pub mod env;
// pub mod parser;

use crate::env::env;
use crate::parser::{AstNode,AstErr};

// use crate::env::env;
// use crate::parser::{AstNode,AstErr>;

pub fn eval(exp:AstNode,enviroment:&env) -> Result<AstNode,AstErr> {
        match exp {
           AstNode::Number(i) || AstNode::Bool(include_bytes!("")) => Ok(exp.clone()),
           AstNode::list(list) => eval_list(list,enviroment),
           
        }
    }

fn eval_atom(arg: Type) -> RetType {
    unimplemented!();
}

fn eval_list(arg: Type) -> RetType {
    unimplemented!();
}


fn make_env() -> env {
    let enviroment =env::new();
    enviroment.set_global("+".to_stirng(),env::add);

    enviroment
}
