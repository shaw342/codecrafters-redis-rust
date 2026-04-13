#![allow(unused_imports)]
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buffer = [0; 512];

                let lignes: Vec<&[u8]> = buffer.split(|x| x == b'\n').collect();

                for ligne in lignes {
                    match _stream.read(&mut ligne) {
                        Ok(_n) => {
                            println!("{:?}", ligne);
                            let _ = _stream.write_all(b"+PONG\r\n");
                        }

                        Err(e) => {
                            println!("error: {}", e)
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
