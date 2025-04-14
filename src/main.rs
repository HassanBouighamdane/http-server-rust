//#![allow(unused_imports)]
#![allow(dead_code)]
mod response;
mod http;
mod thread_pool;
mod utils;
use std::
        { io::{prelude::*, BufReader}, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}
    };

use thread_pool::ThreadPool;
use http::http_request::{ HttpRequest, RequestBody, RequestHeader, RequestHeaders, Requestline};
use response as Response;

fn main() {

    let addr=SocketAddrV4::new(Ipv4Addr::new(127, 0,0, 1), 4221);
    let listener = TcpListener::bind(addr).unwrap();
    let pool=ThreadPool::new(4);
    //This also works 
    //let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("The server is up at: http://{}",addr.to_string());
     for stream in listener.incoming() {
         match stream {
         Ok(_stream) => {
            /* 
            //This code also can be used without using the ThreadPool, it's the simple way to implement concurrency
            //The problem is that we create a thread for each connection opened which is not optimal and DDoS attack can 
            //overhelm the server ressources and make the service unavailable

            thread::spawn(||{
                handle_connection(_stream);
            });
            */
          pool.execute(|| {        
                handle_connection(_stream);
          }) 
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
        },
        ("GET",path)  if path.starts_with("/files/")=>{
            Response::return_file(path)
        }
        _ => Response::not_found_response()
    };
    stream.write_all(http_response.to_string().as_bytes()).unwrap();
}

fn parse_http_request(lines:Vec<String>)-> HttpRequest{
    let first_line=match lines.first(){
        Some(line)=> line,
        None=> &String::from("GET /not-found HTTP/1.1")
    };
    let request_line= Requestline::new(&first_line);
    let mut request_headers: Vec<RequestHeader>=Vec::new();
    for line in lines.iter().skip(1){
        let header=RequestHeader::new(line);
        request_headers.push(header);
    };
    let headers=RequestHeaders::new(request_headers);
    let body=RequestBody::new();

    HttpRequest::new(request_line,headers,body)
}

