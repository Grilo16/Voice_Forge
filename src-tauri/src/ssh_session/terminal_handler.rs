use std::process::Command;
use std::io::{Result, Error, ErrorKind};
use std::string::String;

pub fn send_terminal_command() -> Result<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&[
                "cd",
                "~/spun/repos/su_cloud_scripts",   
                ";",                              
                "python launch.py",             
                "--machine", "g4dn.xlarge",
                "--product", "TTS_deploy",
                "--names", "s04vXu0Qv_repair"
            ])
            .output()
    } else {
        Command::new("sh")
        .args(&[
            "-c",
            "cd ~/spun/repos/su_cloud_scripts && python launch.py --machine g4dn.xlarge --product TTS_deploy --names s04vXu0Qv_repair"
        ])  
            .output()
    }?;

    if output.status.success() {
        let hello = String::from_utf8(output.stdout).map_err(|e| {
            Error::new(ErrorKind::Other, format!("Error converting to string: {}", e))
        })?;
        Ok(hello.to_string())
    } else {
        let stderr = String::from_utf8(output.stderr).map_err(|e| {
            Error::new(ErrorKind::Other, format!("Error converting stderr to string: {}", e))
        })?;
        eprintln!("Command execution failed with error: {}", stderr);
        Err(Error::new(ErrorKind::Other, "Command execution failed"))
    }
}

