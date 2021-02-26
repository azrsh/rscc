use std::fmt;

#[derive(Debug)]
pub enum PunctuatorKind {
    LeftSquareBracket,
    RightSquareBracket,
    LeftRoundBracket,
    RightRoundBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Dot,
    Arrow,
    DoublePluses,
    DoubleMinuses,
    Ampersand,
    Star,
    Plus,
    Minus,
    Tilde,
    Exclamation,
    Slash,
    Percent,
    DoubleGreaterThans,
    DoubleLessThans,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    DoubleEquals,
    ExclamationEqual,
    Hat,
    Pipeline,
    DoubleAmpersands,
    DoublePipelines,
    Question,
    Colon,
    Semicolon,
    TripleDots,
    Equal,
    StarEqual,
    SlashEqual,
    PercentEqual,
    PlusEqual,
    MinusEqual,
    DoubleGreaterThansEqual,
    DoubleLessThansEqual,
    AmpersandEqual,
    HatEqual,
    PipelineEqual,
    Commma,
    Sharp,
    DoubleSharps,
    LessThanColon,
    ColonGreaterThan,
    LessThanPercent,
    PercentGreaterThan,
    PercentColon,
    DoublePercentColons,
}

#[derive(Debug)]
pub enum KeywordKind {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    _Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
    Alignas,
    Alignof,
    Atomic,
    Bool,
    Complex,
    Generic,
    Imaginary,
    Noreturn,
    StaticAssert,
    ThreadLocal,
}

pub enum Literal<'a> {
    String(&'a str),
    Char(char),
    Number(i32),
}

impl fmt::Display for  Literal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "String:{}", s),
            Literal::Char(c) => write!(f, "Char:{}", c),
            Literal::Number(n) => write!(f, "Number:{}", n),
        }
    }
}

pub enum Token<'a> {
    Punctuator(PunctuatorKind),
    Keyword(KeywordKind),
    Identifier(&'a str),
    Literal(Literal<'a>),
    End,
}

impl fmt::Display for  Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Punctuator(kind) => write!(f, "Punctuator:{:?}", kind),
            Token::Keyword(kind) => write!(f, "Keyword:{:?}", kind),
            Token::Identifier(s) => write!(f, "Identifier:{}", s),
            Token::Literal(s) => write!(f, "Literal:{}", s),
            Token::End => write!(f, "End"),
        }
    }
}

struct TokenizationContext<'a> {
    head: &'a str,
}

//context.head must be greater than or equal count
fn consume_str(context: &mut TokenizationContext, count: usize) -> () {
    assert!(context.head.len() >= count);
    context.head = &context.head[count..];
}

fn skip_whitespace(context: &mut TokenizationContext) -> () {
    let mut count = 0;
    for c in context.head.chars() {
        if !c.is_whitespace() {
            break;
        }
        count += 1;
    }   
    consume_str(context, count);
}

fn consume_reserved(context: &mut TokenizationContext, q: &str) -> bool {
    if context.head.starts_with(q) {
        consume_str(context, q.len());
        true
    } else {
        false
    }
}

fn is_identifier_head(p: char) -> bool {
    p == '_' || p.is_ascii_alphabetic()
}

fn is_identifier_body(p: char) -> bool {
    is_identifier_head(p) || p.is_numeric()
}

fn count_identifier_length(head: &str) -> usize {
    let mut chars = head.chars();
    match chars.nth(0) {
        Some(c) if is_identifier_head(c) => {},
        _ => { return 0; },
    }
    
    let mut count = 1;
    for c in chars {
        if !is_identifier_body(c) {
            break;
        }
        count+= 1;
    }
    count
}

fn consume_identifier<'a, 'b>(context: &'a mut TokenizationContext<'b>) -> Option<&'b str> {
    match count_identifier_length(context.head) {
        n if n > 0 => {
            let result = &context.head[..n];
            consume_str(context, n);
            Some(result)
        },
        _ => None,
    }
}

