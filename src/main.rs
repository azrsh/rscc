mod tokenizer;
mod parser;
use crate::tokenizer::tokenize;
use crate::parser::parse;

fn main() {
    compile("int main() { return 0; }");
}

fn compile(source: &str) {
    let tokens = match tokenize(source) {
        Ok(result) => result,
        Err(message) => {
            println!("Tokenization Error: {}", message);
            return;
        }
    };

    let ast = match parse(tokens) {
        Ok(result) => result,
        Err(message) => {
            println!("Parse Error: {}", message);
            return;
        },
    };

    println!("{}", ast)
}
