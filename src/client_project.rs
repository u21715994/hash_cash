use std::io::prelude::*;
use std::net::TcpStream;
use md5::Digest;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let mut buf_len = [0u8; 4]; // pour lire les 4 octets du u32
    stream.read_exact(buf_len.as_mut()).unwrap(); // lit exactement la taille du buffer

    let len = u32::from_le_bytes(buf_len); // convertit les 4 octets en un entier u32

    let mut buf = vec![0u8; len as usize]; // on prépare un buffer pour ce qui va arriver
    stream.read_exact(buf.as_mut()).unwrap(); // on remplit le buffer
    // c'est arrivé
    println!("{buf:?}"); // en octets
    let s = String::from_utf8_lossy(&buf); // la version loosy n'échoue jamais et nettoie les caractères UTF-8 invalides
    println!("{s}"); // en String

    let mut binary = md5::Md5::new();
    binary.update("000000000000034C".to_owned()+&s.to_string());
    let binary_bin = binary.finalize();
    println!("binary : {:X}", binary_bin);
}
