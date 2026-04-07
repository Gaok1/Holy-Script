use crate::ast::*;

// ══════════════════════════════════════════════════════════════════
// Ponto de entrada
// ══════════════════════════════════════════════════════════════════

pub fn print_program(program: &Program) {
    println!("Program");

    let total = program.testaments.len() + program.top_decls.len() + program.stmts.len();
    let mut i = 0;

    for t in &program.testaments {
        i += 1;
        print_testament(t, "", i == total);
    }
    for d in &program.top_decls {
        i += 1;
        print_top_decl(d, "", i == total);
    }
    for s in &program.stmts {
        i += 1;
        print_stmt(s, "", i == total);
    }
}

// ══════════════════════════════════════════════════════════════════
// Helpers de formatação
// ══════════════════════════════════════════════════════════════════

fn branch(last: bool) -> &'static str {
    if last { "└── " } else { "├── " }
}

fn cont(last: bool) -> &'static str {
    if last { "    " } else { "│   " }
}

fn leaf(prefix: &str, last: bool, label: &str) {
    println!("{}{}{}", prefix, branch(last), label);
}

fn child_prefix(prefix: &str, last: bool) -> String {
    format!("{}{}", prefix, cont(last))
}

/// Lista de children com callback.
fn children<T, F>(items: &[T], prefix: &str, f: F)
where
    F: Fn(&T, &str, bool),
{
    let n = items.len();
    for (i, item) in items.iter().enumerate() {
        f(item, prefix, i + 1 == n);
    }
}

fn type_str(ty: &HolyType) -> String {
    match ty {
        HolyType::Atom       => "atom".into(),
        HolyType::Fractional => "fractional".into(),
        HolyType::Word       => "word".into(),
        HolyType::Dogma      => "dogma".into(),
        HolyType::Void       => "void".into(),
        HolyType::Custom(n)  => n.clone(),
    }
}

fn params_str(params: &[(String, HolyType)]) -> String {
    if params.is_empty() { return "–".into(); }
    params.iter()
        .map(|(n, t)| format!("{}: {}", n, type_str(t)))
        .collect::<Vec<_>>()
        .join(", ")
}

// ══════════════════════════════════════════════════════════════════
// Testament
// ══════════════════════════════════════════════════════════════════

fn print_testament(t: &Testament, prefix: &str, last: bool) {
    let rev = t.revealing.as_ref()
        .map(|r| format!(" revealing {}", r.join(", ")))
        .unwrap_or_default();
    leaf(prefix, last, &format!("[testament] {}{}", t.name, rev));
}

// ══════════════════════════════════════════════════════════════════
// Top-level declarations
// ══════════════════════════════════════════════════════════════════

fn print_top_decl(d: &TopDecl, prefix: &str, last: bool) {
    match d {
        TopDecl::Scripture { name, fields } => {
            leaf(prefix, last, &format!("[scripture] {}", name));
            let cp = child_prefix(prefix, last);
            children(fields, &cp, |f, p, l| {
                leaf(p, l, &format!("{}: {}", f.0, type_str(&f.1)));
            });
        }
        TopDecl::SinDecl { name, fields } => {
            leaf(prefix, last, &format!("[sin] {}", name));
            if !fields.is_empty() {
                let cp = child_prefix(prefix, last);
                children(fields, &cp, |f, p, l| {
                    leaf(p, l, &format!("{}: {}", f.0, type_str(&f.1)));
                });
            }
        }
        TopDecl::Covenant { name, variants } => {
            leaf(prefix, last, &format!("[covenant] {}", name));
            let cp = child_prefix(prefix, last);
            children(variants, &cp, |variant, p, l| {
                leaf(p, l, variant);
            });
        }
        TopDecl::Salm { name, params, ret_type, body } => {
            leaf(prefix, last, &format!(
                "[salm] {} ({}) → {}",
                name, params_str(params), type_str(ret_type)
            ));
            let cp = child_prefix(prefix, last);
            print_block(body, &cp);
        }
        TopDecl::MethodSalm { name, target_type, params, ret_type, body } => {
            leaf(prefix, last, &format!(
                "[method salm] {} upon {} ({}) → {}",
                name, target_type, params_str(params), type_str(ret_type)
            ));
            let cp = child_prefix(prefix, last);
            print_block(body, &cp);
        }
    }
}

