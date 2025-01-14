use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::io::BufReader;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
// fn handle_connection(mut stream: TcpStream) {
//     // let mut buffer = [0; 512];
//     // stream.read(&mut buffer).unwrap();

//     // let buf_reader = BufReader::new(&mut stream);


//     let contents = fs::read_to_string("hello.html").unwrap();
//     println!("{:?}", contents);
//     let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
//     println!("{:?}", response);
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let status_line = "HTTP/1.1 200 OK\r\n\r\n";
//     let contents = fs::read_to_string("hello.html").unwrap();

//     let response = format!("{status_line}{}", contents);

//     stream.write_all(response.as_bytes()).unwrap();
// }

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    // stream.read(&mut buffer).unwrap();

    let _ =  BufReader::new(&mut stream).read(&mut buffer);

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let contents = fs::read_to_string("404.html").unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    }
}