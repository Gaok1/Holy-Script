use std::env;
use std::io::IsTerminal;
use std::sync::OnceLock;

use holy_script::parser::ParseError;

static FORCE_COLOR_FLAG: OnceLock<bool> = OnceLock::new();

/// Call once at startup with the value of the `--color` CLI flag.
pub fn init_color(force: bool) {
    FORCE_COLOR_FLAG.set(force).ok();
}

fn use_color() -> bool {
    if env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if *FORCE_COLOR_FLAG.get().unwrap_or(&false) {
        return true;
    }
    if env::var_os("FORCE_COLOR").is_some() || env::var_os("CLICOLOR_FORCE").is_some() {
        return true;
    }
    std::io::stderr().is_terminal()
}

// ── ANSI helpers ──────────────────────────────────────────────────────────────

fn ansi(code: &str, text: &str) -> String {
    if use_color() { format!("\x1b[{}m{}\x1b[0m", code, text) } else { text.to_string() }
}

pub fn red_bold(text: &str) -> String { ansi("1;31", text) }
pub fn red(text: &str)      -> String { ansi("31",   text) }
pub fn yellow(text: &str)   -> String { ansi("33",   text) }
pub fn gray(text: &str)     -> String { ansi("90",   text) }
pub fn bold(text: &str)     -> String { ansi("1",    text) }

// ── Diagnostic reporting ──────────────────────────────────────────────────────

pub fn report_parse_error(source: &str, e: &ParseError) {
    eprintln!("\n{} — line {}, column {}:", red_bold("O Heresy!"), e.line, e.col);

    if e.line > 0 {
        if let Some(line_src) = source.lines().nth(e.line - 1) {
            eprintln!("  {} | {}", gray(&format!("{:>4}", e.line)), line_src);
            let arrow_pad = e.col.saturating_sub(1);
            eprintln!("       {}", red_bold(&format!("{}^", " ".repeat(arrow_pad))));
        }
    }

    eprintln!("  {}\n", yellow(&e.message));
}
