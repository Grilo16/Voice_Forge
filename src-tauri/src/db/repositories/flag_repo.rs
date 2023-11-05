use crate::Flag;
use rusqlite::{Connection, Result, Error, params, ffi::sqlite3_last_insert_rowid};


pub fn get_flags(query: String) -> Result<String> {
    let path = "../sqlite.db";
    let conn = Connection::open(path)?;

    let result = {
        let mut stmt = conn.prepare(&query)?;
        let flags = stmt.query_map([], |row| {
            Ok(Flag {
                id: row.get(0)?,
                label: row.get(1)?,
                flag: row.get(2)?,
                input_type : row.get(3)?,
                required: row.get(4)?,
            })
        })?;
    
        let mut data = Vec::new();
        for flag in flags {
            match flag {
                Ok(flag) => {
                    data.push(flag);
                }
                Err(err) => {
                    return Err(err)
                }
            }
        }
        
        let json_data: String = serde_json::to_string(&data).expect("Failed to serialize to JSON");
        json_data
        };

        match conn.close() {
            Ok(()) => {
                println!("Database connection closed successfully.");
            }
            Err(err) => {
                eprintln!("Error closing the database connection: {:?}", err);
                // Optionally, you can propagate the error here by returning Err(err)
                return Err(err.1)
            }
    }


    Ok(result)
}

pub fn get_flag(id: i64) -> Result<Flag>{
    let path = "../sqlite.db";
    let conn = Connection::open(path)?;
    let query :String = "SELECT * FROM flags Where id=?1".to_string();
    let row = conn.query_row(&query, params![id], |row| {
        Ok(Flag {
            id: row.get(0)?,
            label: row.get(1)?,
            flag: row.get(2)?,
            input_type: row.get(3)?,
            required: row.get(4)?,
        })
    });

    let flag = row?;
    Ok(flag)
}


pub fn create_flag(flag: Flag) -> Result<Flag, Error> {
    let path = "../sqlite.db";
    let conn = Connection::open(path)?;


    let result = {
    conn.execute(
        "INSERT INTO flags (label, flag, input_type, required)
                  VALUES (?1, ?2, ?3, ?4)",
        params![flag.label, flag.flag, flag.input_type, flag.required],
    )?;

    let last_insert_id = conn.last_insert_rowid();
    get_flag(last_insert_id)?

    };


    match conn.close() {
        Ok(()) => {
            println!("Database connection closed successfully.");
        }
        Err(err) => {
            eprintln!("Error closing the database connection: {:?}", err);
            // Optionally, you can propagate the error here by returning Err(err)
            return Err(err.1)
        }
}

    Ok(result)
}