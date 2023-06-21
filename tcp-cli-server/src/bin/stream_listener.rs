use std::io::{Read, Write};

fn handle_client_stream(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    let mut received_data = Vec::new();
    loop {
        println!("Waiting for client to send the message ");
        if let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                // connection closed, or no byte is there to read
                break;
            }
            println!("extending the stream data");
            // collect the received bytes
            received_data.extend_from_slice(&buffer[..bytes_read]);
            if buffer[..bytes_read].iter().any(|b| *b == 10) {
                break;
            }
        }
    }

    // Convert received bytes to a string
    let received_string = String::from_utf8_lossy(&received_data);
    println!("Received Data: {}", received_string);
    stream.write(&received_data).unwrap();
}

fn main() {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:8080").expect("Failed to bind with the TcpListener");
    println!("Server started, waiting for client to connect");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("called handle client");
                std::thread::spawn(move || handle_client_stream(stream));
            }
            Err(e) => {
                eprintln!("Error in accepting the connection: {e}")
            }
        }
    }
}
