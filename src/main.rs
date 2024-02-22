mod base;
use crate::base::tokenize::tokenize;

fn main() {
    let code = r#"
        var x 20
        if $x > 0 {
            log "hi"
        } else {
            
        }

        stdout.log $x
    "#;

    let tokens = tokenize(code);
    for token in tokens {
        println!("Type: {}, Value: {}", token.token_type, token.value);
    }
}