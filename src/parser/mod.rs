use std::fmt;
use crate::tokenizer::*;

struct ParseContext<'a> {
    head: Iterator<Item = Token<'a>>,
}

pub struct AbstractSyntaxTree<'a> {
    head: &'a str,
}
type AST<'a> = AbstractSyntaxTree<'a>;

impl fmt::Display for  AST<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is AST")
    }
}


pub fn parse(tokens: Vec<Token>) -> Result<AST, &str> {
    Ok(AbstractSyntaxTree { head: "None" })
}
