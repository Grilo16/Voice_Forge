// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "db/database.rs"]
mod database;

#[path = "db/repositories/flag_repo.rs"]
mod flag_repo;
#[path = "./db/models/flag.rs"]
mod flag;
use flag::Flag;

#[path = "ssh_session/terminal_handler.rs"]
mod terminal_handler;
use terminal_handler::send_terminal_command;


#[path = "ssh_session/session_handler.rs"]
mod session_handler;

use session_handler::{create_session, execute_command};


use std::{env, path::{Path, self}};
use serde::Serialize;

use database::{initialize_db, run_sql};
use flag_repo::{get_flag, create_flag};
use rusqlite::{Connection, Result, params};

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


fn create_table(conn: Connection){
    conn.execute_batch(
        "BEGIN;
         CREATE TABLE flags(label TEXT, flag TEXT, type TEXT );
         COMMIT;",
    );
}
  
#[tauri::command]
fn create_tables() -> String {
    let conn = open_my_db();
    create_table(conn);

    format!("tables created successfully")
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
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

#[tauri::command]
fn connect_ssh(comand: String, comand2: String ) -> String {
    let sess_result = create_session();

    let output4 = match sess_result {
        Ok(mut sess) => {
            // Now that we have a Session, we can execute the command
            execute_command(&mut sess, &comand);
            execute_command(&mut sess, &comand2);
            format!("{} Connected", sess.authenticated())
        }
        Err(err) => format!("{} failed to authenticate", err)
    };

    output4


    // match send_terminal_command() {
    //     Ok(result) => format!("Command output: {}", result),
    //     Err(error) => format!("Error: {}", error),
    // }


}



fn main() {
    let _ = initialize_db();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_cwd, connect_to_db, create_tables, insert_data, connect_ssh])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
