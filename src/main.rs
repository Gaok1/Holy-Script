mod reporter;

use std::{env, fs, process};

use holy_script::interpreter::Interpreter;
use holy_script::lexer::{Token, tokenize};
use holy_script::parser::Parser;
use holy_script::tree;

use reporter::{bold, gray, red_bold, report_parse_error};

fn main() {
    let args: Vec<String> = env::args().collect();

    let force_color = args.iter().any(|a| a == "--color");
    reporter::init_color(force_color);

    let show_tree = args.iter().any(|a| a == "--tree" || a == "-t");
    let file = args.iter().skip(1).find(|a| !a.starts_with('-')).unwrap_or_else(|| {
        eprintln!("Usage: holy [--tree] [--color] <file.holy>");
        process::exit(1);
    });

    let source = fs::read_to_string(file).unwrap_or_else(|e| {
        eprintln!("The sacred scroll '{}' could not be unsealed: {}", file, e);
        process::exit(1);
    });

    let tokens = tokenize(&source);

    validate_amen(&source, &tokens);

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap_or_else(|e| {
        report_parse_error(&source, &e);
        process::exit(1);
    });

    if show_tree {
        tree::print_program(&program);
        return;
    }

    // Collect everything after the script filename as script arguments.
    let file_pos = args.iter().skip(1).position(|a| !a.starts_with('-')).unwrap_or(0) + 1;
    let script_args = args.get(file_pos + 1..).unwrap_or(&[]).to_vec();

    let source_dir = std::path::Path::new(file)
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .to_path_buf();

    let mut interp = Interpreter::new()
        .with_script_args(script_args)
        .with_source_dir(source_dir);

    if let Err(e) = interp.run(&program) {
        eprintln!("{}: {}", reporter::red("O Profanation!"), e);
        process::exit(1);
    }
}

fn validate_amen(source: &str, tokens: &[holy_script::lexer::Spanned]) {
    let amens: Vec<_> = tokens.iter().filter(|s| s.token == Token::Amen).collect();

    match amens.len() {
        0 => {
            eprintln!(
                "\n{}: every holy scripture must be sealed with {}\n",
                red_bold("Blasphemy!"),
                bold("amen"),
            );
            process::exit(1);
        }
        1 => {
            let last_meaningful = tokens.iter().rev().find(|s| s.token != Token::Eof);
            if last_meaningful.map(|s| &s.token) != Some(&Token::Amen) {
                eprintln!(
                    "\n{}: every holy scripture must be sealed with {}\n",
                    red_bold("Blasphemy!"),
                    bold("amen"),
                );
                process::exit(1);
            }
        }
        n => {
            eprintln!(
                "\n{}: the sacred seal 'amen' was spoken {} times — only one may close the scripture:",
                red_bold("Blasphemy!"),
                n,
            );
            for sp in &amens {
                if let Some(line_src) = source.lines().nth(sp.line - 1) {
                    eprintln!("  {} | {}", gray(&format!("{:>4}", sp.line)), line_src);
                    eprintln!("       {}", red_bold(&format!("{}^", " ".repeat(sp.col.saturating_sub(1)))));
                }
            }
            eprintln!();
            process::exit(1);
        }
    }
}
