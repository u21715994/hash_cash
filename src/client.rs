use std::io::prelude::*;
use std::net::TcpStream;
use md5::Digest;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let message = r#""Hello""#;

    println!("{}", message.len());
    let len = message.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap(); // on écrit le préfixe (taille du prochain message)
    stream.write(message.as_bytes()).unwrap(); // puis le message en tant que tel

    let mut buf_len = [0u8; 4];
    stream.read_exact(buf_len.as_mut()).unwrap(); // lit exactement la taille du buffer

    let len = u32::from_le_bytes(buf_len); // convertit les 4 octets en un entier u32

    let mut buf = vec![0u8; len as usize]; // on prépare un buffer pour ce qui va arriver
    stream.read(buf.as_mut()).unwrap(); // on remplit le buffer
    // c'est arrivé
    println!("{buf:?}"); // en octets
    let s = String::from_utf8_lossy(&buf);
    println!("{:}", s)
}
