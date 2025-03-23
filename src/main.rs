use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    println!("Listening on port 8080");
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        println!("Starting connection");
        match stream {
            Ok(mut stream) => {
                loop {
                    let mut read = [0; 1028];
                    match stream.read(&mut read) {
                        Ok(n) => {
                            if n == 0 {
                                // connection closed
                                break;
                            }
                            let _ = stream.write(&read[0..n]);
                        }
                        Err(err) => panic!("{}", err),
                    }
                }
            }
            Err(err) => panic!("{}", err),
        }
    }
}
