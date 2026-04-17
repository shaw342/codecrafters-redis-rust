#![allow(unused_imports)]
use std::io::{Read, Write};
use std::net::TcpListener;
use tokio::spawn;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        spawn(async move {
            match stream {
                Ok(mut _stream) => loop {
                    let mut buffer = [0; 512];

                    match _stream.read(&mut buffer) {
                        Ok(_n) => {
                            let _ = _stream.write_all(b"+PONG\r\n");
                        }
                        Err(e) => {
                            println!("error: {}", e)
                        }
                    }
                },

                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}
