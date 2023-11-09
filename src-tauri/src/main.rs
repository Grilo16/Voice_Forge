// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "db/database.rs"]
mod database;
#[path = "db/repositories/flag_repo.rs"]
mod flag_repo;
#[path = "db/repositories/ssh_cred_repo.rs"]
mod ssh_cred_repo;
#[path = "./db/models/terminal_flag.rs"]
mod terminal_flag;
#[path = "ssh_session/terminal_handler.rs"]
mod terminal_handler;
#[path = "ssh_session/session_handler.rs"]
mod session_handler;
#[path = "db/models/ssh_credentials.rs"]
mod ssh_credentials;
use database::Database;
use ssh_cred_repo::SshCredentialsRepo;
use terminal_handler::run_launch_machine;
use session_handler::SshSession;

use terminal_flag::TerminalFlag;
// use flag_repo::FlagsRepo;
use ssh_credentials::SshCredentials;

use serde::Serialize;


#[derive(Serialize)]
struct CommandResult<T> {
    data: Option<T>,
    error: Option<String>,
}


#[tauri::command]
fn get_ssh_credentials() -> CommandResult<Vec<SshCredentials>> {
    let credentials_repo = SshCredentialsRepo::new();
    match credentials_repo.get_all_ssh_credentials() {
        Ok(credentials) => CommandResult { data: Some(credentials), error: None },
        Err(err) => CommandResult { data: None, error: Some(err.to_string()) },
    }
}

// 0Sh1g0t0! jays pc pass
#[tauri::command]
fn launch_cloud_instance() -> String {
// fn launch_cloud_instance(machine: String, product: String, name: String) -> String {
    let machine = "t2.small".to_string();
    let product = "TTS_deploy".to_string();
    let name = "s04vXu0Qv_repair".to_string();
    let launch_flags = format!("--machine {} --product {} --name {}", machine, product, name);
    
    let run_command = "tmux new-session -d -s my_session; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py -i Asfas -d 2021-14 -n 99 --accent London --donorid Anything --donorvb --dry".to_string();

    let ssh_credentials = match run_launch_machine(launch_flags, machine, product, name) {
        Ok(output) => output,
        Err(err) => return format!("Error: {}", err),
    };
    
    let mut ssh_session = match SshSession::new(&ssh_credentials) {
        Ok(ssh_session) => ssh_session,
        Err(err) => return format!("Error creating SSH session: {}", err),
    };
    
    match ssh_session.execute_command(&run_command) {
        Ok(output) => format!("Command output: {}", output),
        Err(err) => format!("Error executing SSH command: {}", err),
    }
}


#[tauri::command]
fn launch_instance(machine: String, product: String, name: String) -> CommandResult<SshCredentials> {
    let launch_flags = format!("--machine {} --product {} --name {}", machine, product, name);
    let ssh_credentials = match run_launch_machine(launch_flags, machine, product, name) {
        Ok(output) => CommandResult { data: Some(output), error: None}, 
        Err(err) => CommandResult { data: None, error: Some(err.to_string())}
    };
    ssh_credentials
}

#[tauri::command]
fn run_tmux_command(ssh_credentials: SshCredentials, run_flags: Vec<String>) -> CommandResult<String> {
    let launch_flags = run_flags.join(" ");

    let mut run_script_command = "touch out.txt; tmux new-session -d -s my_session; source ~/.profile; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py ".to_string();
    run_script_command.push_str(&launch_flags);
    let write_to_txt = " |& tee -a out.txt".to_string();
    run_script_command.push_str(&write_to_txt);


    let mut ssh_session = match SshSession::new(&ssh_credentials) {
        Ok(ssh_session) =>  ssh_session, 
        Err(err) => return CommandResult { data: None, error: Some(err.to_string())}
    };
    
    match ssh_session.execute_command(&run_script_command) {
        Ok(output) => CommandResult { data: Some(output), error: None}, 
        Err(err) => CommandResult { data: None, error: Some(err.to_string())}
    }
}



fn main() {
    if let Ok(db) = Database::open() {
        if let Err(err) = db.create_tables() {
            eprintln!("Error creating tables: {:?}", err);
            return;
        }
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![
                launch_cloud_instance,
                launch_instance,
                run_tmux_command,
                get_ssh_credentials
            ])
            .run(tauri::generate_context!())
            .expect("Error while running Tauri application");
    } else {
        eprintln!("Error opening the database");
    }
}
