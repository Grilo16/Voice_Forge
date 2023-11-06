// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "db/database.rs"]
mod database;
use database::Database;

#[path = "db/repositories/flag_repo.rs"]
mod flag_repo;

#[path = "db/repositories/ssh_cred_repo.rs"]
mod ssh_cred_repo;

#[path = "./db/models/flag.rs"]
mod flag;
use flag::Flag;

#[path = "ssh_session/terminal_handler.rs"]
mod terminal_handler;
use serde_json::to_string;
use terminal_handler::run_launch_machine;


#[path = "ssh_session/session_handler.rs"]
mod session_handler;

#[path = "db/models/ssh_credentials.rs"]
mod ssh_credentials;
use ssh_credentials::SshCredentials;

use session_handler::SshSession;


extern crate regex;

use regex::Regex;

use std::{env, path::{Path, self}};
use serde::Serialize;

use database::{run_sql};
use flag_repo::{get_flag, create_flag, FlagsRepo};
use rusqlite::{Connection, Result, params};
use ssh_cred_repo::SshCredentialsRepo;

// this is ssh : Command output: Retrieving security groups based on your IP address... Launching a on_demand g4dn.xlarge instance with AMI: test_WEB-312 Launched instance i-08bde2c2523254caa Wait until the instance is running... Instance is running! Tagging resources... Finding the public DNS... Public DNS: ec2-35-178-172-93.eu-west-2.compute.amazonaws.com Now trying to ssh into machine... Finish your session by exiting and terminating from AWS console ssh -i /Users/jemimagoodall/.aws_ssh/jemima_aws23.pem.txt ubuntu@ec2-35-178-172-93.eu-west-2.compute.amazonaws.com # Running on MacOs - the above command has also been copied to clipboard Waiting for status being ok (but you can try ssh-ing already!)... Instance status OK!


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// fn test() -> std::io::Result<()> {
//     let path = env::current_dir()?;
//     println!("The current directory is {}", path.display());
//     Ok(())
// }

// fn create_or_open_database() -> Result<Connection, sqlite::Error> {
//     let project_dirs = env::current_dir().unwrap();

//     let data_dir = project_dirs.push("/my_database.db");
//     println!("{} db_path, {} data_dir, {} project_dirs", database_path, data_dir, project_dirs.display());
//     if !data_dir.exists() {
//         // If the database file doesn't exist, create it
//         let connection = sqlite::Connection::open(&database_path)?;
//         connection.execute(
//             "
//             CREATE TABLE IF NOT EXISTS my_table (
//                 id INTEGER PRIMARY KEY,
//                 name TEXT,
//                 value INTEGER
//             )
//             ",
//          )?;
//         Ok(connection)
//     } else {
//         // If the database file already exists, just open it
//         sqlite::Connection::open(&database_path)
//     }
// }


fn open_my_db() -> Connection {
    let path = "../sqlite.db";
    let db = Connection::open(path).unwrap();
    // Use the database somehow...
    println!("{}", db.is_autocommit());
    return db;
}


#[tauri::command]
fn connect_to_db() -> String {
    let path = "./sqlite.db";
    let conn = Connection::open(path).unwrap();
    // conn.execute_batch(
    //     "BEGIN;
    //      CREATE TABLE flags(label TEXT, flag TEXT, type TEXT );
    //      COMMIT;",
    // );
    match conn.execute(
        "INSERT INTO flags (label, flag, type) values (:label, :flag, :type)",
        &[(":label", "i"),(":flag", "-i"),(":type", "number")],
    ) {
        Ok(updated) => format!("{} rows were updated", updated),
        Err(err) => format!("update failed: {}", err),
    }
}


#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

#[tauri::command]
fn insert_data(query: i64) -> String {
    let flag_instance: Flag = Flag {
        id: 0,
        flag: "test".to_string(),
        label:"test".to_string(),
        input_type: "test".to_string(),
        required: false
    };

    let data = create_flag(flag_instance);
    // get_flag(query);
    
    let json_data = match data {
        Ok(data) => serde_json::to_string(&data),
        Err(err) => serde_json::to_string(&ErrorResponse { message: err.to_string() }),
    };
    match json_data {
        Ok(json) => format!("JSON Data: {}", json),
        Err(err) => format!("Failed to serialize to JSON: {}", err),
    }




    
    // match flag_repo::get_flags(query) {
    //     Ok(flags) => format!("Success: {}", flags),
    //     Err(err) => format!("Error: {}", err),
    // }




    // let conn = open_my_db();
    // match conn.execute(
    //     "INSERT INTO flags (label, flag, type) values (:label, :flag, :type)",
    //     &[(":label", "i"),(":flag", "-i"),(":type", "number")],
    // ) {
    //     Ok(updated) => format!("{} rows were updated", updated),
    //     Err(err) => format!("update failed: {}", err),
    // }
}



#[tauri::command]
fn get_cwd() -> String {
    let path = env::current_dir();
    format!("Cwd is :  {:?}", path)
}


fn extract_username_host(text: &str) -> Option<String> {
    let re = Regex::new(r"ubuntu@[^ ]+").unwrap();

    if let Some(cap) = re.find(text) {
        Some(cap.as_str().to_string())
    } else {
        None
    }
}


