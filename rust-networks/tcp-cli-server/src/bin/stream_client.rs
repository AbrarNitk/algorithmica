use std::io;
use std::io::{BufRead, Read, Write};

fn main() {
    let mut stream =
        std::net::TcpStream::connect("127.0.0.1:8080").expect("failed to connect with server");
    loop {
        let mut input = String::new();
        println!("Enter Your Message: ");
        io::stdin()
            .lock()
            .read_line(&mut input)
            .expect("Failed to read input");

        // send data to server
        stream
            .write_all(&input.as_bytes())
            .expect("Failed to send data to server");

        // receive data from server
        let mut buffer = [0; 1024];
        let mut received_data = Vec::new();
        loop {
            let bytes_read = stream
                .read(&mut buffer)
                .expect("failed to receive data from server");
            if bytes_read == 0 {
                break;
            }
            received_data.extend_from_slice(&buffer[..bytes_read]);
            let received_string = String::from_utf8_lossy(received_data.as_ref());
            println!("Message from Server: {}", received_string);
        }
    }
}
