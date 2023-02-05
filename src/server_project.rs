use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {


    let message = "hello";

    println!("{}", message.len());
    let len = message.len() as u32;
    stream.write(&len.to_le_bytes()).unwrap(); // on écrit le préfixe (taille du prochain message)
    stream.write(message.as_bytes()).unwrap(); // puis le message en tant que tel
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
