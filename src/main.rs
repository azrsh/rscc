mod tokenizer;
use crate::tokenizer::tokenize;

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

    for token in tokens {
        println!("{}", token);
    }
}
