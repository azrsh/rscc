use crate::tokenizer::*;
use std::fmt;

struct ParseContext<'a> {
    head: &'a [Token<'a>],
}

#[derive(Debug)]
pub struct AST<'a> {
    root: ExpressionNode<'a>,
}

#[derive(Debug)]
pub enum ExpressionNode<'a> {
    TernaryOperator {
        kind: TernaryOperatorKind,
        first: Box<ExpressionNode<'a>>,
        second: Box<ExpressionNode<'a>>,
        third: Box<ExpressionNode<'a>>,
    },
    BinaryOperator {
        kind: BinaryOperatorKind,
        lhs: Box<ExpressionNode<'a>>,
        rhs: Box<ExpressionNode<'a>>,
    },
    UnaryOperator {
        kind: UnaryOperatorKind,
        operand: Box<ExpressionNode<'a>>,
    },
    Immediate(Immediate<'a>),
}

#[derive(Debug)]
pub enum TernaryOperatorKind {
    Conditional,
}

#[derive(Debug)]
pub enum BinaryOperatorKind {
    Add,           // +
    Sub,           // -
    Mul,           // *
    Div,           // /
    Mod,           // %
    Dot,           // .
    LeftShift,     // <<
    RightShift,    // >>
    Equal,         // ==
    NotEqual,      // !=
    LessThan,      // <
    LessThanEqual, // <=
    LogicalAnd,    // &&
    LogicalOr,     // ||
    BitwiseAnd,    // &
    BitwiseXor,    // ^
    BitwiseOr,     // |
    Assign,        // =
    Comma,         // ,
}

#[derive(Debug)]
pub enum UnaryOperatorKind {
    LogicalNot,   // !operand
    BitwiseNot,   // ~operand
    Reference,    // &operand
    Dereference,  // *operand
    Cast,         // (type-name)operand
    FunctionCall, // operand(parameter-lisr)
}

#[derive(Debug)]
pub enum Immediate<'a> {
    String(&'a str),
    Char(char),
    Number(u32),
}

fn consume_token(context: &mut ParseContext) {
    assert!(context.head.len() > 0);
    context.head = &context.head[1..];
}

fn consume_punctuator<'a, 'b>(
    context: &'a mut ParseContext<'b>,
    target: PunctuatorKind,
) -> Option<()> {
    assert!(context.head.len() > 0);
    if let Token::Punctuator(kind) = &context.head[0] {
        if target == *kind {
            consume_token(context);
            return Some(());
        }
    }
    None
}

fn expect_punctuator<'a, 'b>(
    context: &'a mut ParseContext<'b>,
    target: PunctuatorKind,
) -> Result<(), String> {
    consume_punctuator(context, target).ok_or(format!("expected {:?} but not found.", target))
}

fn consume_literal<'a, 'b>(context: &'a mut ParseContext<'b>) -> Option<&'b Literal<'b>> {
    match &context.head[0] {
        Token::Literal(content) => Some(content),
        _ => None,
    }
}

fn expect_literal<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<&'b Literal<'b>, String> {
    consume_literal(context).ok_or("expected literal but not found.".to_string())
}

//parser body
fn expression<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    primary(context)
}

fn primary<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    match consume_punctuator(context, PunctuatorKind::LeftRoundBracket) {
        Some(_) => {
            let result = expression(context);
            match expect_punctuator(context, PunctuatorKind::RightRoundBracket) {
                Ok(_) => result,
                Err(message) => return Err(message),
            }
        }
        None => literal(context),
    }
}

fn literal<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let content = match expect_literal(context) {
        Ok(content) => content,
        Err(message) => return Err(message),
    };

    let result = match content {
        Literal::String(content) => Immediate::String(content),
        Literal::Char(content) => Immediate::Char(*content),
        Literal::Number(content) => Immediate::Number(*content),
    };

    Ok(ExpressionNode::Immediate(result))
}

pub fn parse<'a>(tokens: &'a [Token<'a>]) -> Result<AST<'a>, String> {
    let mut context = ParseContext::<'a> { head: tokens };
    match expression(&mut context) {
        Ok(node) => Ok(AST { root: node }),
        Err(message) => Err(message),
    }
}
