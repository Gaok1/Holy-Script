mod lexer;
mod ast;
mod parser;
mod interpreter;
mod tree;

use std::{env, fs, process};

use lexer::tokenize;
use parser::{Parser, ParseError};
use interpreter::Interpreter;

fn report_parse_error(source: &str, e: &ParseError) {
    eprintln!("\n\x1b[1;31msyntax error\x1b[0m — line {}, column {}:", e.line, e.col);

    if e.line > 0 {
        if let Some(line_src) = source.lines().nth(e.line - 1) {
            eprintln!("  \x1b[90m{:>4} │\x1b[0m {}", e.line, line_src);
            let arrow_pad = e.col.saturating_sub(1);
            eprintln!("       \x1b[1;31m{}^\x1b[0m", " ".repeat(arrow_pad));
        }
    }

    eprintln!("  \x1b[33m{}\x1b[0m\n", e.message);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let show_tree = args.iter().any(|a| a == "--tree" || a == "-t");
    let file = args.iter().skip(1).find(|a| !a.starts_with('-')).unwrap_or_else(|| {
        eprintln!("Usage: holy [--tree] <file.holy>");
        process::exit(1);
    });

    let source = fs::read_to_string(file).unwrap_or_else(|e| {
        eprintln!("Error reading '{}': {}", file, e);
        process::exit(1);
    });

    // Lex
    let tokens = tokenize(&source);

    // Validate 'amen'
    let amens: Vec<_> = tokens.iter().filter(|s| s.token == lexer::Token::Amen).collect();
    match amens.len() {
        0 => {
            eprintln!("\n\x1b[1;31merror\x1b[0m: every holy program must end with \x1b[1mamen\x1b[0m\n");
            process::exit(1);
        }
        1 => {} // ok
        n => {
            eprintln!("\n\x1b[1;31merror\x1b[0m: found {} 'amen' tokens — exactly one is required (at the end):", n);
            for sp in &amens {
                if let Some(line_src) = source.lines().nth(sp.line - 1) {
                    eprintln!("  \x1b[90m{:>4} │\x1b[0m {}", sp.line, line_src);
                    eprintln!("       \x1b[1;31m{}^\x1b[0m", " ".repeat(sp.col.saturating_sub(1)));
                }
            }
            eprintln!();
            process::exit(1);
        }
    }

    // Parse
    let mut p = Parser::new(tokens);
    let program = p.parse_program().unwrap_or_else(|e| {
        report_parse_error(&source, &e);
        process::exit(1);
    });

    if show_tree {
        tree::print_program(&program);
        return;
    }

    // Interpret
    let mut interp = Interpreter::new();
    if let Err(e) = interp.run(&program) {
        eprintln!("\x1b[31mruntime error:\x1b[0m {}", e);
        process::exit(1);
    }
}
