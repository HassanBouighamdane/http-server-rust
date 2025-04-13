#[allow(unused_imports)]

use std::
        {io::{prelude::*, BufReader}, 
        net::{TcpListener,Ipv4Addr, SocketAddrV4, TcpStream},
        thread,
        time::Duration
    };
mod response;
mod http;
use http::http_request::{ HttpRequest, RequestBody, RequestHeader, RequestHeaders, Requestline};
use response as Response;
fn main() {

    let addr=SocketAddrV4::new(Ipv4Addr::new(127, 0,0, 1), 4221);
    let listener = TcpListener::bind(addr).unwrap();
    //This also works 
    //let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("The server is up at: http://{}",addr.to_string());

     for stream in listener.incoming() {
         match stream {
         Ok(_stream) => {
            thread::spawn(||{
                handle_connection(_stream);
            });
             }
             Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader=BufReader::new(&stream);
    let http_request_lines:Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let http_request=parse_http_request(http_request_lines);
    //Request line 
    let request_line=http_request.request_line;
    let path=request_line.path.as_str();
    let method=request_line.method.as_str();

    let http_response=match (method,path){
       ("GET","/")=> {
        Response::success_response()
    },
       ("GET",path) if path.starts_with("/echo/") => {
            let text=&path[6..];
            Response::echo_text(text)
        },
        ("GET",path)  if path.starts_with("/user-agent")=>{
            Response::user_agent(http_request.headers)
        }
        _ => Response::not_found_response()
    };
    stream.write_all(http_response.to_string().as_bytes()).unwrap();
}

fn parse_http_request(lines:Vec<String>)-> HttpRequest{
    let request_line=Requestline::new(lines.first().unwrap());
    let mut request_headers: Vec<RequestHeader>=Vec::new();
    for line in lines.iter().skip(1){
        let header=RequestHeader::new(line);
        request_headers.push(header);
    };
    let headers=RequestHeaders::new(request_headers);
    let body=RequestBody::new();

    HttpRequest::new(request_line,headers,body)
}
