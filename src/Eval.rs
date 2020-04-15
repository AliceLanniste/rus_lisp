use crate::env::env;
use crate::parser::{LispValue,LispRet};

pub fn eval(exp:LispValue,enviroment:&env) -> LispRet {
        match exp {
        //    LispValue::Number(i) || LispValue::Bool(include_bytes!("")) => Ok(exp.clone()),
        //    LispValue::list(list) => eval_list(list,enviroment),
           
        }
    }

fn eval_atom(arg: Type) -> RetType {
    unimplemented!();
}

fn eval_list(arg: Type) -> RetType {
    unimplemented!();
}


// fn make_env() -> env {
//     let enviroment =env::new();
//     enviroment.set_global("+".to_stirng(),env::add);

//     enviroment
// }
