use crate::Flag;
use crate::Database;
use rusqlite::{Connection, Result, Error, params};


pub struct FlagsRepo {
    conn: Connection,
}

impl FlagsRepo {
    pub fn new() -> Self {
        let db = Database::open().expect("Failed to open database");
        Self { conn: db.get_connection() }
    }
    
    pub fn insert_flag(&self, flag: &Flag) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO flags (label, flag, input_type, required)
            VALUES (?1, ?2, ?3, ?4)",
            params![flag.label, flag.flag, flag.input_type, flag.required],
        )?;
        Ok(())
    }
    pub fn get_flag(&self, id: String) -> Result<Option<Flag>, Error> {
        let parsed_id = id.parse::<i64>().map_err(|e| {
            eprintln!("Failed to parse flag ID as i64: {}", e);
            rusqlite::Error::QueryReturnedNoRows
        })?;
    
        let mut stmt = self.conn.prepare("SELECT * FROM flags WHERE id = ?1")?;
        let flag = stmt.query_row(params![parsed_id], |row| {
            Ok(Flag {
                id: row.get(0)?,
                label: row.get(1)?,
                flag: row.get(2)?,
                input_type: row.get(3)?,
                required: row.get(4)?,
            })
        });

        match flag {
            Ok(flag) => Ok(Some(flag)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None), 
            Err(err) => Err(err),
        }
    }

    pub fn get_all_flags(&self) -> Result<Vec<Flag>, Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM flags")?;
        let flags_iter = stmt.query_map([], |row| {
            Ok(Flag {
                id: row.get(0)?,
                label: row.get(1)?,
                flag: row.get(2)?,
                input_type: row.get(3)?,
                required: row.get(4)?,
            })
        })?;
    
        let mut flags = Vec::new();
        
        for flag in flags_iter {
            match flag {
                Ok(flag) => {
                    flags.push(flag);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    
        Ok(flags)
    }

    pub fn update_flag(&self, flag: &Flag) -> Result<(), Error> {
        self.conn.execute(
            "UPDATE flags
             SET label = ?2, flag = ?3, input_type = ?4, required = ?5
             WHERE id = ?1",
            params![flag.id, flag.label, flag.flag, flag.input_type, flag.required],
        )?;
        Ok(())
    }

    pub fn delete_flag(&self, id: String) -> Result<(), Error> {
        let parsed_id = id.parse::<i64>().map_err(|e| {
            eprintln!("Failed to parse flag ID as i64: {}", e);
            rusqlite::Error::QueryReturnedNoRows
        })?;
    
        let result = self.conn.execute("DELETE FROM flags WHERE id = ?1", params![parsed_id]);

        match result {
            Ok(_) => Ok(()), 
            Err(err) => {
                eprintln!("Error deleting flag: {:?}", err);
                Err(err)
            }
        }
    }
}

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