// ══════════════════════════════════════════════════════════════════
// Block
// ══════════════════════════════════════════════════════════════════

fn print_block(stmts: &[Stmt], prefix: &str) {
    let n = stmts.len();
    for (i, s) in stmts.iter().enumerate() {
        print_stmt(s, prefix, i + 1 == n);
    }
}

// ══════════════════════════════════════════════════════════════════
// Statements
// ══════════════════════════════════════════════════════════════════

fn print_stmt(stmt: &Stmt, prefix: &str, last: bool) {
    match stmt {
        Stmt::DeclNoVal { name, ty } => {
            leaf(prefix, last, &format!("let there be {}: {}", name, type_str(ty)));
        }
        Stmt::DeclVal { name, ty, val } => {
            leaf(prefix, last, &format!("let there {}: {} =", name, type_str(ty)));
            let cp = child_prefix(prefix, last);
            print_expr(val, &cp, true);
        }
        Stmt::Assign { name, val } => {
            leaf(prefix, last, &format!("{} become", name));
            let cp = child_prefix(prefix, last);
            print_expr(val, &cp, true);
        }
        Stmt::FnCallStmt { name, args } => {
            leaf(prefix, last, &format!("hail {} ({})", name, args.len()));
            let cp = child_prefix(prefix, last);
            children(args, &cp, |a, p, l| print_expr(a, p, l));
        }
        Stmt::MethodCallStmt { method, target, args } => {
            leaf(prefix, last, &format!("hail {} upon {} ({})", method, target, args.len()));
            if !args.is_empty() {
                let cp = child_prefix(prefix, last);
                children(args, &cp, |a, p, l| print_expr(a, p, l));
            }
        }
        Stmt::Reveal(expr) => {
            leaf(prefix, last, "reveal");
            let cp = child_prefix(prefix, last);
            print_expr(expr, &cp, true);
        }
        Stmt::Conditional { branches, otherwise } => {
            leaf(prefix, last, &format!(
                "whether ({} branch{}{})",
                branches.len(),
                if branches.len() != 1 { "es" } else { "" },
                if otherwise.is_some() { " + otherwise" } else { "" }
            ));
            let cp = child_prefix(prefix, last);
            let total = branches.len() + usize::from(otherwise.is_some());
            for (i, (cond, body)) in branches.iter().enumerate() {
                let tag = if i == 0 { "cond" } else { "otherwise so" };
                let bl = i + 1 == total && otherwise.is_none();
                leaf(&cp, bl, tag);
                let cp2 = child_prefix(&cp, bl);
                print_expr(cond, &cp2, false);
                leaf(&cp2, true, "then");
                let cp3 = child_prefix(&cp2, true);
                print_block(body, &cp3);
            }
            if let Some(else_body) = otherwise {
                leaf(&cp, true, "otherwise");
                let cp2 = child_prefix(&cp, true);
                print_block(else_body, &cp2);
            }
        }
        Stmt::Litany { cond, body } => {
            leaf(prefix, last, "litany for");
            let cp = child_prefix(prefix, last);
            leaf(&cp, false, "cond");
            let cp2 = child_prefix(&cp, false);
            print_expr(cond, &cp2, true);
            leaf(&cp, true, "body");
            let cp2 = child_prefix(&cp, true);
            print_block(body, &cp2);
        }
        Stmt::Confess { try_block, handlers, absolve } => {
            leaf(prefix, last, &format!(
                "confess ({} handler{}{})",
                handlers.len(),
                if handlers.len() != 1 { "s" } else { "" },
                if absolve.is_some() { " + absolve" } else { "" }
            ));
            let cp = child_prefix(prefix, last);
            let has_abs = absolve.is_some();
            leaf(&cp, handlers.is_empty() && !has_abs, "try");
            let cp2 = child_prefix(&cp, handlers.is_empty() && !has_abs);
            print_block(try_block, &cp2);

            for (i, h) in handlers.iter().enumerate() {
                let hl = i + 1 == handlers.len() && !has_abs;
                let label = match &h.binding {
                    Some(b) => format!("answer for {} as {}", h.sin_type, b),
                    None    => format!("answer for {}", h.sin_type),
                };
                leaf(&cp, hl, &label);
                let cp2 = child_prefix(&cp, hl);
                print_block(&h.body, &cp2);
            }

            if let Some(abs) = absolve {
                leaf(&cp, true, "absolve");
                let cp2 = child_prefix(&cp, true);
                print_block(abs, &cp2);
            }
        }
        Stmt::Discern { target, branches, otherwise } => {
            leaf(prefix, last, &format!(
                "discern {} ({} branch{}{})",
                target,
                branches.len(),
                if branches.len() != 1 { "es" } else { "" },
                if otherwise.is_some() { " + otherwise" } else { "" }
            ));
            let cp = child_prefix(prefix, last);
            let total = branches.len() + usize::from(otherwise.is_some());
            for (i, (variant, body)) in branches.iter().enumerate() {
                let bl = i + 1 == total && otherwise.is_none();
                leaf(&cp, bl, &format!("as {}", variant));
                let cp2 = child_prefix(&cp, bl);
                print_block(body, &cp2);
            }
            if let Some(otherwise_body) = otherwise {
                leaf(&cp, true, "otherwise");
                let cp2 = child_prefix(&cp, true);
                print_block(otherwise_body, &cp2);
            }
        }
        Stmt::Transgress { sin_type, args } => {
            leaf(prefix, last, &format!("transgress {} ({})", sin_type, args.len()));
            let cp = child_prefix(prefix, last);
            children(args, &cp, |a, p, l| print_expr(a, p, l));
        }
        Stmt::Forsake => { leaf(prefix, last, "forsake"); }
        Stmt::Ascend  => { leaf(prefix, last, "ascend"); }
    }
}

