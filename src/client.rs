use std::io::prelude::*;
use std::net::TcpStream;
use md5::Digest;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let message = r#"{"SubscribeResult":{"result":{"Err":"InvalidName"}}}"#;

    println!("{}", message.len());
    let len = message.len() as u32;
    stream.write(&len.to_le_bytes()).unwrap(); // on écrit le préfixe (taille du prochain message)
    stream.write(message.as_bytes()).unwrap(); // puis le message en tant que tel
}
