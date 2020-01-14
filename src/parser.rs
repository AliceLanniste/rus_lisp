
// mod lexer;
// pub mod Ast;

use crate::lexer::{Lexer, SExp};
use crate::Ast::AstNode;


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

  


    pub fn get_expr(&mut self) -> Option<AstNode> {
            match self.lexer.next() {
                Some(SExp::Number(i)) => Some(AstNode::Number(i)),
                Some(SExp::Bool(i)) => if i =="#f" {
                                        return Some(AstNode::Bool(false));
                                    } else {
                                        return Some(AstNode::Bool(true));
                                    },
                Some(SExp::LParen) => {
                      unreachable!()},
                Some(SExp::RParen) => {panic!("");}
                _ => None,                   
            }
        }


    // fn into_ast(arg: Type) -> RetType {
    //     unimplemented!();
    // }

}