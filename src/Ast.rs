
#[derive(Debug,Clone)]
pub enum AstNode {
    Number(i64),
    Bool(bool),
    List(Vec<AstNode>),
    
}