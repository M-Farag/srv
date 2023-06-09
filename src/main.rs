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

    
    let (status_line,file_name) = match &http_request[0][..]
    {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 loading index","index.html"),
        "GET /sleep HTTP/1.1" => {
            std::thread::sleep(std::time::Duration::from_secs(10));
            ("HTTP/1.1 200 loading sleep","index.html")
        },
        _ => ("HTTP/1.1 404 Page not found","404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let contents_len = contents.len();
    
    let headers = format!("Content-Length: {}",contents_len);
    let response = format!("{}\r\n{}\r\n\r\n{}",status_line,headers,contents);
    stream.write_all(response.as_bytes()).unwrap()
}