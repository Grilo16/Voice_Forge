use core::fmt;
use std::process::{Command, Output};
use std::string::String;
use std::error::Error;


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

#[derive(Debug)]
enum ConversionError{
    Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::Utf8Error(err) => write!(f, "UTF-8 conversion error: {}", err)
        }
    }
}

impl Error for ConversionError {}


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

pub fn run_launch_machine() -> Result<String, Box<dyn Error>> {
    
    match launch_machine_command() {
        Ok(output) => {
            let result = String::from_utf8(output.stdout).map_err(|e| LaunchError::ConversionError(Box::new(e)))?;
            Ok(result)
        }
        Err(err) => {
            eprintln!("command execution failed with eerror : {}", err);
            Err(Box::new(err))
        }

    }

    // let output = if cfg!(target_os = "windows") {
    //     Command::new("powershell")
    //         .args(&[
    //             "cd",
    //             "~/spun/repos/su_cloud_scripts",   
    //             ";",                              
    //             "python launch.py",             
    //             "--machine", "t2.small",
    //             "--product", "TTS_deploy",
    //             "--names", "s04vXu0Qv_repair"
    //         ])
    //         .output()
    // } else {
    //     Command::new("sh")
    //     .args(&[
    //         "-c",
    //         "cd ~/spun/repos/su_cloud_scripts && python3 launch.py --machine t2.small --product TTS_deploy --name s04vXu0Qv_repair"
    //     ])  
    //         .output()
    // }?;

    // if output.status.success() {
    //     let hello = String::from_utf8(output.stdout).map_err(|e| {
    //         Error::new(ErrorKind::Other, format!("Error converting to string: {}", e))
    //     })?;
    //     Ok(hello.to_string())
    // } else {
    //     let stderr = String::from_utf8(output.stderr).map_err(|e| {
    //         Error::new(ErrorKind::Other, format!("Error converting stderr to string: {}", e))
    //     })?;
    //     eprintln!("Command execution failed with error: {}", stderr);
    //     Err(Error::new(ErrorKind::Other, "Command execution failed"))
    // }
}

