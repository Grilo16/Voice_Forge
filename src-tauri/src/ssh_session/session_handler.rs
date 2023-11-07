use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
use ssh2::{Session, DisconnectCode};
use std::fmt;
use std::error::Error;
use crate::ssh_credentials::SshCredentials;


#[derive(Debug)]
pub enum SshError {
    AuthenticationFailed,
    Utf8Conversion(std::string::FromUtf8Error),
    SshError(ssh2::Error),
    IoError(std::io::Error),
}

impl fmt::Display for SshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SshError::Utf8Conversion(err) => write!(f, "utf error {}", err),
            SshError::AuthenticationFailed => write!(f, "SSH authentication failed"),
            SshError::SshError(err) => write!(f, "SSH error: {}", err),
            SshError::IoError(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl Error for SshError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SshError::SshError(err) => Some(err),
            SshError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ssh2::Error> for SshError {
    fn from(error: ssh2::Error) -> Self {
        SshError::SshError(error)
    }
}

impl From<std::io::Error> for SshError {
    fn from(error: std::io::Error) -> Self {
        SshError::IoError(error)
    }
}

impl From<std::string::FromUtf8Error> for SshError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        SshError::Utf8Conversion(error)
    }
}

pub struct SshSession {
   session: Session,
}

impl SshSession {
    pub fn new(credentials: &SshCredentials) -> Result<Self, SshError> {
        let host = format!("{}:22", credentials.host);
        let username = format!("{}", credentials.username);

        let tcp = TcpStream::connect(&host)?;
        let mut sess = Session::new()?;

        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        sess.userauth_pubkey_file(&username, None, Path::new("/Users/jemimagoodall/.aws_ssh/jemima_aws23.pem.txt"), None)?;

        if !sess.authenticated() {
            return Err(SshError::AuthenticationFailed);
        }

        Ok(SshSession { session: sess })
    }

    pub fn execute_command(&mut self, command: &str) -> Result<String, SshError> {
        println!("Command: {}", command);
    
        let mut channel = self.session.channel_session()?;
        channel.exec(command)?;
    
        let mut s = Vec::new();
        let mut buf = Vec::new(); 

        loop {
            let count = channel.read_to_end(&mut buf)?;
            if count == 0 {
                break;
            }
            s.extend_from_slice(&buf[..count]);
        }
    
        channel.send_eof()?;
        channel.wait_close()?;
    
        let output = String::from_utf8(s).map_err(SshError::Utf8Conversion)?;
    
        println!("Output: {}", output);
    
        Ok(output)
    }
}

impl Drop for SshSession {
    fn drop(&mut self) {
        self.session.disconnect(Some(DisconnectCode::ByApplication), "Session closed", Some("")).ok();
    }
}

