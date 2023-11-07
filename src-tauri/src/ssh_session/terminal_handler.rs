use std::process::{Command, Output};
use std::string::String;
use std::error::Error;

use regex::Regex;

use crate::ssh_cred_repo::SshCredentialsRepo;
use crate::ssh_credentials::SshCredentials;


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

fn launch_machine_command(launch_flags: String) -> Result<Output, LaunchError>{
    if cfg!(target_os = "windows") {
        let mut windows_args = vec![
            "cd",
            "~/spun/repos/su_cloud_scripts",
            ";",
            "python",
            "launch.py",
            ]; 

            let windows_flags: Vec<&str> = launch_flags.split(" ").collect();
            windows_args.extend(windows_flags.iter().cloned());
            run_command("powershell", &windows_args)
    } else {
        let mut mac_args = vec![
            "-c"
        ];
        let mut mac_flags = "cd ~/spun/repos/su_cloud_scripts && python3 launch.py".to_string();
        mac_flags.push_str(&launch_flags);
        mac_args.push(&mac_flags);
        run_command("sh",&mac_args)
    }
}
// fn launch_machine_command() -> Result<Output, LaunchError>{
//     if cfg!(target_os = "windows") {
//         run_command(
//             "powershell",
//             &[
//                 "cd",
//                 "~/spun/repos/su_cloud_scripts",
//                 ";",
//                 "python",
//                 "launch.py",
//                 "--machine",
//                 "t2.small",
//                 "--product",
//                 "TTS_deploy",
//                 "--names",
//                 "s04vXu0Qv_repair",
//             ],
//         )
//     } else {
//         run_command(
//             "sh",
//             &[
//                 "-c",
//                 "cd ~/spun/repos/su_cloud_scripts && python3 launch.py --machine t2.small --product TTS_deploy --name s04vXu0Qv_repair",
//             ],
//         )
//     }
// }


fn extract_username_host(text: &str) -> Result<&str, Box<dyn Error>> {
    let re = Regex::new(r"ubuntu@[^ ]+").map_err(|e| Box::new(e) as Box<dyn Error>)?;

    if let Some(cap) = re.find(text) {
        Ok(cap.as_str())
    } else {
        Err("Pattern not found in text".into())
    }
}


pub fn run_launch_machine(launch_flags: String) -> Result<SshCredentials, Box<dyn Error>> {
    let result = match launch_machine_command(launch_flags) {
        Ok(output) => {
            String::from_utf8(output.stdout).map_err(|e| LaunchError::ConversionError(Box::new(e)))?
        }
        Err(err) => {
            eprintln!("Command execution failed with error: {}", err);
            return Err(Box::new(err));
        }
    };
    
    let username_host = extract_username_host(&result)?;

    match SshCredentials::from_ssh_string(&username_host) {
        Some(credentials) => {
            let ssh_cred_repo = SshCredentialsRepo::new();
            let _ = ssh_cred_repo.insert_ssh_credentials(&credentials);
            
            Ok(credentials)
        }
        None => Err(Box::new(LaunchError::ConversionError("No username or host found, check run.py".into())))
    }
    
}

