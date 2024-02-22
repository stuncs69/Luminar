use std::collections::HashMap;

use crate::base::enums;
use crate::lib::main::{self, LibraryFunction};

fn execute_func_call(func_name: &str, args: Vec<&str>) {
    let lib = main::LibLibrary;
    println!("Hello, test");
    lib.call(func_name, args);
}

pub fn runtime(tokens: Vec<enums::Token>) {
    let mut variables: HashMap<String, String> = HashMap::new();

    for (index, token) in tokens.iter().enumerate() {
        match token.token_type {
            enums::TokenType::FuncCall => {
                let mut args = Vec::new();
                args.push("Ok");
                execute_func_call(&token.value, args)
            }
            enums::TokenType::Keyword => {
                // Ensure there are enough tokens to represent a variable definition (Keyword, Identifier, Literal)
                if let Some(identifier_token) = tokens.get(index + 1) {
                    if let Some(literal_token) = tokens.get(index + 2) {
                        // Extract variable name and value from Identifier and Literal tokens
                        let var_name = &identifier_token.value;
                        let literal = &literal_token.value;
                        println!("New var: {} | {}", var_name, literal);
                        if literal.starts_with("&") {

                        }
                        variables.insert(var_name.to_string(), literal.to_string());
                    } else {
                        println!("Error: Missing literal token after keyword");
                    }
                } else {
                    println!("Error: Missing identifier token after keyword");
                }
            }
            
            _ => {
            }
        }
    }
}
