use std::{net::TcpStream, path:: Path};

use ssh2::{Session};

use std::io::Read;

 

pub fn create_session() -> Session {

    let host = "ec2-3-11-80-53.eu-west-2.compute.amazonaws.com:22";

    let username = "ubuntu";

    let password = "B0oPkkemMakjDA";

    // ssh -i /Users/jemimagoodall/.aws_ssh/jemima_aws23.pem.txt   

 

    let tcp = TcpStream::connect(host).unwrap();

    let mut sess = Session::new().unwrap();

    sess.set_tcp_stream(tcp);

    sess.handshake().unwrap();

    sess.userauth_pubkey_file(username, None, Path::new("/Users/jemimagoodall/.aws_ssh/jemima_aws23.pem.txt"), None).unwrap();

    assert!(sess.authenticated());
    println!("Authenticated: {}", sess.authenticated());
    sess
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

 
 

