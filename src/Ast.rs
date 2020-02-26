

#[derive(Debug,Clone)]
pub enum AstNode {
    Number(i64),
    Bool(bool),
    List(Vec<AstNode>),
    Func(SCallbe)
}


type SCallbe = fn(&[AstNode]) -> Result<AstNode, AstError>;

[derive(Debug)]
pub enum AstErr {
   Msg(String),
}