
use crate::lexer::{Lexer, SExp};
use crate::ast::AstNode;


#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>
}

impl<'a> Parser<'a> {
    pub fn new (text: &'a str) -> Self{
        Parser{
            lexer: Lexer::new(text)
        }
    }

    // 这个方法应该改为，get_expr(&mut self, t:SExp)
    pub fn get_expr(&mut self) -> Option<AstNode> {
            match self.lexer.next() {
                Some(SExp::Number(i)) => Some(AstNode::Number(i)),
                Some(SExp::Bool(i)) => if i =="#f" {
                                        return Some(AstNode::Bool(false));
                                    } else {
                                        return Some(AstNode::Bool(true));
                                    },
                Some(SExp::LParen) => {
                     let list = self.get_list();
                     return Some(AstNode::List(list)); },
                Some(SExp::RParen) => {panic!("");}
                _ => None,                   
            }
        }


    fn get_list(&mut self)  -> Vec<AstNode>{
        let mut list : Vec<AstNode> = Vec::new();
        loop {
            match self.lexer.next() {
                Some(SExp::RParen)  =>{ return list;},
                Some(token) => {
                    unreachable!();
                },
                None => {return vec![];},
            }
        }


    }

    // fn into_ast(arg: Type) -> RetType {
    //     unimplemented!();
    // }

}