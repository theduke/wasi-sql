mod util;

pub mod bindings;

impl bindings::sql_v1_alpha1::SqlError {
    fn new_message(message: impl Into<String>) -> Self {
        Self {
            code_numeric: None,
            code_text: None,
            message: message.into(),
            extra: None,
        }
    }
}

pub mod sqlite;
