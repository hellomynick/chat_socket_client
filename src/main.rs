use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

static HOST_SERVER: &str = "127.0.0.1:8080";

struct Client {
    name: String,
}

fn main() {
    let stream = TcpStream::connect(HOST_SERVER);

    match stream {
        Ok(stream) => {
            println!("Connected to {}", HOST_SERVER);

            let client = Client {
                name: String::from("Anonymous"),
            };

            let mut buffer = String::new();
            println!("Please input your message");
            loop {
                io::stdin().read_line(&mut buffer).unwrap();
                let mut stream_writer =
                    BufWriter::new(stream.try_clone().expect("Stream is expires"));
                stream_writer.write_all(buffer.trim().as_bytes()).unwrap();
            }
        }
        Err(err) => {
            println!("Cannot connected, {}", err);
        }
    }
}
