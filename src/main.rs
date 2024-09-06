mod base;
mod lib;
use crate::lib::main;

fn main() {
    let code = r#"
            var x 20
            if $x > 10 {
                printf $x
            }
    "#;

    let tokens = base::tokenize::tokenize(code);
    base::runtime::runtime(tokens);
}