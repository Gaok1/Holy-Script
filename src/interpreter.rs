use std::collections::HashMap;
use std::fmt;
use std::io::{self, BufRead, Write};

use crate::ast::*;

// ══════════════════════════════════════════════════════════════════
// Runtime value
// ══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Void,
    CovenantVariant { covenant: String, variant: String },
    Scripture { type_name: String, fields: HashMap<String, Value> },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n)    => write!(f, "{}", n),
            Value::Float(x)  => write!(f, "{}", x),
            Value::Str(s)    => write!(f, "{}", s),
            Value::Bool(b)   => write!(f, "{}", if *b { "blessed" } else { "forsaken" }),
            Value::Void      => write!(f, "void"),
            Value::CovenantVariant { covenant, variant } => write!(f, "{}::{}", covenant, variant),
            Value::Scripture { type_name, fields } => {
                write!(f, "{}{{", type_name)?;
                let mut first = true;
                let mut keys: Vec<_> = fields.keys().collect();
                keys.sort(); // stable ordering for tests / display
                for k in keys {
                    if !first { write!(f, ", ")?; }
                    write!(f, "{}: {}", k, fields[k])?;
                    first = false;
                }
                write!(f, "}}")
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════════
// Control-flow signals and errors
// ══════════════════════════════════════════════════════════════════

#[derive(Debug)]
pub enum HolyError {
    /// Raised by `reveal expr` — caught at the salm call site.
    Return(Value),
    /// Raised by `transgress` — caught by `confess`.
    Sin { type_name: String, fields: HashMap<String, Value> },
    /// Raised by `forsake` — caught by the enclosing `litany for`.
    Break,
    /// Raised by `ascend` — caught by the enclosing `litany for`.
    Continue,
    /// Unrecoverable runtime error.
    Runtime(String),
}

impl fmt::Display for HolyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HolyError::Return(_)            => write!(f, "'reveal' used outside a salm"),
            HolyError::Sin { type_name, .. }=> write!(f, "unhandled sin: {}", type_name),
            HolyError::Break                => write!(f, "'forsake' used outside a litany"),
            HolyError::Continue             => write!(f, "'ascend' used outside a litany"),
            HolyError::Runtime(msg)         => write!(f, "RuntimeError: {}", msg),
        }
    }
}

type EvalResult = Result<Value, HolyError>;
type ExecResult = Result<(), HolyError>;

// ══════════════════════════════════════════════════════════════════
// Variable environment
// ══════════════════════════════════════════════════════════════════

/// Lexically-scoped environment.
///
/// `global` holds variables declared at the top level.
/// `locals` is a stack of scopes for the current call frame.
/// When entering a salm, `locals` is replaced with a fresh stack —
/// the called salm cannot see the caller's local variables.
struct Env {
    global: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
}

impl Env {
    fn new() -> Self {
        Env { global: HashMap::new(), locals: Vec::new() }
    }

    /// Opens a new inner scope (whether, litany, confess, …).
    fn push(&mut self) {
        self.locals.push(HashMap::new());
    }

    /// Closes the innermost scope.
    fn pop(&mut self) {
        self.locals.pop();
    }

    fn get(&self, name: &str) -> Option<Value> {
        for scope in self.locals.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v.clone());
            }
        }
        self.global.get(name).cloned()
    }

    /// Defines a variable in the innermost available scope.
    /// Falls back to global when not inside any salm.
    fn define(&mut self, name: &str, val: Value) {
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name.to_string(), val);
        } else {
            self.global.insert(name.to_string(), val);
        }
    }

    /// Assigns to the nearest existing binding (locals → global).
    /// Returns `false` if the variable has not been defined yet.
    fn assign(&mut self, name: &str, val: Value) -> bool {
        for scope in self.locals.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), val);
                return true;
            }
        }
        if self.global.contains_key(name) {
            self.global.insert(name.to_string(), val);
            return true;
        }
        false
    }

    /// Saves the current locals and returns an isolated fresh frame.
    /// Used when entering a salm call.
    fn enter_call(&mut self) -> Vec<HashMap<String, Value>> {
        std::mem::replace(&mut self.locals, vec![HashMap::new()])
    }

    /// Restores the caller's frame after a salm call.
    fn exit_call(&mut self, saved: Vec<HashMap<String, Value>>) {
        self.locals = saved;
    }
}

