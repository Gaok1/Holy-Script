use holy_script::lexer::tokenize;
use holy_script::parser::Parser;
use holy_script::ast::Program;
use holy_script::interpreter::{Interpreter, Value};

pub fn parse(source: &str) -> Program {
    Parser::new(tokenize(source))
        .parse_program()
        .unwrap_or_else(|e| panic!("parse error: {}", e))
}

pub fn run(source: &str) -> Interpreter {
    let program = parse(source);
    let mut interp = Interpreter::new();
    interp.run(&program).unwrap_or_else(|e| panic!("runtime error: {}", e));
    interp
}

pub fn run_err(source: &str) -> String {
    let program = parse(source);
    let mut interp = Interpreter::new();
    interp.run(&program).expect_err("expected runtime error").to_string()
}

pub fn get_str(interp: &Interpreter, name: &str) -> String {
    match interp.env_get(name) {
        Some(Value::Str(s)) => s,
        other => panic!("expected word for '{}', got {:?}", name, other),
    }
}

pub fn get_int(interp: &Interpreter, name: &str) -> i64 {
    match interp.env_get(name) {
        Some(Value::Int(n)) => n,
        other => panic!("expected atom for '{}', got {:?}", name, other),
    }
}

pub fn get_bool(interp: &Interpreter, name: &str) -> bool {
    match interp.env_get(name) {
        Some(Value::Bool(b)) => b,
        other => panic!("expected dogma for '{}', got {:?}", name, other),
    }
}