#[tauri::command]
fn connect_ssh(comand: String ) -> String {
    // let sess_result = create_session();

    // let output4 = match sess_result {
    //     Ok(mut sess) => {
    //         // Now that we have a Session, we can execute the command
    //         execute_command(&mut sess, &comand);
    //         format!("{} Connected", sess.authenticated())
    //     }
    //     Err(err) => format!("{} failed to authenticate", err)
    // };

    // output4


    // match send_terminal_command() {
    //     Ok(result) => format!("Command output: {}", result),
    //     Err(error) => format!("Error: {}", error),
    // }


    let text = "this is ssh : Command output: Retrieving security groups based on your IP address... Launching a on_demand g4dn.xlarge instance with AMI: test_WEB-312 Launched instance i-08bde2c2523254caa Wait until the instance is running... Instance is running! Tagging resources... Finding the public DNS... Public DNS: ec2-35-178-172-93.eu-west-2.compute.amazonaws.com Now trying to ssh into machine... Finish your session by exiting and terminating from AWS console ssh -i /Users/jemimagoodall/.aws_ssh/jemima_aws23.pem.txt ubuntu@ec2-31-478-420-69.eu-west-2.compute.amazonaws.com # Running on MacOs - the above command has also been copied to clipboard Waiting for status being ok (but you can try ssh-ing already!)... Instance status OK!";

    let re = Regex::new(r"ubuntu@[^ ]+").unwrap();

  
    if let Some(matched_string) = extract_username_host(text) {
        let parts: Vec<&str> = matched_string.split("@").collect();
        format!("Match: Username: {}, Host: {}", parts[0], parts[1])
    }else {
        format!("found nothing")
    }
}



#[tauri::command]
fn just_do_it() -> String {
    //  let connectionString = send_terminal_command()?;
     
    //  let re = Regex::new(r"ubuntu@[^ ]+").unwrap();

    //  if let Some(matched_string) = extract_username_host(&connectionString) {
    //     let parts: Vec<&str> = matched_string.split("@").collect();
    //     let username: parts[0];
    //     let host: parts[2];
    //     format!("Match: Username: {}, Host: {}", parts[0], parts[1])
    // }else {
    //     format!("found nothing")
    // }

    // let command = "tmux new-session -d -s my_session; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py -i Asfas -d 2021-14 -n 99 --accent London --donorid Anything --donorvb --dry ".to_string();
    // let sess_result = create_session(host);
    // let output4 = match sess_result {
    //         Ok(mut sess) => {
    //             // Now that we have a Session, we can execute the command
    //             execute_command(&mut sess, &command);
    //             format!("{} Connected", sess.authenticated())
    //         }
    //         Err(err) => format!("{} failed to authenticate", err)
        // };
    
        // output4


        let connection_string = run_launch_machine().unwrap();
        let raw_credentials = extract_username_host(&connection_string);

        if let Some(matched_string) = raw_credentials {

            if let Some(credentials) = SshCredentials::from_ssh_string(&matched_string) {

                let command = "tmux new-session -d -s my_session; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py -i Asfas -d 2021-14 -n 99 --accent London --donorid Anything --donorvb --dry".to_string();

                match SshSession::new(&credentials) {
                    Ok(mut ssh_session) => {
                        match ssh_session.execute_command(&command) {
                            Ok(output) => {
                                format!("Command output: {}", output)
                            }
                            Err(err) => {
                                format!("Error executing SSH command: {}", err)
                            }
                        }
                    }
                    Err(err) => {
                        format!("Error creating SSH session: {}", err)
                    }
                }
            } else {
                format!("Failed to parse SSH credentials")
            }
        } else {
            format!("Found nothing")
        }
        }

#[tauri::command]
fn launch_cloud_client() -> String{
    // let flag = Flag::new(0, "test again".to_string(), "test flag again".to_string(), "test type input".to_string(), true);
    // let flagRepo = FlagsRepo::new();
    // flagRepo.insert_flag(&flag);

    let ssh_cred_repo = SshCredentialsRepo::new();

    // let flags = flagRepo.get_all_flags().unwrap();

    // for flag in flags{
    //     println!("{}", flag)
    // }

    match run_launch_machine() {
        Ok(output) => {
            if let Some(username_host) = extract_username_host(&output){
                if let Some(credentials) = SshCredentials::from_ssh_string(&username_host) {
                    ssh_cred_repo.insert_ssh_credentials(&credentials);
                    
                    format!("Username and host :  {}", credentials);
                    
                    let command = "tmux new-session -d -s my_session; source ~/puffin_env/bin/activate; python3 ~/spun/repos/speedy/script/run.py -i Asfas -d 2021-14 -n 99 --accent London --donorid Anything --donorvb --dry".to_string();

                    match SshSession::new(&credentials) {
                        Ok(mut ssh_session) => {
                            match ssh_session.execute_command(&command) {
                                Ok(output) => {
                                    format!("Command output: {}", output)
                                }
                                Err(err) => {
                                    format!("Error executing SSH command: {}", err)
                                }
                            }
                        }
                        Err(err) => {
                            format!("Error creating SSH session: {}", err)
                        }
                    }
                }else{
                    "No username or host found, check run.py".to_string()
                }
            }else{
                "No username or host found, check run.py".to_string()
            }
            
        }
        Err(err) => {
            format!("Error : {}", err)
        }
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
                get_cwd, 
                connect_to_db, 
                insert_data, 
                connect_ssh, 
                just_do_it,
                launch_cloud_client
            ])
            .run(tauri::generate_context!())
            .expect("Error while running Tauri application");
    } else {
        eprintln!("Error opening the database");
    }
}