// ══════════════════════════════════════════════════════════════════
// Stored salm definition
// ══════════════════════════════════════════════════════════════════

#[derive(Clone)]
struct SalmDef {
    params: Vec<(String, HolyType)>,
    body:   Vec<Stmt>,
}

// ══════════════════════════════════════════════════════════════════
// Interpreter
// ══════════════════════════════════════════════════════════════════

pub struct Interpreter {
    env:               Env,
    salms:             HashMap<String, SalmDef>,
    /// key: (method_name, scripture_type)
    methods:           HashMap<(String, String), SalmDef>,
    scriptures:        HashMap<String, Vec<(String, HolyType)>>,
    sins:              HashMap<String, Vec<(String, HolyType)>>,
    covenants:         HashMap<String, Vec<String>>,
    /// Maps each variant name back to its covenant name.
    covenant_variants: HashMap<String, String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env:               Env::new(),
            salms:             HashMap::new(),
            methods:           HashMap::new(),
            scriptures:        HashMap::new(),
            sins:              HashMap::new(),
            covenants:         HashMap::new(),
            covenant_variants: HashMap::new(),
        }
    }

    /// Exposes a variable from the environment (used in tests).
    pub fn env_get(&self, name: &str) -> Option<Value> {
        self.env.get(name)
    }

    // ──────────────────────────────────────────────────────────────
    // Entry point
    // ──────────────────────────────────────────────────────────────

    pub fn run(&mut self, program: &Program) -> Result<(), HolyError> {
        for decl in &program.top_decls {
            self.register_top_decl(decl);
        }
        self.exec_stmts(&program.stmts)
    }

    fn register_top_decl(&mut self, decl: &TopDecl) {
        match decl {
            TopDecl::Salm { name, params, body, .. } => {
                self.salms.insert(name.clone(), SalmDef {
                    params: params.clone(),
                    body: body.clone(),
                });
            }
            TopDecl::MethodSalm { name, target_type, params, body, .. } => {
                self.methods.insert((name.clone(), target_type.clone()), SalmDef {
                    params: params.clone(),
                    body: body.clone(),
                });
            }
            TopDecl::Scripture { name, fields } => {
                self.scriptures.insert(name.clone(), fields.clone());
            }
            TopDecl::SinDecl { name, fields } => {
                self.sins.insert(name.clone(), fields.clone());
            }
            TopDecl::Covenant { name, variants } => {
                self.covenants.insert(name.clone(), variants.clone());
                for variant in variants {
                    self.covenant_variants.insert(variant.clone(), name.clone());
                }
            }
        }
    }

    // ──────────────────────────────────────────────────────────────
    // Statement execution
    // ──────────────────────────────────────────────────────────────

    fn exec_stmts(&mut self, stmts: &[Stmt]) -> ExecResult {
        for stmt in stmts {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> ExecResult {
        match stmt {
            Stmt::DeclNoVal { name, ty } => {
                let val = default_value(ty);
                self.env.define(name, val);
            }
            Stmt::DeclVal { name, ty: _, val } => {
                let v = self.eval_expr(val)?;
                self.env.define(name, v);
            }
            Stmt::Assign { name, val } => {
                let v = self.eval_expr(val)?;
                if !self.env.assign(name, v.clone()) {
                    // variable not yet defined — create it in the current scope
                    self.env.define(name, v);
                }
            }
            Stmt::FnCallStmt { name, args } => {
                let vals = self.eval_args(args)?;
                self.call_salm(name, vals)?;
            }
            Stmt::MethodCallStmt { method, target, args } => {
                let target_val = self.env.get(target)
                    .ok_or_else(|| HolyError::Runtime(format!("undefined variable '{}'", target)))?;
                let vals = self.eval_args(args)?;
                self.call_method(method, target_val, vals)?;
            }
            Stmt::Reveal(expr) => {
                let v = self.eval_expr(expr)?;
                return Err(HolyError::Return(v));
            }
            Stmt::Conditional { branches, otherwise } => {
                let mut executed = false;
                for (cond, body) in branches {
                    let cv = self.eval_expr(cond)?;
                    if is_truthy(&cv) {
                        self.env.push();
                        let r = self.exec_stmts(body);
                        self.env.pop();
                        r?;
                        executed = true;
                        break;
                    }
                }
                if !executed {
                    if let Some(else_body) = otherwise {
                        self.env.push();
                        let r = self.exec_stmts(else_body);
                        self.env.pop();
                        r?;
                    }
                }
            }
            Stmt::Litany { cond, body } => {
                loop {
                    let cv = self.eval_expr(cond)?;
                    if !is_truthy(&cv) { break; }
                    self.env.push();
                    let r = self.exec_stmts(body);
                    self.env.pop();
                    match r {
                        Ok(())                    => {}
                        Err(HolyError::Break)     => break,
                        Err(HolyError::Continue)  => continue,
                        Err(e)                    => return Err(e),
                    }
                }
            }
            Stmt::Forsake => return Err(HolyError::Break),
            Stmt::Ascend  => return Err(HolyError::Continue),
            Stmt::Confess { try_block, handlers, absolve } => {
                self.exec_confess(try_block, handlers, absolve)?;
            }
            Stmt::Discern { target, branches, otherwise } => {
                self.exec_discern(target, branches, otherwise)?;
            }
            Stmt::Transgress { sin_type, args } => {
                let fields = self.build_sin_fields(sin_type, args)?;
                return Err(HolyError::Sin { type_name: sin_type.clone(), fields });
            }
        }
        Ok(())
    }

    fn exec_confess(
        &mut self,
        try_block: &[Stmt],
        handlers: &[SinHandler],
        absolve: &Option<Vec<Stmt>>,
    ) -> ExecResult {
        self.env.push();
        let try_result = self.exec_stmts(try_block);
        self.env.pop();

        let after = match try_result {
            Err(HolyError::Sin { ref type_name, ref fields }) => {
                let matched = handlers.iter().find(|h| &h.sin_type == type_name);
                match matched {
                    Some(h) => {
                        let sin_val = Value::Scripture {
                            type_name: type_name.clone(),
                            fields: fields.clone(),
                        };
                        let body = h.body.clone();
                        let bind = h.binding.clone();
                        self.env.push();
                        if let Some(b) = &bind {
                            self.env.define(b, sin_val);
                        }
                        let r = self.exec_stmts(&body);
                        self.env.pop();
                        r
                    }
                    None => try_result, // re-propagate unhandled sin
                }
            }
            other => other,
        };

        // absolve always runs, even if a sin was raised (equivalent to `finally`)
        if let Some(abs) = absolve {
            let abs = abs.clone();
            self.env.push();
            let ar = self.exec_stmts(&abs);
            self.env.pop();
            if ar.is_err() {
                return ar; // absolve error takes priority
            }
        }

        after
    }

    fn exec_discern(
        &mut self,
        target: &str,
        branches: &[(String, Vec<Stmt>)],
        otherwise: &Option<Vec<Stmt>>,
    ) -> ExecResult {
        let target_value = self.env.get(target)
            .ok_or_else(|| HolyError::Runtime(format!("undefined variable '{}'", target)))?;

        let matched_variant = match target_value {
            Value::CovenantVariant { variant, .. } => variant,
            other => {
                return Err(HolyError::Runtime(format!(
                    "'discern' expects a covenant variant in '{}', got {}",
                    target,
                    value_type_name(&other)
                )));
            }
        };

        if let Some((_, body)) = branches.iter().find(|(variant, _)| variant == &matched_variant) {
            self.env.push();
            let result = self.exec_stmts(body);
            self.env.pop();
            return result;
        }

        if let Some(body) = otherwise {
            self.env.push();
            let result = self.exec_stmts(body);
            self.env.pop();
            return result;
        }

        Err(HolyError::Runtime(format!(
            "no 'discern' branch covers variant '{}'",
            matched_variant
        )))
    }

    // ──────────────────────────────────────────────────────────────
    // Salm calls
    // ──────────────────────────────────────────────────────────────

    fn call_salm(&mut self, name: &str, args: Vec<Value>) -> EvalResult {
        // Built-in salms
        match name {
            "proclaim" => {
                let s = args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
                println!("{}", s);
                return Ok(Value::Void);
            }
            "herald" => {
                let s = args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
                print!("{}", s);
                io::stdout().flush().ok();
                return Ok(Value::Void);
            }
            "inquire" => {
                let mut line = String::new();
                io::stdin().lock().read_line(&mut line).ok();
                return Ok(Value::Str(line.trim_end_matches('\n').to_string()));
            }
            "atom_of" => {
                let n = match args.first() {
                    Some(Value::Str(s)) => s.trim().parse().unwrap_or(0),
                    _ => 0,
                };
                return Ok(Value::Int(n));
            }
            "word_of" => {
                return Ok(Value::Str(args.first().map(|v| v.to_string()).unwrap_or_default()));
            }
            _ => {}
        }

        let def = self.salms.get(name)
            .ok_or_else(|| HolyError::Runtime(format!("salm '{}' is not defined", name)))?
            .clone();

        self.exec_salm_body(&def, None, args)
    }

    fn call_method(&mut self, method: &str, target: Value, args: Vec<Value>) -> EvalResult {
        let type_name = match &target {
            Value::Scripture { type_name, .. } => type_name.clone(),
            other => return Err(HolyError::Runtime(format!(
                "cannot call a method on a value of type '{}'",
                value_type_name(other)
            ))),
        };

        let def = self.methods.get(&(method.to_string(), type_name))
            .ok_or_else(|| HolyError::Runtime(format!("method '{}' not found", method)))?
            .clone();

        self.exec_salm_body(&def, Some(target), args)
    }

    /// Executes a salm or method_salm body in an isolated call frame.
    /// `self_val` is `Some(target)` for method_salms — bound as `its` inside the body.
    fn exec_salm_body(&mut self, def: &SalmDef, self_val: Option<Value>, args: Vec<Value>) -> EvalResult {
        let saved = self.env.enter_call();
        if let Some(val) = self_val {
            self.env.define("its", val);
        }
        for ((pname, _), val) in def.params.iter().zip(args) {
            self.env.define(pname, val);
        }
        let result = self.exec_stmts(&def.body);
        self.env.exit_call(saved);

        match result {
            Ok(())                     => Ok(Value::Void),
            Err(HolyError::Return(v)) => Ok(v),
            Err(e)                     => Err(e),
        }
    }

    // ──────────────────────────────────────────────────────────────
    // Expression evaluation
    // ──────────────────────────────────────────────────────────────

    fn eval_expr(&mut self, expr: &Expr) -> EvalResult {
        match expr {
            Expr::Lit(lit) => Ok(eval_literal(lit)),

            Expr::Var(name) => {
                if let Some(value) = self.env.get(name) {
                    Ok(value)
                } else if let Some(covenant) = self.covenant_variants.get(name) {
                    Ok(Value::CovenantVariant {
                        covenant: covenant.clone(),
                        variant: name.clone(),
                    })
                } else {
                    Err(HolyError::Runtime(format!("undefined variable '{}'", name)))
                }
            }

            Expr::Negate(expr) => {
                let value = self.eval_expr(expr)?;
                match value {
                    Value::Int(n)   => Ok(Value::Int(-n)),
                    Value::Float(x) => Ok(Value::Float(-x)),
                    other => Err(HolyError::Runtime(format!(
                        "'negate' expects atom or fractional, got {}",
                        value_type_name(&other)
                    ))),
                }
            }

            Expr::BinOp { op, left, right } => {
                let lv = self.eval_expr(left)?;
                let rv = self.eval_expr(right)?;
                eval_binop(op, lv, rv)
            }

            Expr::FnCall { name, args } => {
                let vals = self.eval_args(args)?;
                self.call_salm(name, vals)
            }

            Expr::MethodCall { method, target, args } => {
                let tv = self.env.get(target)
                    .ok_or_else(|| HolyError::Runtime(format!("undefined variable '{}'", target)))?;
                let vals = self.eval_args(args)?;
                self.call_method(method, tv, vals)
            }

            Expr::Manifest { scripture, args } => {
                let def = self.scriptures.get(scripture)
                    .ok_or_else(|| HolyError::Runtime(format!("scripture '{}' is not defined", scripture)))?
                    .clone();

                if args.len() != def.len() {
                    return Err(HolyError::Runtime(format!(
                        "scripture '{}' expects {} fields, got {}",
                        scripture, def.len(), args.len()
                    )));
                }
                let mut fields = HashMap::new();
                for ((fname, _), arg) in def.iter().zip(args.iter()) {
                    let v = self.eval_expr(arg)?;
                    fields.insert(fname.clone(), v);
                }
                Ok(Value::Scripture { type_name: scripture.clone(), fields })
            }

            Expr::FieldAccess { field, object } => {
                let obj = self.env.get(object)
                    .ok_or_else(|| HolyError::Runtime(format!("undefined variable '{}'", object)))?;
                get_field(&obj, field)
            }

            Expr::SelfFieldAccess { field } => {
                let its = self.env.get("its")
                    .ok_or_else(|| HolyError::Runtime("'its' is not available outside a method_salm".into()))?;
                get_field(&its, field)
            }
        }
    }

    fn eval_args(&mut self, args: &[Expr]) -> Result<Vec<Value>, HolyError> {
        args.iter().map(|a| self.eval_expr(a)).collect()
    }

    // ──────────────────────────────────────────────────────────────
    // Helpers
    // ──────────────────────────────────────────────────────────────

    fn build_sin_fields(&mut self, sin_type: &str, args: &[Expr]) -> Result<HashMap<String, Value>, HolyError> {
        let def = self.sins.get(sin_type)
            .ok_or_else(|| HolyError::Runtime(format!("sin '{}' is not declared", sin_type)))?
            .clone();

        let mut fields = HashMap::new();
        for ((fname, _), arg) in def.iter().zip(args.iter()) {
            let v = self.eval_expr(arg)?;
            fields.insert(fname.clone(), v);
        }
        Ok(fields)
    }
}

// ══════════════════════════════════════════════════════════════════
// Free helpers
// ══════════════════════════════════════════════════════════════════

fn eval_literal(lit: &Literal) -> Value {
    match lit {
        Literal::Int(n)   => Value::Int(*n),
        Literal::Float(f) => Value::Float(*f),
        Literal::Str(s)   => Value::Str(s.clone()),
        Literal::Bool(b)  => Value::Bool(*b),
    }
}

fn is_truthy(v: &Value) -> bool {
    match v {
        Value::Bool(b)            => *b,
        Value::Int(n)             => *n != 0,
        Value::Float(f)           => *f != 0.0,
        Value::Str(s)             => !s.is_empty(),
        Value::Void               => false,
        Value::CovenantVariant { .. } => true,
        Value::Scripture { .. }   => true,
    }
}

fn default_value(ty: &HolyType) -> Value {
    match ty {
        HolyType::Atom       => Value::Int(0),
        HolyType::Fractional => Value::Float(0.0),
        HolyType::Word       => Value::Str(String::new()),
        HolyType::Dogma      => Value::Bool(false),
        HolyType::Void       => Value::Void,
        HolyType::Custom(_)  => Value::Void,
    }
}

fn get_field(val: &Value, field: &str) -> EvalResult {
    match val {
        Value::Scripture { fields, .. } => fields
            .get(field)
            .cloned()
            .ok_or_else(|| HolyError::Runtime(format!("field '{}' not found", field))),
        _ => Err(HolyError::Runtime("field access on a non-scripture value".into())),
    }
}

fn eval_binop(op: &BinOp, l: Value, r: Value) -> EvalResult {
    use Value::*;

    // Numeric promotion: Int op Float → Float op Float
    let (l, r) = match (&l, &r) {
        (Int(a), Float(b))  => (Float(*a as f64), Float(*b)),
        (Float(a), Int(b))  => (Float(*a), Float(*b as f64)),
        _ => (l, r),
    };

    match op {
        BinOp::Add => match (&l, &r) {
            (Int(a),   Int(b))   => Ok(Int(a + b)),
            (Float(a), Float(b)) => Ok(Float(a + b)),
            (Str(a),   Str(b))   => Ok(Str(format!("{}{}", a, b))),
            _ => type_err("plus", &l, &r),
        },
        BinOp::Sub => match (&l, &r) {
            (Int(a),   Int(b))   => Ok(Int(a - b)),
            (Float(a), Float(b)) => Ok(Float(a - b)),
            _ => type_err("minus", &l, &r),
        },
        BinOp::Mul => match (&l, &r) {
            (Int(a),   Int(b))   => Ok(Int(a * b)),
            (Float(a), Float(b)) => Ok(Float(a * b)),
            _ => type_err("times", &l, &r),
        },
        BinOp::Div => match (&l, &r) {
            (Int(a),   Int(b)) => {
                if *b == 0 { Err(HolyError::Runtime("division by zero".into())) }
                else { Ok(Int(a / b)) }
            }
            (Float(a), Float(b)) => {
                if *b == 0.0 { Err(HolyError::Runtime("division by zero".into())) }
                else { Ok(Float(a / b)) }
            }
            _ => type_err("over", &l, &r),
        },
        BinOp::Rem => match (&l, &r) {
            (Int(a), Int(b)) => {
                if *b == 0 { Err(HolyError::Runtime("division by zero".into())) }
                else { Ok(Int(a % b)) }
            }
            _ => type_err("remainder", &l, &r),
        },
        BinOp::Eq => Ok(Bool(values_equal(&l, &r))),
        BinOp::Ne => Ok(Bool(!values_equal(&l, &r))),
        BinOp::Gt => numeric_cmp(&l, &r, |a, b| a > b),
        BinOp::Lt => numeric_cmp(&l, &r, |a, b| a < b),
        BinOp::Ge => numeric_cmp(&l, &r, |a, b| a >= b),
        BinOp::Le => numeric_cmp(&l, &r, |a, b| a <= b),
    }
}

fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(x),   Value::Int(y))   => x == y,
        (Value::Float(x), Value::Float(y)) => x == y,
        (Value::Str(x),   Value::Str(y))   => x == y,
        (Value::Bool(x),  Value::Bool(y))  => x == y,
        (Value::Void,     Value::Void)     => true,
        _ => false,
    }
}

fn numeric_cmp(l: &Value, r: &Value, f: impl Fn(f64, f64) -> bool) -> EvalResult {
    let lf = to_float(l).ok_or_else(|| HolyError::Runtime("invalid numeric comparison".into()))?;
    let rf = to_float(r).ok_or_else(|| HolyError::Runtime("invalid numeric comparison".into()))?;
    Ok(Value::Bool(f(lf, rf)))
}

fn to_float(v: &Value) -> Option<f64> {
    match v {
        Value::Int(n)   => Some(*n as f64),
        Value::Float(f) => Some(*f),
        _ => None,
    }
}

fn type_err(op: &str, l: &Value, r: &Value) -> EvalResult {
    Err(HolyError::Runtime(format!(
        "operation '{}' not supported between {} and {}",
        op, value_type_name(l), value_type_name(r)
    )))
}

pub fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Int(_)             => "atom",
        Value::Float(_)           => "fractional",
        Value::Str(_)             => "word",
        Value::Bool(_)            => "dogma",
        Value::Void               => "void",
        Value::CovenantVariant { .. } => "covenant",
        Value::Scripture { .. }   => "scripture",
    }
}
