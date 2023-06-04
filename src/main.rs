use std::net::{TcpListener as tcp, TcpStream};
use std::io::{prelude::*,BufReader};
use std::fs;

fn main() {
    let listener = tcp::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        process_request(stream);
        
    }
}

fn process_request(mut stream: TcpStream)
{
    let buff_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buff_reader.lines()
    .map(|result|{result.unwrap()})
    .take_while(|line|{! line.is_empty()})
    .collect();

    // let response = "HTTP/1.1 200 ok \r\n Content-Type: application/json \r\n\r\n";
    // let json_response = r#"{"Mina":"is Awesome"}"#;
    // let response = format!("{} {}",response,json_response.escape_default());
    let contents = fs::read_to_string("index.html").unwrap();
    let contents_len = contents.len();
    let status_line = "HTTP/1.1 200 loading index";
    let headers = format!("Content-Length: {}",contents_len);
    let response = format!("{}\r\n{}\r\n\r\n{}",status_line,headers,contents);
    stream.write_all(response.as_bytes()).unwrap()
}