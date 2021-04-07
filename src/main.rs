mod parser;
mod tokenizer;
mod util;
use crate::parser::parse;
use crate::tokenizer::tokenize;

fn main() {
    compile("if (1 > 0) { (1 + 2 + 3 - 1) * 2; }");
}

fn compile(source: &str) {
    let tokens = match tokenize(source) {
        Ok(result) => result,
        Err(message) => {
            println!("Tokenization Error: {}", message);
            return;
        }
    };

    for token in &tokens {
        println!("{:?}", token);
    }

    let ast = match parse(&tokens) {
        Ok(result) => result,
        Err(message) => {
            println!("Parse Error: {}", message);
            return;
        }
    };

    println!("{:?}", ast)
}