// ══════════════════════════════════════════════════════════════════
// Expressions
// ══════════════════════════════════════════════════════════════════

fn print_expr(expr: &Expr, prefix: &str, last: bool) {
    match expr {
        Expr::Lit(lit) => {
            let s = match lit {
                Literal::Int(n)   => format!("Int({})", n),
                Literal::Float(f) => format!("Float({})", f),
                Literal::Str(s)   => format!("Str({:?})", s),
                Literal::Bool(b)  => if *b { "blessed".into() } else { "forsaken".into() },
            };
            leaf(prefix, last, &s);
        }
        Expr::Var(name) => {
            leaf(prefix, last, &format!("Var({})", name));
        }
        Expr::Negate(expr) => {
            leaf(prefix, last, "Negate");
            let cp = child_prefix(prefix, last);
            print_expr(expr, &cp, true);
        }
        Expr::BinOp { op, left, right } => {
            let op_str = match op {
                BinOp::Add => "plus",
                BinOp::Sub => "minus",
                BinOp::Mul => "times",
                BinOp::Div => "over",
                BinOp::Rem => "remainder",
                BinOp::Eq  => "is",
                BinOp::Ne  => "is not",
                BinOp::Gt  => "greater than",
                BinOp::Lt  => "lesser than",
                BinOp::Ge  => "no lesser than",
                BinOp::Le  => "no greater than",
            };
            leaf(prefix, last, &format!("BinOp({})", op_str));
            let cp = child_prefix(prefix, last);
            print_expr(left, &cp, false);
            print_expr(right, &cp, true);
        }
        Expr::FnCall { name, args } => {
            leaf(prefix, last, &format!("hail {} ({})", name, args.len()));
            let cp = child_prefix(prefix, last);
            children(args, &cp, |a, p, l| print_expr(a, p, l));
        }
        Expr::MethodCall { method, target, args } => {
            leaf(prefix, last, &format!("hail {} upon {} ({})", method, target, args.len()));
            if !args.is_empty() {
                let cp = child_prefix(prefix, last);
                children(args, &cp, |a, p, l| print_expr(a, p, l));
            }
        }
        Expr::Manifest { scripture, args } => {
            leaf(prefix, last, &format!("manifest {} ({})", scripture, args.len()));
            let cp = child_prefix(prefix, last);
            children(args, &cp, |a, p, l| print_expr(a, p, l));
        }
        Expr::FieldAccess { field, object } => {
            leaf(prefix, last, &format!("FieldAccess({} from {})", field, object));
        }
        Expr::SelfFieldAccess { field } => {
            leaf(prefix, last, &format!("FieldAccess({} from its)", field));
        }
    }
}
