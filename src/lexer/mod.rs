mod token;

pub use token::{token_name, Spanned, Token};

use token::Spanned as S;

/// Converts source code into a flat list of positioned tokens.
/// Blank lines and `--` comments are skipped.
/// Indentation changes are emitted as `Indent` / `Dedent` tokens.
pub fn tokenize(source: &str) -> Vec<Spanned> {
    let mut result       = Vec::new();
    let mut indent_stack = vec![0usize];

    for (line_idx, line) in source.lines().enumerate() {
        let line_num = line_idx + 1;
        let content  = line.trim();

        if content.is_empty() || content.starts_with("--") {
            continue;
        }

        let indent_count = leading_whitespace(line);
        let current      = *indent_stack.last().unwrap();

        if indent_count > current {
            indent_stack.push(indent_count);
            result.push(S::new(Token::Indent, line_num, indent_count + 1));
        } else if indent_count < current {
            while indent_stack.len() > 1 && *indent_stack.last().unwrap() > indent_count {
                indent_stack.pop();
                result.push(S::new(Token::Dedent, line_num, indent_count + 1));
            }
        }

        tokenize_line(line, line_num, &mut result);
    }

    // Close any open indentation blocks left at end-of-file
    while indent_stack.len() > 1 {
        indent_stack.pop();
        result.push(S::new(Token::Dedent, 0, 0));
    }

    result.push(S::new(Token::Eof, 0, 0));
    result
}

fn leading_whitespace(line: &str) -> usize {
    line.chars().take_while(|c| *c == ' ' || *c == '\t').count()
}

fn tokenize_line(line: &str, line_num: usize, result: &mut Vec<Spanned>) {
    let mut chars = line.chars().peekable();
    let mut col   = 1usize;

    while let Some(&c) = chars.peek() {
        let tok_col = col;
        match c {
            ' ' | '\t' => { chars.next(); col += 1; }

            '-' => {
                chars.next(); col += 1;
                if chars.peek() == Some(&'-') {
                    break; // line comment — stop processing this line
                }
                // Negative numeric literal: `-1`, `-3.14`
                if chars.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                    let tok = scan_number(&mut chars, &mut col, true);
                    result.push(S::new(tok, line_num, tok_col));
                }
            }

            ',' => { chars.next(); col += 1; result.push(S::new(Token::Comma, line_num, tok_col)); }

            '"' => {
                chars.next(); col += 1;
                let mut s = String::new();
                for c in chars.by_ref() {
                    col += 1;
                    if c == '"' { break; }
                    s.push(c);
                }
                result.push(S::new(Token::StrLit(s), line_num, tok_col));
            }

            '0'..='9' => {
                let tok = scan_number(&mut chars, &mut col, false);
                result.push(S::new(tok, line_num, tok_col));
            }

            c if c.is_alphabetic() || c == '_' => {
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' { word.push(c); chars.next(); col += 1; }
                    else { break; }
                }
                result.push(S::new(keyword_or_ident(word), line_num, tok_col));
            }

            _ => { chars.next(); col += 1; }
        }
    }
}

fn scan_number(
    chars:    &mut std::iter::Peekable<std::str::Chars>,
    col:      &mut usize,
    negative: bool,
) -> Token {
    let mut num     = if negative { String::from("-") } else { String::new() };
    let mut has_dot = false;

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            num.push(c); chars.next(); *col += 1;
        } else if c == '.' && !has_dot {
            has_dot = true;
            num.push(c); chars.next(); *col += 1;
        } else {
            break;
        }
    }

    if has_dot {
        Token::FloatLit(num.parse().unwrap_or(0.0))
    } else {
        Token::IntLit(num.parse().unwrap_or(0))
    }
}

fn keyword_or_ident(word: String) -> Token {
    match word.as_str() {
        "testament"  => Token::Testament,
        "revealing"  => Token::Revealing,
        "scripture"  => Token::Scripture,
        "sin"        => Token::Sin,
        "covenant"   => Token::Covenant,
        "salm"       => Token::Salm,
        "upon"       => Token::Upon,
        "receiving"  => Token::Receiving,
        "reveals"    => Token::Reveals,
        "let"        => Token::Let,
        "there"      => Token::There,
        "be"         => Token::Be,
        "of"         => Token::Of,
        "become"     => Token::Become,
        "hail"       => Token::Hail,
        "praying"    => Token::Praying,
        "reveal"     => Token::Reveal,
        "whether"    => Token::Whether,
        "otherwise"  => Token::Otherwise,
        "so"         => Token::So,
        "litany"     => Token::Litany,
        "for"        => Token::For,
        "confess"    => Token::Confess,
        "answer"     => Token::Answer,
        "absolve"    => Token::Absolve,
        "as"         => Token::As,
        "transgress" => Token::Transgress,
        "manifest"   => Token::Manifest,
        "from"       => Token::From,
        "its"        => Token::Its,
        "discern"    => Token::Discern,
        "amen"       => Token::Amen,
        "forsake"    => Token::Forsake,
        "ascend"     => Token::Ascend,
        "bearing"    => Token::Bearing,
        "after"      => Token::After,
        "thus"       => Token::Thus,
        "plus"       => Token::Plus,
        "minus"      => Token::Minus,
        "times"      => Token::Times,
        "over"       => Token::Over,
        "remainder"  => Token::Remainder,
        "negate"     => Token::Negate,
        "is"         => Token::Is,
        "not"        => Token::Not,
        "greater"    => Token::Greater,
        "lesser"     => Token::Lesser,
        "than"       => Token::Than,
        "no"         => Token::No,
        "blessed"    => Token::Blessed,
        "forsaken"   => Token::Forsaken,
        "and"        => Token::And,
        "void"       => Token::Void,
        "atom"       => Token::Atom,
        "fractional" => Token::Fractional,
        "word"       => Token::Word,
        "dogma"      => Token::Dogma,
        _            => Token::Ident(word),
    }
}
