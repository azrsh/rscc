mod parser;
mod tokenizer;
use crate::parser::parse;
use crate::tokenizer::tokenize;

fn main() {
    compile("0");
}

fn compile(source: &str) {
    let tokens = match tokenize(source) {
        Ok(result) => result,
        Err(message) => {
            println!("Tokenization Error: {}", message);
            return;
        }
    };

    let ast = match parse(&tokens) {
        Ok(result) => result,
        Err(message) => {
            println!("Parse Error: {}", message);
            return;
        }
    };

    println!("{:?}", ast)
}