fn consume_literal<'a, 'b>(context: &'a mut TokenizationContext<'b>) -> Option<Literal<'b>> {
    let mut chars = context.head.chars();
    match chars.nth(0) {
        Some(c) if c == '\"' => {
            let mut count = 0;
            for c in chars {
                match c {
                    c if c == '\"' => { break; },
                    _ => {}, 
                }
                count += 1;
            }
            let result = &context.head[1..count - 1];
            consume_str(context, count);
            Some(Literal::String(result))
        },
        Some(c) if c == '\'' => {
            let result = chars.nth(0).unwrap();
            consume_str(context, 3);
            Some(Literal::Char(result))
        },
        Some(c) if c.is_digit(10) => {
            let mut count = 1;
            for c in chars {
                if !c.is_digit(10) {
                    break;
                }
                count += 1;
            }
            let result = &context.head[..count];
            let result = result.parse::<i32>();
            consume_str(context, count);
            match result {
                Ok(n) => Some(Literal::Number(n)),
                Err(_) => None,
            }
        },
        _ => None,
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, &str> {
    let mut result =  Vec::<Token>::new();
    let context = &mut TokenizationContext { head: source };
    loop {
        if context.head.len() == 0 {
            break;
        }

        skip_whitespace(context);

        let item: Token = if consume_reserved(context, "%:%:") {
            Token::Punctuator(PunctuatorKind::DoublePercentColons)
        } else if consume_reserved(context, "...") {
            Token::Punctuator(PunctuatorKind::TripleDots)
        } else if consume_reserved(context, "<<=") {
            Token::Punctuator(PunctuatorKind::DoubleLessThansEqual)
        } else if consume_reserved(context, ">>=") {
            Token::Punctuator(PunctuatorKind::DoubleGreaterThansEqual)
        } else if consume_reserved(context, "->") {
            Token::Punctuator(PunctuatorKind::Arrow)
        } else if consume_reserved(context, "++") {
            Token::Punctuator(PunctuatorKind::DoublePluses)
        } else if consume_reserved(context, "--") {
            Token::Punctuator(PunctuatorKind::DoubleMinuses)
        } else if consume_reserved(context, "<<") {
            Token::Punctuator(PunctuatorKind::DoubleLessThans)
        } else if consume_reserved(context, ">>") {
            Token::Punctuator(PunctuatorKind::DoubleGreaterThans)
        } else if consume_reserved(context, "<=") {
            Token::Punctuator(PunctuatorKind::LessThanEqual)
        } else if consume_reserved(context, ">=") {
            Token::Punctuator(PunctuatorKind::GreaterThanEqual)
        } else if consume_reserved(context, "==") {
            Token::Punctuator(PunctuatorKind::DoubleEquals)
        } else if consume_reserved(context, "!=") {
            Token::Punctuator(PunctuatorKind::ExclamationEqual)
        } else if consume_reserved(context, "&&") {
            Token::Punctuator(PunctuatorKind::DoubleAmpersands)
        } else if consume_reserved(context, "||") {
            Token::Punctuator(PunctuatorKind::DoublePipelines)
        } else if consume_reserved(context, "*=") {
            Token::Punctuator(PunctuatorKind::StarEqual)
        } else if consume_reserved(context, "/=") {
            Token::Punctuator(PunctuatorKind::SlashEqual)
        } else if consume_reserved(context, "%=") {
            Token::Punctuator(PunctuatorKind::PercentEqual)
        } else if consume_reserved(context, "+=") {
            Token::Punctuator(PunctuatorKind::PlusEqual)
        } else if consume_reserved(context, "-=") {
            Token::Punctuator(PunctuatorKind::MinusEqual)
        } else if consume_reserved(context, "&=") {
            Token::Punctuator(PunctuatorKind::AmpersandEqual)
        } else if consume_reserved(context, "^=") {
            Token::Punctuator(PunctuatorKind::HatEqual)
        } else if consume_reserved(context, "|=") {
            Token::Punctuator(PunctuatorKind::PipelineEqual)
        } else if consume_reserved(context, "##") {
            Token::Punctuator(PunctuatorKind::DoubleSharps)
        } else if consume_reserved(context, "<:") {
            Token::Punctuator(PunctuatorKind::LessThanColon)
        } else if consume_reserved(context, ":>") {
            Token::Punctuator(PunctuatorKind::ColonGreaterThan)
        } else if consume_reserved(context, "<%") {
            Token::Punctuator(PunctuatorKind::LessThanPercent)
        } else if consume_reserved(context, "%>") {
            Token::Punctuator(PunctuatorKind::PercentGreaterThan)
        } else if consume_reserved(context, "%:") {
            Token::Punctuator(PunctuatorKind::PercentColon)
        } else if consume_reserved(context, "%:") {
            Token::Punctuator(PunctuatorKind::PercentColon)
        } else if consume_reserved(context, "[") {
            Token::Punctuator(PunctuatorKind::LeftSquareBracket)
        } else if consume_reserved(context, "]") {
            Token::Punctuator(PunctuatorKind::RightSquareBracket)
        } else if consume_reserved(context, "(") {
            Token::Punctuator(PunctuatorKind::LeftRoundBracket)
        } else if consume_reserved(context, ")") {
            Token::Punctuator(PunctuatorKind::RightRoundBracket)
        } else if consume_reserved(context, "{") {
            Token::Punctuator(PunctuatorKind::LeftCurlyBracket)
        } else if consume_reserved(context, "}") {
            Token::Punctuator(PunctuatorKind::RightCurlyBracket)
        } else if consume_reserved(context, ".") {
            Token::Punctuator(PunctuatorKind::Dot)
        } else if consume_reserved(context, "&") {
            Token::Punctuator(PunctuatorKind::Ampersand)
        } else if consume_reserved(context, "*") {
            Token::Punctuator(PunctuatorKind::Star)
        } else if consume_reserved(context, "+") {
            Token::Punctuator(PunctuatorKind::Plus)
        } else if consume_reserved(context, "-") {
            Token::Punctuator(PunctuatorKind::Minus)
        } else if consume_reserved(context, "~") {
            Token::Punctuator(PunctuatorKind::Tilde)
        } else if consume_reserved(context, "!") {
            Token::Punctuator(PunctuatorKind::Exclamation)
        } else if consume_reserved(context, "/") {
            Token::Punctuator(PunctuatorKind::Slash)
        } else if consume_reserved(context, "%") {
            Token::Punctuator(PunctuatorKind::Percent)
        } else if consume_reserved(context, "<") {
            Token::Punctuator(PunctuatorKind::LessThan)
        } else if consume_reserved(context, ">") {
            Token::Punctuator(PunctuatorKind::GreaterThan)
        } else if consume_reserved(context, "^") {
            Token::Punctuator(PunctuatorKind::Hat)
        } else if consume_reserved(context, "|") {
            Token::Punctuator(PunctuatorKind::Pipeline)
        } else if consume_reserved(context, "?") {
            Token::Punctuator(PunctuatorKind::Question)
        } else if consume_reserved(context, ":") {
            Token::Punctuator(PunctuatorKind::Colon)
        } else if consume_reserved(context, ";") {
            Token::Punctuator(PunctuatorKind::Semicolon)
        } else if consume_reserved(context, "=") {
            Token::Punctuator(PunctuatorKind::Equal)
        } else if consume_reserved(context, ",") {
            Token::Punctuator(PunctuatorKind::Commma)
        } else if consume_reserved(context, "#") {
            Token::Punctuator(PunctuatorKind::Sharp)
        } else if consume_reserved(context, "auto") {
            Token::Keyword(KeywordKind::Auto)
        } else if consume_reserved(context, "break") {
            Token::Keyword(KeywordKind::Break)
        } else if consume_reserved(context, "case") {
            Token::Keyword(KeywordKind::Case)
        } else if consume_reserved(context, "char") {
            Token::Keyword(KeywordKind::Char)
        } else if consume_reserved(context, "const") {
            Token::Keyword(KeywordKind::Const)
        } else if consume_reserved(context, "continue") {
            Token::Keyword(KeywordKind::Continue)
        } else if consume_reserved(context, "default") {
            Token::Keyword(KeywordKind::_Default)
        } else if consume_reserved(context, "do") {
            Token::Keyword(KeywordKind::Do)
        } else if consume_reserved(context, "double") {
            Token::Keyword(KeywordKind::Double)
        } else if consume_reserved(context, "else") {
            Token::Keyword(KeywordKind::Else)
        } else if consume_reserved(context, "enum") {
            Token::Keyword(KeywordKind::Enum)
        } else if consume_reserved(context, "extern") {
            Token::Keyword(KeywordKind::Extern)
        } else if consume_reserved(context, "float") {
            Token::Keyword(KeywordKind::Float)
        } else if consume_reserved(context, "for") {
            Token::Keyword(KeywordKind::For)
        } else if consume_reserved(context, "goto") {
            Token::Keyword(KeywordKind::Goto)
        } else if consume_reserved(context, "if") {
            Token::Keyword(KeywordKind::If)
        } else if consume_reserved(context, "inline") {
            Token::Keyword(KeywordKind::Inline)
        } else if consume_reserved(context, "int") {
            Token::Keyword(KeywordKind::Int)
        } else if consume_reserved(context, "long") {
            Token::Keyword(KeywordKind::Long)
        } else if consume_reserved(context, "register") {
            Token::Keyword(KeywordKind::Register)
        } else if consume_reserved(context, "restrict") {
            Token::Keyword(KeywordKind::Restrict)
        } else if consume_reserved(context, "return") {
            Token::Keyword(KeywordKind::Return)
        } else if consume_reserved(context, "short") {
            Token::Keyword(KeywordKind::Short)
        } else if consume_reserved(context, "signed") {
            Token::Keyword(KeywordKind::Signed)
        } else if consume_reserved(context, "sizeof") {
            Token::Keyword(KeywordKind::Sizeof)
        } else if consume_reserved(context, "static") {
            Token::Keyword(KeywordKind::Static)
        } else if consume_reserved(context, "struct") {
            Token::Keyword(KeywordKind::Struct)
        } else if consume_reserved(context, "switch") {
            Token::Keyword(KeywordKind::Switch)
        } else if consume_reserved(context, "typedef") {
            Token::Keyword(KeywordKind::Typedef)
        } else if consume_reserved(context, "union") {
            Token::Keyword(KeywordKind::Union)
        } else if consume_reserved(context, "unsigned") {
            Token::Keyword(KeywordKind::Unsigned)
        } else if consume_reserved(context, "void") {
            Token::Keyword(KeywordKind::Void)
        } else if consume_reserved(context, "volatile") {
            Token::Keyword(KeywordKind::Volatile)
        } else if consume_reserved(context, "while") {
            Token::Keyword(KeywordKind::While)
        } else if consume_reserved(context, "_Alignas") {
            Token::Keyword(KeywordKind::Alignas)
        } else if consume_reserved(context, "_Alignof") {
            Token::Keyword(KeywordKind::Alignof)
        } else if consume_reserved(context, "_Atomic") {
            Token::Keyword(KeywordKind::Atomic)
        } else if consume_reserved(context, "_Bool") {
            Token::Keyword(KeywordKind::Bool)
        } else if consume_reserved(context, "_Complex") {
            Token::Keyword(KeywordKind::Complex)
        } else if consume_reserved(context, "_Generic") {
            Token::Keyword(KeywordKind::Generic)
        } else if consume_reserved(context, "_Imaginary") {
            Token::Keyword(KeywordKind::Imaginary)
        } else if consume_reserved(context, "_Noreturn") {
            Token::Keyword(KeywordKind::Noreturn)
        } else if consume_reserved(context, "_Static_assert") {
            Token::Keyword(KeywordKind::StaticAssert)
        } else if consume_reserved(context, "_Thread_local") {
            Token::Keyword(KeywordKind::ThreadLocal)
        } else if let Some(identifier) = consume_identifier(context) {
            Token::Identifier(identifier)
        } else if let Some(literal) = consume_literal(context) {
            Token::Literal(literal)
        } else {
            println!("{}", context.head);
            return Err("invalid token");
        };

        result.push(item);
    }

    result.push(Token::End);
    Ok(result)
}
