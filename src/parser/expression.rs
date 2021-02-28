use crate::parser::util::*;
use crate::tokenizer::*;
use std::sync::Arc;

#[derive(Debug)]
pub enum ExpressionNode<'a> {
    TernaryOperator {
        kind: TernaryOperatorKind,
        first: Arc<ExpressionNode<'a>>,
        second: Arc<ExpressionNode<'a>>,
        third: Arc<ExpressionNode<'a>>,
    },
    BinaryOperator {
        kind: BinaryOperatorKind,
        lhs: Arc<ExpressionNode<'a>>,
        rhs: Arc<ExpressionNode<'a>>,
    },
    UnaryOperator {
        kind: UnaryOperatorKind<'a>,
        operand: Arc<ExpressionNode<'a>>,
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
pub enum UnaryOperatorKind<'a> {
    LogicalNot,                            // !operand
    BitwiseNot,                            // ~operand
    Reference,                             // &operand
    Dereference,                           // *operand
    Cast(Type),                            // (type-name)operand
    FunctionCall(Vec<ExpressionNode<'a>>), // operand(parameter-lisr)
}

#[derive(Debug)]
pub enum Immediate<'a> {
    String(&'a str),
    Char(char),
    Number(u32),
}

#[derive(Debug)]
pub struct Type {}

//parser body
pub fn expression<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut current = assign(context)?;
    loop {
        if consume_punctuator(context, PunctuatorKind::Commma).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Comma,
                lhs: Arc::new(current),
                rhs: Arc::new(assign(context)?),
            };
        } else {
            break;
        }
    }

    Ok(current)
}

fn assign<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut current = conditional(context)?;
    loop {
        if consume_punctuator(context, PunctuatorKind::Equal).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Assign,
                lhs: Arc::new(current),
                rhs: Arc::new(conditional(context)?),
            };
        } else {
            break;
        }
    }

    Ok(current)
}

fn conditional<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let condition = logical_or(context)?;
    let result = if consume_punctuator(context, PunctuatorKind::Question).is_some() {
        let second = expression(context)?;
        expect_punctuator(context, PunctuatorKind::Colon)?;
        ExpressionNode::TernaryOperator {
            kind: TernaryOperatorKind::Conditional,
            first: Arc::new(condition),
            second: Arc::new(second),
            third: Arc::new(conditional(context)?),
        }
    } else {
        condition
    };

    Ok(result)
}

fn logical_or<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut current = logical_and(context)?;
    let result = loop {
        if consume_punctuator(context, PunctuatorKind::DoublePipelines).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::LogicalOr,
                lhs: Arc::new(current),
                rhs: Arc::new(logical_and(context)?),
            };
        } else {
            break current;
        }
    };

    Ok(result)
}

fn logical_and<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut current = bitwise_inclusive_or(context)?;
    let result = loop {
        if consume_punctuator(context, PunctuatorKind::DoubleAmpersands).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::LogicalAnd,
                lhs: Arc::new(current),
                rhs: Arc::new(bitwise_inclusive_or(context)?),
            };
        } else {
            break current;
        }
    };

    Ok(result)
}

fn bitwise_inclusive_or<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<ExpressionNode<'b>, String> {
    let mut current = bitwise_exclusive_or(context)?;
    let result = loop {
        if consume_punctuator(context, PunctuatorKind::Pipeline).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::BitwiseOr,
                lhs: Arc::new(current),
                rhs: Arc::new(bitwise_exclusive_or(context)?),
            };
        } else {
            break current;
        }
    };

    Ok(result)
}

fn bitwise_exclusive_or<'a, 'b>(
    context: &'a mut ParseContext<'b>,
) -> Result<ExpressionNode<'b>, String> {
    let mut current = bitwise_and(context)?;
    let result = loop {
        if consume_punctuator(context, PunctuatorKind::Hat).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::BitwiseXor,
                lhs: Arc::new(current),
                rhs: Arc::new(bitwise_and(context)?),
            };
        } else {
            break current;
        }
    };

    Ok(result)
}

fn bitwise_and<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut current = equality(context)?;
    let result = loop {
        if consume_punctuator(context, PunctuatorKind::Ampersand).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::BitwiseAnd,
                lhs: Arc::new(current),
                rhs: Arc::new(equality(context)?),
            };
        } else {
            break current;
        }
    };

    Ok(result)
}

fn equality<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut current = relational(context)?;
    let result = loop {
        if consume_punctuator(context, PunctuatorKind::DoubleEquals).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Equal,
                lhs: Arc::new(current),
                rhs: Arc::new(relational(context)?),
            };
        } else if consume_punctuator(context, PunctuatorKind::ExclamationEqual).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::NotEqual,
                lhs: Arc::new(current),
                rhs: Arc::new(relational(context)?),
            };
        } else {
            break current;
        }
    };

    Ok(result)
}

fn relational<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut current = shift(context)?;
    let result = loop {
        if consume_punctuator(context, PunctuatorKind::LessThan).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::LessThan,
                lhs: Arc::new(current),
                rhs: Arc::new(shift(context)?),
            };
        } else if consume_punctuator(context, PunctuatorKind::GreaterThan).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::LessThan,
                lhs: Arc::new(shift(context)?),
                rhs: Arc::new(current),
            };
        } else if consume_punctuator(context, PunctuatorKind::LessThanEqual).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::LessThanEqual,
                lhs: Arc::new(current),
                rhs: Arc::new(shift(context)?),
            };
        } else if consume_punctuator(context, PunctuatorKind::GreaterThanEqual).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::LessThanEqual,
                lhs: Arc::new(shift(context)?),
                rhs: Arc::new(current),
            };
        } else {
            break current;
        }
    };

    Ok(result)
}

