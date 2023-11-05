use std::{net::TcpStream, path:: Path, io::Error};

use ssh2::{Session};

use std::io::Read;

 

pub fn create_session(host_param: &String) -> Result<Session, Error> {

    
    // let host = "127.0.0.1:2222";
    // let username = "grilo16";
    // let password = "B0oPkkemMakjDA";
    // ec2-35-178-172-93.eu-west-2.compute.amazonaws.com
    let host_param_cleaned = host_param.replace("\n", "").replace("#", "");
    let host_param_cleaned_str = host_param_cleaned.trim().to_string(); // Convert to String;
    let host = format!("{}:22", host_param_cleaned_str);
    println!("{}", host);
    let username = "ubuntu";
    let password = "B0oPkkemMakjDA";

    let tcp = TcpStream::connect(host)?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    // sess.userauth_pubkey_file(username, None, Path::new("c:\\") , None )?;
    sess.userauth_pubkey_file(username, None, Path::new("/Users/jemimagoodall/.aws_ssh/jemima_aws23.pem.txt"), None)?;
    assert!(sess.authenticated());
    println!("Authenticated: {}", sess.authenticated());
    Ok(sess)

}


pub fn execute_command(sess: &mut ssh2::Session, command: &str) -> String {

    println!("Command: {}", command);

    let mut channel = sess.channel_session().unwrap();

    channel.exec(command).unwrap();

    let mut s = String::new();

    channel.send_eof();

    channel.wait_close();

    channel.read_to_string(&mut s).unwrap();

    println!("output: {}", s);

    s

}

 
 

