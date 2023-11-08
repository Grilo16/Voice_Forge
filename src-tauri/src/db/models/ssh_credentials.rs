use regex::Regex;
use serde::{Serialize, Deserialize};
use rusqlite::types::{FromSql, ValueRef, FromSqlError};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct SshCredentials {
    pub id: i64,
    pub username: String,
    pub host: String,
    pub job_name: String,
    pub machine_type: String,
    pub product: String,
    pub ssh_command: String,
}

impl SshCredentials {
    pub fn new(id: i64, username: String, host: String, ssh_command: String, job_name: String, machine_type: String, product: String) -> Self {
        SshCredentials {
            id,
            username,
            host,
            job_name,
            machine_type,
            product,
            ssh_command,
        }
    }

    pub fn from_ssh_string(string: &str, job_name: &str, machine_type: &str, product: &str) -> Option<SshCredentials> {
        let (username, host) = Self::parse_username_host(string)?;
        let id = 0;
        let ssh_command = format!("ssh -i /Users/jemimagoodall/.aws_ssh/jemima_aws23.pem.txt {}@{}", username, host);
        Some(Self::new(id, username, host, ssh_command, job_name.to_string(), machine_type.to_string(), product.to_string()))
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
        let job_name: String =  value.as_str()?.to_string();
        let machine_type: String = value.as_str()?.to_string();
        let product: String =  value.as_str()?.to_string();
        let ssh_command: String =  value.as_str()?.to_string();

        Ok(SshCredentials {
            id,
            username,
            host,
            job_name,
            machine_type,
            product,
            ssh_command,
        })
    }
}