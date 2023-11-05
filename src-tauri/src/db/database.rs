#[path = "./models/flag.rs"]
mod flag;
use std::path::Path;

use rusqlite::{Connection, Result, params, Error};
use flag::Flag;
use rusqlite::types::{FromSql, FromSqlError, Value, ToSql, ToSqlOutput};
use serde_json;



pub fn initialize_db() -> Result<()>{
    let path = Path::new("../sqlite.db");
    if Path::exists(path){
        Ok(())
    } else {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS flags (
                id INTEGER PRIMARY KEY,
                label TEXT ,
                flag TEXT ,
                input_type TEXT ,
                required BOOLEAN ,
                alt_flags TEXT
            )",
            [],
        )?;
        Ok(())
    }
 }


pub fn run_sql() -> Result<String>  {
    let path = "../sqlite.db";
    let conn = Connection::open(path)?;

    let flag = Flag {
        id: 0,
        label: "i".to_string(),
        flag: "-i".to_string(),
        input_type: "number".to_string(),
        required: true,
    };
    conn.execute(
        "INSERT INTO flags (label, flag, input_type, required) VALUES (?1, ?2, ?3, ?4)",
        (&flag.label, &flag.flag, &flag.input_type, &flag.required),
    )?;

    let mut stmt = conn.prepare("SELECT * FROM flags")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Flag {
            id: row.get(0)?,
            label: row.get(1)?,
            flag: row.get(2)?,
            input_type : row.get(3)?,
            required: row.get(4)?,
        })
    })?;
    let mut data = Vec::new();

    for person in person_iter {
        
        data.push(person.unwrap());
        // println!("Found person {:?}", person.unwrap());
        
    }

    let json_data = serde_json::to_string(&data).expect("Failed to serialize to JSON");
    

    println!("{}", json_data);

    Ok(json_data)  
}

