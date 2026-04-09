use std::collections::HashMap;
use std::fmt;

use super::Value;

#[derive(Debug)]
pub enum HolyError {
    Return(Value),
    Sin { type_name: String, fields: HashMap<String, Value> },
    Break,
    Continue,
}

impl fmt::Display for HolyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HolyError::Return(_) => write!(f, "'reveal' was defiled outside a salm — revelation belongs only within the sacred salms"),
            HolyError::Sin { type_name, fields } => {
                if let Some(Value::Str(message)) = fields.get("message") {
                    write!(f, "an unabsolved sin has escaped into the world: {} — {}", type_name, message)
                } else {
                    write!(f, "an unabsolved sin has escaped into the world: {}", type_name)
                }
            }
            HolyError::Break => write!(f, "'forsake' was invoked outside a litany — abandonment is permitted only within the litany"),
            HolyError::Continue => write!(f, "'ascend' was invoked outside a litany — ascension is permitted only within the litany"),
        }
    }
}
