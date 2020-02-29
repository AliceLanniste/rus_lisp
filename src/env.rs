use crate::parser::{AstErr, AstNode, SCallbe};

use std::collections::HashMap;

pub struct Env {
    enviroment: Vec<HashMap<String, SCallbe>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            enviroment: vec![HashMap::new()],
        }
    }

    pub fn set(&mut self, name: String, value: SCallbe) {
        let mut map = HashMap::new();
        map.insert(name, value);
        self.enviroment.insert(0, map);
    }

    pub fn set_global(&mut self, name: String, value: SCallbe) {
        self.enviroment[0].insert(name, value);
    }

    pub fn get(&self, key: &String) -> Option<SCallbe> {
        match self.enviroment.iter().next() {
            Some(v) => {
                let func = if let Some(f) = v.get(key) {
                    Some(f.clone())
                } else {
                    self.get(key)
                };
                func
            }
            None => None,
        }
    }
}

pub fn add(number: &[AstNode]) -> Result<AstNode, AstErr> {
    number
        .iter()
        .fold(Ok(AstNode::Number(0)), |a, b| match (a, b) {
            (Ok(AstNode::Number(i)), AstNode::Number(i2)) => Ok(AstNode::Number(i + i2)),
            (_, _) => Err(AstErr::Msg("parser error".to_string())),
        })
}
pub fn sub(number: &[AstNode]) -> Result<AstNode, AstErr> {
    let first = &number[0];
    number
        .into_iter()
        .skip(1)
        .fold(Ok(first.clone()), |a, b| match (a, b) {
            (Ok(AstNode::Number(i)), AstNode::Number(i2)) => Ok(AstNode::Number(i - i2)),
            (_, _) => Err(AstErr::Msg("sub error".to_string())),
        })
}

pub fn mul(number: &[AstNode]) -> Result<AstNode, AstErr> {
    unimplemented!();
}
