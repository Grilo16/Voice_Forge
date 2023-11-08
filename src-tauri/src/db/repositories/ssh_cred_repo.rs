use crate::SshCredentials;
use crate::Database;
use rusqlite::{Connection, Result, Error, params};

pub struct SshCredentialsRepo {
    conn: Connection,
}

impl SshCredentialsRepo {
    pub fn new() -> Self {
        let db = Database::open().expect("Failed to open database");
        Self { conn: db.get_connection() }
    }

    pub fn insert_ssh_credentials(&self, credentials: &SshCredentials) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO ssh_credentials (username, host, job_name, machine_type, product, ssh_command)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![&credentials.username, &credentials.host, &credentials.job_name, &credentials.machine_type, &credentials.product, &credentials.ssh_command],
        )?;
        Ok(())
    }

    pub fn get_ssh_credentials(&self, id: i64) -> Result<Option<SshCredentials>, Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM ssh_credentials WHERE id = ?1")?;
        let credentials = stmt.query_row(params![id], |row| {
            Ok(SshCredentials {
                id: row.get(0)?,
                username: row.get(1)?,
                host: row.get(2)?,
                job_name: row.get(3)?,
                machine_type: row.get(4)?,
                product: row.get(5)?,
                ssh_command: row.get(6)?,
            })
        });

        match credentials {
            Ok(credentials) => Ok(Some(credentials)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn get_all_ssh_credentials(&self) -> Result<Vec<SshCredentials>, Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM ssh_credentials")?;
        let credentials_iter = stmt.query_map([], |row| {
            Ok(SshCredentials {
                id: row.get(0)?,
                username: row.get(1)?,
                host: row.get(2)?,
                job_name: row.get(3)?,
                machine_type: row.get(4)?,
                product: row.get(5)?,
                ssh_command: row.get(6)?,
            })
        })?;

        let mut credentials = Vec::new();

        for credential in credentials_iter {
            match credential {
                Ok(credential) => {
                    credentials.push(credential);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(credentials)
    }

    pub fn update_ssh_credentials(&self, credentials: &SshCredentials) -> Result<(), Error> {
        self.conn.execute(
            "UPDATE ssh_credentials
             SET username = ?2, host = ?3, job_name = ?4, machine_type = ?5, product = ?6, ssh_command = ?7
             WHERE id = ?1",
            params![credentials.id, &credentials.username, &credentials.host, &credentials.job_name, &credentials.machine_type, &credentials.product, &credentials.ssh_command],
        )?;
        Ok(())
    }

    pub fn delete_ssh_credentials(&self, id: i64) -> Result<(), Error> {
        let result = self.conn.execute("DELETE FROM ssh_credentials WHERE id = ?1", params![id]);

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Error deleting SSH credentials: {:?}", err);
                Err(err)
            }
        }
    }
}