fn shift<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut current = add(context)?;
    let result = loop {
        if consume_punctuator(context, PunctuatorKind::DoubleLessThans).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::LeftShift,
                lhs: Arc::new(current),
                rhs: Arc::new(add(context)?),
            };
        } else if consume_punctuator(context, PunctuatorKind::DoubleGreaterThans).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::RightShift,
                lhs: Arc::new(current),
                rhs: Arc::new(add(context)?),
            };
        } else {
            break current;
        }
    };

    Ok(result)
}

fn add<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut current = multiply(context)?;
    let result = loop {
        if consume_punctuator(context, PunctuatorKind::Plus).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Add,
                lhs: Arc::new(current),
                rhs: Arc::new(multiply(context)?),
            };
        } else if consume_punctuator(context, PunctuatorKind::Minus).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Sub,
                lhs: Arc::new(current),
                rhs: Arc::new(multiply(context)?),
            };
        } else {
            break current;
        }
    };

    Ok(result)
}

fn multiply<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut current = unary(context)?;
    let result = loop {
        if consume_punctuator(context, PunctuatorKind::Star).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Mul,
                lhs: Arc::new(current),
                rhs: Arc::new(unary(context)?),
            };
        } else if consume_punctuator(context, PunctuatorKind::Slash).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Div,
                lhs: Arc::new(current),
                rhs: Arc::new(unary(context)?),
            };
        } else if consume_punctuator(context, PunctuatorKind::Percent).is_some() {
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Mod,
                lhs: Arc::new(current),
                rhs: Arc::new(unary(context)?),
            };
        } else {
            break current;
        }
    };

    Ok(result)
}

fn unary<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut stack = Vec::<UnaryOperatorKind>::new();
    loop {
        if consume_punctuator(context, PunctuatorKind::Exclamation).is_some() {
            stack.push(UnaryOperatorKind::LogicalNot);
        } else if consume_punctuator(context, PunctuatorKind::Tilde).is_some() {
            stack.push(UnaryOperatorKind::BitwiseNot);
        } else if consume_punctuator(context, PunctuatorKind::Ampersand).is_some() {
            stack.push(UnaryOperatorKind::Reference);
        } else if consume_punctuator(context, PunctuatorKind::Star).is_some() {
            stack.push(UnaryOperatorKind::Dereference);
        //} else if consume_punctuator(context, PunctuatorKind::LeftRoundBracket).is_some() {
        //    let typeName = Type {};
        //    stack.push(UnaryOperatorKind::Cast(typeName));
        } else {
            break;
        }
    }

    let mut current = postfix(context)?;
    while !stack.is_empty() {
        let operator = stack.pop().unwrap();
        current = ExpressionNode::UnaryOperator {
            kind: operator,
            operand: Arc::new(current),
        };
    }

    Ok(current)
}

fn postfix<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let mut current = primary(context)?;

    let result = loop {
        if consume_punctuator(context, PunctuatorKind::LeftRoundBracket).is_some() {
            let mut parameters = Vec::new();
            if consume_punctuator(context, PunctuatorKind::RightRoundBracket).is_none() {
                loop {
                    parameters.push(assign(context)?);
                    if consume_punctuator(context, PunctuatorKind::Commma).is_none() {
                        break;
                    }
                }
            }
            current = ExpressionNode::UnaryOperator {
                kind: UnaryOperatorKind::FunctionCall(parameters),
                operand: Arc::new(current),
            };
        } else if consume_punctuator(context, PunctuatorKind::LeftSquareBracket).is_some() {
            let index = expression(context)?;
            expect_punctuator(context, PunctuatorKind::RightSquareBracket)?;
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Add,
                lhs: Arc::new(current),
                rhs: Arc::new(index),
            };
        } else if consume_punctuator(context, PunctuatorKind::DoublePluses).is_some() {
            let current_arc = Arc::new(current);
            let add = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Add,
                lhs: Arc::clone(&current_arc),
                rhs: Arc::new(ExpressionNode::Immediate(Immediate::Number(1))),
            };
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Assign,
                lhs: Arc::clone(&current_arc),
                rhs: Arc::new(add),
            };
        } else if consume_punctuator(context, PunctuatorKind::DoubleMinuses).is_some() {
            let current_arc = Arc::new(current);
            let sub = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Sub,
                lhs: Arc::clone(&current_arc),
                rhs: Arc::new(ExpressionNode::Immediate(Immediate::Number(1))),
            };
            current = ExpressionNode::BinaryOperator {
                kind: BinaryOperatorKind::Assign,
                lhs: Arc::clone(&current_arc),
                rhs: Arc::new(sub),
            };
        } else {
            break current;
        }
    };

    Ok(result)
}

fn primary<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    match consume_punctuator(context, PunctuatorKind::LeftRoundBracket) {
        Some(_) => {
            let result = expression(context);
            expect_punctuator(context, PunctuatorKind::RightRoundBracket)?;
            result
        }
        None => literal(context),
    }
}

fn literal<'a, 'b>(context: &'a mut ParseContext<'b>) -> Result<ExpressionNode<'b>, String> {
    let content = expect_literal(context)?;

    let result = match content {
        Literal::String(content) => Immediate::String(content),
        Literal::Char(content) => Immediate::Char(*content),
        Literal::Number(content) => Immediate::Number(*content),
    };

    Ok(ExpressionNode::Immediate(result))
}
