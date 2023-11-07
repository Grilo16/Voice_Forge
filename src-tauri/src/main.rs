// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#[path = "db/database.rs"]
mod database;
#[path = "db/repositories/flag_repo.rs"]
mod flag_repo;
#[path = "db/repositories/ssh_cred_repo.rs"]
mod ssh_cred_repo;
#[path = "./db/models/flag.rs"]
mod flag;
#[path = "ssh_session/terminal_handler.rs"]
mod terminal_handler;
#[path = "ssh_session/session_handler.rs"]
mod session_handler;
#[path = "db/models/ssh_credentials.rs"]
mod ssh_credentials;



use database::Database;
use terminal_handler::run_launch_machine;
use session_handler::SshSession;

use flag::Flag;
use flag_repo::{FlagsRepo};

use ssh_credentials::SshCredentials;
use ssh_cred_repo::SshCredentialsRepo;



// #[tauri::command]
// fn just_do_it() -> String {
//     let connection_string = run_launch_machine().unwrap();
//     let raw_credentials = extract_username_host(&connection_string);

//     if let Some(matched_string) = raw_credentials {

//         if let Some(credentials) = SshCredentials::from_ssh_string(&matched_string) {

//             let command = "tmux new-session -d -s my_session; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py -i Asfas -d 2021-14 -n 99 --accent London --donorid Anything --donorvb --dry".to_string();

//             match SshSession::new(&credentials) {
//                 Ok(mut ssh_session) => {
//                     match ssh_session.execute_command(&command) {
//                         Ok(output) => {
//                             format!("Command output: {}", output)
//                         }
//                         Err(err) => {
//                             format!("Error executing SSH command: {}", err)
//                         }
//                     }
//                 }
//                 Err(err) => {
//                     format!("Error creating SSH session: {}", err)
//                 }
//             }
//         } else {
//             format!("Failed to parse SSH credentials")
//         }
//     } else {
//         format!("Found nothing")
//     }
// }
// 0Sh1g0t0! jays pc pass
#[tauri::command]
fn launch_cloud_client() -> String{
    
    let ssh_cred_repo = SshCredentialsRepo::new();

    let output = match run_launch_machine() {
        Ok(output) => output,
        Err(err) => return format!("Error: {}", err),
    };

    let credentials = match SshCredentials::from_ssh_string(&output) {
        Some(credentials) => credentials,
        None => return "No username or host found, check run.py".to_string(),
    };

    let _ = ssh_cred_repo.insert_ssh_credentials(&credentials);

    let command = "tmux new-session -d -s my_session; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py -i Asfas -d 2021-14 -n 99 --accent London --donorid Anything --donorvb --dry".to_string();

    let mut ssh_session = match SshSession::new(&credentials) {
        Ok(ssh_session) => ssh_session,
        Err(err) => return format!("Error creating SSH session: {}", err),
    };

    match ssh_session.execute_command(&command) {
        Ok(output) => format!("Command output: {}", output),
        Err(err) => format!("Error executing SSH command: {}", err),
    }
    
    // let ssh_cred_repo = SshCredentialsRepo::new();
    
    // match run_launch_machine() {
    //     Ok(output) => {
    //             if let Some(credentials) = SshCredentials::from_ssh_string(&output.unwrap()) {
    //                 ssh_cred_repo.insert_ssh_credentials(&credentials);
                    
    //                 let command = "tmux new-session -d -s my_session; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py -i Asfas -d 2021-14 -n 99 --accent London --donorid Anything --donorvb --dry".to_string();

    //                 match SshSession::new(&credentials) {
                        
    //                     Ok(mut ssh_session) => {
                           
    //                         match ssh_session.execute_command(&command) {
                                
    //                             Ok(output) => {
    //                                 format!("Command output: {}", output)
    //                             }
                                
    //                             Err(err) => {
    //                                 format!("Error executing SSH command: {}", err)
    //                             }
    //                         }
    //                     }
    //                     Err(err) => {
    //                         format!("Error creating SSH session: {}", err)
    //                     }
    //                 }
    //             }else{
    //                 "No username or host found, check run.py".to_string()
    //             }
    //     }
    //     Err(err) => {
    //         format!("Error : {}", err)
    //     }
    // }
}



fn main() {
    if let Ok(db) = Database::open() {
        if let Err(err) = db.create_tables() {
            eprintln!("Error creating tables: {:?}", err);
            return;
        }
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![
                launch_cloud_client
            ])
            .run(tauri::generate_context!())
            .expect("Error while running Tauri application");
    } else {
        eprintln!("Error opening the database");
    }
}
