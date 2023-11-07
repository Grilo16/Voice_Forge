use std::process::{Command, Output};
use std::string::String;
use std::error::Error;

use regex::Regex;


#[derive(Debug)]
enum LaunchError {
    ExecutionFailed,
    ConversionError(Box<dyn Error + Send + Sync>),
}

impl std::fmt::Display for LaunchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            LaunchError::ExecutionFailed => write!(f, "command Execution Failed"),
            LaunchError::ConversionError(err) => write!(f, "Error converting {}", err)
        }
    }
}

impl Error for LaunchError {}

// #[derive(Debug)]
// enum ConversionError{
//     Utf8Error(std::string::FromUtf8Error),
// }

// impl fmt::Display for ConversionError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             ConversionError::Utf8Error(err) => write!(f, "UTF-8 conversion error: {}", err)
//         }
//     }
// }

// impl Error for ConversionError {}


fn run_command(command: &str, args: &[&str]) -> Result<Output, LaunchError>{
    let output = Command::new(command).args(args).output().map_err(|e| {
        let err: Box<dyn Error + Send + Sync> = Box::new(e);
        LaunchError::ConversionError(err)
    })?;
    if output.status.success(){
        Ok(output)
    }else{
        Err(LaunchError::ExecutionFailed)
    }
}

fn launch_machine_command() -> Result<Output, LaunchError>{
    if cfg!(target_os = "windows") {
        run_command(
            "powershell",
            &[
                "cd",
                "~/spun/repos/su_cloud_scripts",
                ";",
                "python",
                "launch.py",
                "--machine",
                "t2.small",
                "--product",
                "TTS_deploy",
                "--names",
                "s04vXu0Qv_repair",
            ],
        )
    } else {
        run_command(
            "sh",
            &[
                "-c",
                "cd ~/spun/repos/su_cloud_scripts && python3 launch.py --machine t2.small --product TTS_deploy --name s04vXu0Qv_repair",
            ],
        )
    }
}


fn extract_username_host(text: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"ubuntu@[^ ]+").map_err(|e| Box::new(e) as Box<dyn Error>)?;

    if let Some(cap) = re.find(text) {
        Ok(cap.as_str().to_string())
    } else {
        Err("Pattern not found in text".into())
    }
}

pub fn run_launch_machine() -> Result<String, Box<dyn Error>> {
    let result = match launch_machine_command() {
        Ok(output) => {
            String::from_utf8(output.stdout).map_err(|e| LaunchError::ConversionError(Box::new(e)))?
        }
        Err(err) => {
            eprintln!("Command execution failed with error: {}", err);
            return Err(Box::new(err));
        }
    };

    extract_username_host(&result)
}

