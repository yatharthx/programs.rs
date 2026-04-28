use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42069")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream).unwrap();
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty()) // reads only till headers
        .collect();

    println!("Request: {http_request:#?}");
    stream.write_all("HTTP/1.1 200 OK\r\n\r\nhey world, this is server!".as_bytes())?;

    Ok(())
}
