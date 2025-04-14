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
    let mut buf_reader=BufReader::new(&stream);
    

    let http_request=parse_http_request(&mut buf_reader);
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
            Response::return_file(&path[7..])
        },
        ("POST",path) if path.starts_with("/files/")=>{
            Response::create_file(http_request.body,&path[7..])
        }
        _ => Response::not_found_response()
    };
    stream.write_all(http_response.to_string().as_bytes()).unwrap();
}

fn parse_http_request(buf_reader:&mut BufReader<&TcpStream>)-> HttpRequest{
    let mut headers=Vec::new();
    let mut line=String::new();

    loop{
        line.clear();
        let bytes_read=buf_reader.read_line(&mut line).unwrap();
        if bytes_read==0 || line=="\r\n"{
            break;
        }
        headers.push(line.trim().to_string());
    }
   // Parse request line and headers
   let first_line = headers.first().cloned().unwrap_or_else(|| "GET /not-found HTTP/1.1".to_string());
   let request_line = Requestline::new(&first_line);

   let mut request_headers_vec = Vec::new();
   for header_line in headers.iter().skip(1) {
       request_headers_vec.push(RequestHeader::new(header_line));
   }
   let headers_obj = RequestHeaders::new(request_headers_vec.clone());

   // Determine content length
   let content_length = request_headers_vec.iter()
   .find(|h| h.header.to_lowercase() == "content-length")
   .and_then(|h| h.value.parse::<usize>().ok())
   .unwrap_or(0);


   // Read body
   let mut body_buf = vec![0; content_length];
   buf_reader.read_exact(&mut body_buf).unwrap_or_default();
   let body = String::from_utf8_lossy(&body_buf).to_string();
   let request_body = RequestBody::new(body);
   HttpRequest::new(request_line, headers_obj, request_body)
}

