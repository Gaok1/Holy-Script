mod decls;
mod exprs;
mod stmts;

use termtree::Tree;

use crate::ast::*;

pub fn print_program(program: &Program) {
    println!("{}", build_program_tree(program));
}

fn build_program_tree(program: &Program) -> Tree<String> {
    let mut root = Tree::new("Program".into());

    for testament in &program.testaments {
        root.push(decls::build_testament_tree(testament));
    }
    for decl in &program.top_decls {
        root.push(decls::build_top_decl_tree(decl));
    }
    for stmt in &program.stmts {
        root.push(stmts::build_stmt_tree(stmt));
    }

    root
}

// ── Shared formatting helpers ─────────────────────────────────────────────────

pub(super) fn type_str(ty: &HolyType) -> String {
    match ty {
        HolyType::Atom               => "atom".into(),
        HolyType::Fractional         => "fractional".into(),
        HolyType::Word               => "word".into(),
        HolyType::Dogma              => "dogma".into(),
        HolyType::Void               => "void".into(),
        HolyType::Custom(name)       => name.clone(),
        HolyType::Generic(name, args) => {
            let args_str = args.iter().map(type_str).collect::<Vec<_>>().join(", ");
            format!("{} of {}", name, args_str)
        }
    }
}

pub(super) fn params_str(params: &[(String, HolyType)]) -> String {
    if params.is_empty() {
        return "-".into();
    }
    params.iter()
        .map(|(name, ty)| format!("{}: {}", name, type_str(ty)))
        .collect::<Vec<_>>()
        .join(", ")
}
