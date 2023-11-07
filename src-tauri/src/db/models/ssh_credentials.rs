use regex::Regex;
use serde::{Serialize, Deserialize};
use rusqlite::types::{FromSql, ValueRef, FromSqlError};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct SshCredentials {
    pub id: i64,
    pub username: String,
    pub host: String,
    pub ssh_command: String,
}

impl SshCredentials {
    pub fn new(id: i64, username: String, host: String, ssh_command: String) -> Self {
        SshCredentials {
            id,
            username,
            host,
            ssh_command,
        }
    }

    pub fn from_ssh_string(string : &str) -> Option<SshCredentials> {
        let (username, host) = Self::parse_username_host(string)?;
        let id = 0;
        let ssh_command = format!("-ssh {}@{}", username, host);
        Some(Self::new(id, username, host, ssh_command))
    }

    pub fn parse_username_host(string: &str) -> Option<(String, String)> {
        let re = Regex::new(r"([^@]+)@([^ ]+.com)").unwrap();
        if let Some(credentials) = re.captures(string) {
            let username = credentials[1].to_string();
            let host = credentials[2].to_string();
            Some((username, host))
        } else {
            None
        }
    } 

}

impl fmt::Display for SshCredentials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}, username: {}, host: {}, ssh_command: {}",
            self.id, self.username, self.host, self.ssh_command
        )
    }
}

impl FromSql for SshCredentials {
    fn column_result(value: ValueRef) -> Result<SshCredentials, FromSqlError> {
        let id: i64 = value.as_i64()?;
        let username: String = value.as_str()?.to_string();
        let host: String =  value.as_str()?.to_string();
        let ssh_command: String =  value.as_str()?.to_string();

        Ok(SshCredentials {
            id,
            username,
            host,
            ssh_command,
        })
    }
}