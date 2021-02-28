use crate::tokenizer::*;

pub struct ParseContext<'a> {
    head: &'a [Token<'a>],
}

impl ParseContext<'_> {
    pub fn new<'a>(tokens: &'a [Token]) -> ParseContext<'a> {
        ParseContext::<'a> { head: tokens }
    }
}

pub fn consume_token(context: &mut ParseContext) {
    assert!(!context.head.is_empty());
    context.head = &context.head[1..];
}

pub fn consume_punctuator(context: &mut ParseContext, target: PunctuatorKind) -> Option<()> {
    assert!(!context.head.is_empty());
    if let Token::Punctuator(kind) = &context.head[0] {
        if target == *kind {
            consume_token(context);
            return Some(());
        }
    }
    None
}

pub fn expect_punctuator(context: &mut ParseContext, target: PunctuatorKind) -> Result<(), String> {
    consume_punctuator(context, target)
        .ok_or_else(|| format!("expected {:?} but not found.", target))
}

pub fn consume_literal<'a, 'b>(context: &'a mut ParseContext<'b>) -> Option<&'b Literal<'b>> {
    match &context.head[0] {
        Token::Literal(content) => {
            consume_token(context);
            Some(content)
        }
        _ => None,
    }
}

pub fn expect_literal<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<&'b Literal<'b>, String> {
    consume_literal(context).ok_or_else(|| "expected literal but not found.".to_string())
}
