mod builtins;
mod call;
mod env;
mod errors;
mod eval;
mod exec;
mod generics;
mod ops;
mod types;
mod value;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::ast::*;
use crate::lexer::tokenize;
use crate::parser::Parser;

use self::builtins::builtin_sins;
use self::env::Env;
use self::generics::builtin_covenants;
pub use self::errors::HolyError;
pub use self::value::{value_type_name, Value};

type EvalResult = Result<Value, HolyError>;
type ExecResult = Result<(), HolyError>;

/// Definition of a user-declared salm (function or method).
#[derive(Clone)]
struct SalmDef {
    type_params: Vec<String>,
    params:      Vec<(String, HolyType)>,
    ret_type:    HolyType,
    body:        Vec<Stmt>,
}

pub struct Interpreter {
    env:               Env,
    salms:             HashMap<String, SalmDef>,
    methods:           HashMap<(String, String), SalmDef>,
    scriptures:        HashMap<String, Vec<(String, HolyType)>>,
    sins:              HashMap<String, Vec<(String, HolyType)>>,
    covenants:         HashMap<String, Vec<CovenantVariantDecl>>,
    /// Maps variant name → (covenant name, ordered field types).
    covenant_variants: HashMap<String, (String, Vec<(String, HolyType)>)>,
    /// CLI args forwarded to the running script (accessible via `hail args`).
    script_args:       Vec<String>,
    /// Directory of the entry file, used to resolve `testament` imports.
    source_dir:        Option<PathBuf>,
    /// Tracks already-loaded module names to prevent circular imports.
    loaded_modules:    HashSet<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        let (builtin_cov, builtin_cov_variants) = builtin_covenants();
        Interpreter {
            env:               Env::new(),
            salms:             HashMap::new(),
            methods:           HashMap::new(),
            scriptures:        HashMap::new(),
            sins:              builtin_sins(),
            covenants:         builtin_cov,
            covenant_variants: builtin_cov_variants,
            script_args:       Vec::new(),
            source_dir:        None,
            loaded_modules:    HashSet::new(),
        }
    }

    /// Pass the script's command-line arguments (everything after the filename).
    pub fn with_script_args(mut self, args: Vec<String>) -> Self {
        self.script_args = args;
        self
    }

    /// Set the directory used to resolve `testament` imports.
    pub fn with_source_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.source_dir = Some(dir.into());
        self
    }

    pub fn env_get(&self, name: &str) -> Option<Value> {
        self.env.get(name)
    }

    pub fn run(&mut self, program: &Program) -> Result<(), HolyError> {
        // Load testament imports first so their symbols are visible to the
        // current program's declarations and statements.
        for testament in &program.testaments.clone() {
            self.load_testament(&testament.name, testament.revealing.as_deref())?;
        }

        for decl in &program.top_decls {
            self.register_top_decl(decl);
        }
        self.validate_declared_types(program)?;
        self.exec_stmts(&program.stmts)
    }

    // ── Testament loading ─────────────────────────────────────────────────────

    fn load_testament(
        &mut self,
        module_name: &str,
        revealing:   Option<&[String]>,
    ) -> Result<(), HolyError> {
        use self::builtins::builtin_sin;

        if self.loaded_modules.contains(module_name) {
            return Ok(()); // already loaded — skip silently
        }

        let dir = self.source_dir.clone().unwrap_or_else(|| PathBuf::from("."));
        let path = dir.join(format!("{}.holy", module_name));

        let source = std::fs::read_to_string(&path).map_err(|e| {
            builtin_sin(
                "UndefinedVariable",
                format!("testament '{}' could not be unsealed at '{}': {}", module_name, path.display(), e),
            )
        })?;

        let tokens = tokenize(&source);

        let mut parser = Parser::new(tokens);
        let module_program = parser.parse_program().map_err(|e| {
            builtin_sin(
                "UndefinedVariable",
                format!("testament '{}' contains a transgression: {}", module_name, e),
            )
        })?;

        self.loaded_modules.insert(module_name.to_string());

        // Recursively load the module's own testaments first
        for dep in &module_program.testaments.clone() {
            self.load_testament(&dep.name, dep.revealing.as_deref())?;
        }

        // Register declarations, filtered by `revealing` if present
        for decl in &module_program.top_decls {
            if should_import(decl, revealing) {
                self.register_top_decl(decl);
            }
        }

        Ok(())
    }

    // ── Declaration registration ──────────────────────────────────────────────

    fn register_top_decl(&mut self, decl: &TopDecl) {
        match decl {
            TopDecl::Salm { name, type_params, params, ret_type, body } => {
                self.salms.insert(name.clone(), SalmDef {
                    type_params: type_params.clone(),
                    params:      params.clone(),
                    ret_type:    ret_type.clone(),
                    body:        body.clone(),
                });
            }
            TopDecl::MethodSalm { name, type_params, target_type, params, ret_type, body } => {
                self.methods.insert((name.clone(), target_type.clone()), SalmDef {
                    type_params: type_params.clone(),
                    params:      params.clone(),
                    ret_type:    ret_type.clone(),
                    body:        body.clone(),
                });
            }
            TopDecl::Scripture { name, fields, .. } => {
                self.scriptures.insert(name.clone(), fields.clone());
            }
            TopDecl::SinDecl { name, fields } => {
                self.sins.insert(name.clone(), fields.clone());
            }
            TopDecl::Covenant { name, variants, .. } => {
                self.covenants.insert(name.clone(), variants.clone());
                for v in variants {
                    self.covenant_variants.insert(
                        v.name.clone(),
                        (name.clone(), v.fields.clone()),
                    );
                }
            }
        }
    }

    // ── Upfront type validation ───────────────────────────────────────────────

    fn validate_declared_types(&self, program: &Program) -> Result<(), HolyError> {
        for decl in &program.top_decls {
            match decl {
                TopDecl::Salm { params, ret_type, type_params, .. } => {
                    for (_, ty) in params {
                        self.ensure_type_exists_with_params(ty, type_params)?;
                    }
                    self.ensure_type_exists_with_params(ret_type, type_params)?;
                }
                TopDecl::MethodSalm { target_type, params, ret_type, type_params, .. } => {
                    self.ensure_custom_type_exists(target_type)?;
                    for (_, ty) in params {
                        self.ensure_type_exists_with_params(ty, type_params)?;
                    }
                    self.ensure_type_exists_with_params(ret_type, type_params)?;
                }
                TopDecl::Scripture { fields, type_params, .. } => {
                    for (_, ty) in fields {
                        self.ensure_type_exists_with_params(ty, type_params)?;
                    }
                }
                TopDecl::SinDecl { fields, .. } => {
                    for (_, ty) in fields {
                        self.ensure_type_exists(ty)?;
                    }
                }
                TopDecl::Covenant { variants, type_params, .. } => {
                    for v in variants {
                        for (_, ty) in &v.fields {
                            self.ensure_type_exists_with_params(ty, type_params)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

// ── Testament import filtering ────────────────────────────────────────────────

/// Returns `true` if `decl` should be imported given the `revealing` filter.
/// `None` → import everything. `Some(list)` → import only listed symbols.
/// Method salms are included whenever their target type is included (or no filter).
fn should_import(decl: &TopDecl, revealing: Option<&[String]>) -> bool {
    let Some(list) = revealing else { return true };

    match decl {
        TopDecl::Salm       { name, .. }  => list.contains(name),
        TopDecl::Scripture  { name, .. }  => list.contains(name),
        TopDecl::SinDecl    { name, .. }  => list.contains(name),
        TopDecl::Covenant   { name, .. }  => list.contains(name),
        // Include method salms whose target type is in the revealing list,
        // or whose method name is explicitly listed.
        TopDecl::MethodSalm { name, target_type, .. } => {
            list.contains(name) || list.contains(target_type)
        }
    }
}
