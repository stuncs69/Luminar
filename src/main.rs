mod base;
use crate::base::tokenize::tokenize;

fn main() {
    let code = r#"
        if $x > 0 {
            log "hi"
        } else {
            return "hi"
        }
    "#;

    let tokens = tokenize(code);
    for token in tokens {
        println!("Type: {}, Value: {}", token.token_type, token.value);
    }
}