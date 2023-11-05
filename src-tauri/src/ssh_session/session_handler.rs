use std::{net::TcpStream, path:: Path, io::Error};

use ssh2::{Session};

use std::io::Read;

 

pub fn create_session() -> Result<Session, Error> {

    
    // let host = "127.0.0.1:2222";
    // let username = "grilo16";
    // let password = "B0oPkkemMakjDA";

    let host = "ec2-3-11-80-53.eu-west-2.compute.amazonaws.com:22";
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

 
 
