mod base;
mod lib;
use crate::lib::main;

fn main() {
    let code = r#"
        var x 20
        printf 20
    "#;

    let tokens = base::tokenize::tokenize(code);
    base::runtime::runtime(tokens);
    for token in base::tokenize::tokenize(code) {
        println!("{}, {}", token.token_type.to_string(), token.value.to_string())
    }
}