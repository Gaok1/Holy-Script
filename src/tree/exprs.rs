use termtree::Tree;

use crate::ast::*;

use super::type_str;

pub(super) fn build_expr_tree(expr: &Expr) -> Tree<String> {
    match expr {
        Expr::Lit(lit) => Tree::new(lit_label(lit)),

        Expr::Var(name) => Tree::new(format!("Var({})", name)),

        Expr::Negate(inner) => {
            let mut tree = Tree::new("Negate".into());
            tree.push(build_expr_tree(inner));
            tree
        }

        Expr::BinOp { op, left, right } => {
            let mut tree = Tree::new(format!("BinOp({})", binop_str(op)));
            tree.push(build_expr_tree(left));
            tree.push(build_expr_tree(right));
            tree
        }

        Expr::FnCall { name, type_args, args } => {
            let ta = fmt_type_args(type_args);
            let mut tree = Tree::new(format!("hail {}{} ({})", name, ta, args.len()));
            for arg in args {
                tree.push(build_expr_tree(arg));
            }
            tree
        }

        Expr::MethodCall { method, target, args } => {
            let mut tree = Tree::new(format!("hail {} upon ({})", method, args.len()));
            tree.push(build_expr_tree(target));
            for arg in args {
                tree.push(build_expr_tree(arg));
            }
            tree
        }

        Expr::Manifest { scripture, args } => {
            let mut tree = Tree::new(format!("manifest {} ({})", scripture, args.len()));
            for arg in args {
                tree.push(build_expr_tree(arg));
            }
            tree
        }

        Expr::ManifestVariant { variant, covenant, type_args, args } => {
            let ta = fmt_type_args(type_args);
            let mut tree = Tree::new(format!("manifest {} of {}{} ({})", variant, covenant, ta, args.len()));
            for arg in args {
                tree.push(build_expr_tree(arg));
            }
            tree
        }

        Expr::TypedUnitVariant { variant, covenant, type_args } => {
            let ta = fmt_type_args(type_args);
            Tree::new(format!("{} of {}{}", variant, covenant, ta))
        }

        Expr::FieldAccess { field, object } => {
            let mut tree = Tree::new(format!("FieldAccess({})", field));
            tree.push(build_expr_tree(object));
            tree
        }

        Expr::SelfFieldAccess { field } => {
            Tree::new(format!("FieldAccess({} from its)", field))
        }
    }
}

/// Short label for an expression, used in parent nodes (e.g. `discern` header).
pub(super) fn expr_label(expr: &Expr) -> String {
    match expr {
        Expr::Var(name)                    => name.clone(),
        Expr::FieldAccess { field, object } => format!("{} from {}", field, expr_label(object)),
        Expr::SelfFieldAccess { field }    => format!("{} from its", field),
        _                                  => "<expr>".into(),
    }
}

fn lit_label(lit: &Literal) -> String {
    match lit {
        Literal::Int(n)       => format!("Int({})", n),
        Literal::Float(f)     => format!("Float({})", f),
        Literal::Str(s)       => format!("Str({:?})", s),
        Literal::Bool(true)   => "blessed".into(),
        Literal::Bool(false)  => "forsaken".into(),
    }
}

fn binop_str(op: &BinOp) -> &'static str {
    match op {
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
    }
}

fn fmt_type_args(type_args: &[HolyType]) -> String {
    if type_args.is_empty() {
        String::new()
    } else {
        format!("<{}>", type_args.iter().map(type_str).collect::<Vec<_>>().join(", "))
    }
}
