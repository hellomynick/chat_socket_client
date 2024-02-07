use std::{
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    net::TcpStream,
    thread,
};

static HOST_SERVER: &str = "127.0.0.1:8080";

struct Client {
    name: String,
}

fn main() {
    let stream = TcpStream::connect(HOST_SERVER);

    match stream {
        Ok(mut stream) => {
            println!("Connected to {}", HOST_SERVER);

            let client = Client {
                name: String::from("Anonymous"),
            };
            let mut stream_reader = stream.try_clone().unwrap();
            thread::spawn(move || loop {
                let mut response = vec![0, 32];
                match stream_reader.read_exact(&mut response) {
                    Ok(_) => {
                        let msg = response.into_iter().take_while(|&x| x != 0).collect();
                        let mut msg = String::from_utf8(msg).expect("Can not convert to string");

                        println!("Response from server: {}", msg);
                        msg.clear();
                    }
                    Err(_) => {
                        println!("Connection is close");
                    }
                }
            });

            let mut buffer = String::new();
            println!("Please input your message");
            loop {
                io::stdin().read_line(&mut buffer).unwrap();
                let mut buf = buffer.clone().into_bytes();
                buf.resize(32, 0);
                stream.write_all(&buf).unwrap();
                buffer.clear();
            }
        }
        Err(err) => {
            println!("Cannot connected, {}", err);
        }
    }
}
