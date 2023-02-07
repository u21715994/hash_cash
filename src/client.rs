use std::io::prelude::*;
use std::net::TcpStream;
use md5::Digest;
use serde_json::Value;

mod message;
use message::Message;
use crate::message::Challenge::MD5HashCash;
use crate::message::ChallengeInput;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let message = r#""Hello""#;

    println!("{}", message.len());
    let len = message.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap(); // on écrit le préfixe (taille du prochain message)
    stream.write(message.as_bytes()).unwrap(); // puis le message en tant que tel

    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

// Envoyer le message Hello au serveur

    loop {
        let hello_message = Message::Hello;
        let hello_json = serde_json::to_string(&hello_message).unwrap();
        let hello_len = hello_json.len() as u32;
        let hello_len_buf = hello_len.to_be_bytes();
        stream.write_all(&hello_len_buf).unwrap();
        stream.write_all(hello_json.as_bytes()).unwrap();

        let mut len_buf = [0u8; 4];
        let _ = stream.read_exact(&mut len_buf).unwrap();
        let len = u32::from_be_bytes(len_buf);

        let mut message_buf = vec![0u8; len as usize];
        let _ = stream.read_exact(&mut message_buf).unwrap();

        let json_value: Value = serde_json::from_slice(&message_buf).unwrap();
        let message: Message = serde_json::from_value(json_value).unwrap();

        match message {
            Message::Welcome { version } => {
                // handle "Welcome" message with version field

                // Envoyer le message Subscribe au serveur
                let subscribe_message = Message::Subscribe {name: "r".to_string()};
                let subscribe_json = serde_json::to_string(&subscribe_message).unwrap();
                let subscribe_len = subscribe_json.len() as u32;
                let subscribe_len_buf = subscribe_len.to_be_bytes();
                stream.write_all(&subscribe_len_buf).unwrap();
                stream.write_all(subscribe_json.as_bytes()).unwrap();
            },
            Message::Challenge(MD5HashCash(ChallengeInput {
                complexity: u8,
                message: String,
                }))=>{
                println!("challenge recu");
            }
            _ => {}
        }
    }
}
