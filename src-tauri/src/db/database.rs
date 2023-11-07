use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection
}

impl Database {
    pub fn open() -> Result<Self>{
        let conn = Connection::open("../sqlite.db")?;
        Ok(Self {conn})
    }

    pub fn create_tables(&self) -> Result<()> {
        self.conn.execute(
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

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS ssh_credentials (
                id INTEGER PRIMARY KEY,
                username TEXT,
                host TEXT,
                ssh_command TEXT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn get_connection(self) -> Connection {
        self.conn
    }
}
