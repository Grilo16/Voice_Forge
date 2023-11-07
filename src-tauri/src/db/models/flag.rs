use core::fmt;

use rusqlite::types::{FromSql, ValueRef, FromSqlError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Flag {
    pub id: i64,
    pub label: String,
    pub flag: String,
    pub input_type: String, 
    pub required: bool,
}

impl Flag {
    // Define a new function for creating instances of Flag
    pub fn new(id: i64, label: String, flag: String, input_type: String, required: bool) -> Self {
        Flag {
            id,
            label,
            flag,
            input_type,
            required,
        }
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}, label: {}, flag: {}, input_type: {}, required: {}",
            self.id, self.label, self.flag, self.input_type, self.required
        )
    }
}


impl FromSql for Flag {
    fn column_result(value: ValueRef) -> Result<Flag, FromSqlError> {
        let id: i64 = value.as_i64()?;
        let label: String = value.as_str()?.to_string();
        let flag: String =  value.as_str()?.to_string();
        let input_type: String =  value.as_str()?.to_string();
        let required: bool = value.as_i64()? != 0;

        Ok(Flag {
            id,
            flag,
            label,
            input_type,
            required
        })
    }
}
