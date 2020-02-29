use crate::env::{add, sub, Env};
use crate::parser::{AstErr, AstNode};

pub fn eval(exp: AstNode, enviroment: &Env) -> Result<AstNode, AstErr> {
    match exp {
        AstNode::Number(_) | AstNode::Bool(_) => Ok(exp.clone()),
        AstNode::List(list) => eval_list(list, enviroment),
        _ => unimplemented!(),
    }
}

fn eval_list(list: Vec<AstNode>, enviroment: &Env) -> Result<AstNode, AstErr> {
    let first = &list[0];
    match first {
        AstNode::Symbol(ref sym) => {
            let f = enviroment.get(&sym).unwrap();
            let value = f(&list[1..]);
            return value;
        }
        _ => Err(AstErr::Msg("add error".to_string())),
    }
}

pub fn make_env() -> Env {
    let mut enviroment = Env::new();
    enviroment.set_global("+".to_string(), add);
    enviroment.set_global("-".to_string(), sub);
    enviroment
}
