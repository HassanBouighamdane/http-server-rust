#[allow(unused_imports)]
use std::net::TcpListener;
use std::{io::{prelude::*, BufReader}, net::{Ipv4Addr, SocketAddrV4, TcpStream}};
mod response;

use response::success_response;
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let addr=SocketAddrV4::new(Ipv4Addr::new(127, 0,0, 1), 4221);
    let listener = TcpListener::bind(addr).unwrap();
    //This also works 
    //let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

     for stream in listener.incoming() {
         match stream {
         Ok(_stream) => {
                 handle_connection(_stream);
             }
             Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader=BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response=success_response();
    
    stream.write_all(response.as_bytes()).unwrap();
}
