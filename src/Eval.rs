pub mod env;
pub mod parser;

use crate::env::env;

pub fn eval(exp:AstNode) {
        unimplemented!();
    }




fn make_env() -> env {
    let enviroment =env::new();
    enviroment.set("+".to_stirng(),env::add);

    enviroment
}
