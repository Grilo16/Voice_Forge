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
