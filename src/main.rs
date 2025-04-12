#[allow(unused_imports)]
use std::net::TcpListener;
use std::{io::{prelude::*, BufReader}, net::{Ipv4Addr, SocketAddrV4, TcpStream}};
mod response;
mod http;
use http::Requestline;
use response as Response;
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let addr=SocketAddrV4::new(Ipv4Addr::new(127, 0,0, 1), 4221);
    let listener = TcpListener::bind(addr).unwrap();
    //This also works 
    //let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    /* 
    //test the print of the response
    let status_line=Statustline::new(String::from("HTTP/1.1"), 200, String::from("OK"));
    let headers=vec![ResponseHeader::new()];
    let body=ResponseBody::new();
    let response=HttpResponse::new(status_line,headers,body);
    println!("{}",response);
    */

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

    let request_line=Requestline::new(http_request.first().unwrap());
    let path=request_line.path.as_str();
    let http_response=match path{
       "/"=> Response::success_response(),
       path if path.starts_with("/echo/") => {
        let text=&path[6..];
        Response::echo_text(text)
    },
        _ => Response::not_found_response()
    };
    stream.write_all(http_response.to_string().as_bytes()).unwrap();
}
