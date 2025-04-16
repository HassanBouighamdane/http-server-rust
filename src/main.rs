//#![allow(unused_imports)]
#![allow(dead_code)]
mod response;
mod http;
mod thread_pool;
mod utils;
use std::
        { io::{prelude::*, BufReader}, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    };

use thread_pool::ThreadPool;
use response as Response;
use utils::parse_http_request;

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
    //stream.set_read_timeout(Some(Duration::from_secs(30)));
    loop {
        let mut buf_reader=BufReader::new(&stream);
    

    let http_request=parse_http_request(&mut buf_reader);
    //Request line 
    let request_line=&http_request.request_line;
    let path=request_line.path.as_str();
    let method=request_line.method.as_str();

    let http_response=match (method,path){
       ("GET","/")=> {
        Response::success_response(&http_request)
    },
       ("GET",path) if path.starts_with("/echo/") => {
            let text=&path[6..];
            Response::echo_text(&http_request,text)
        },
        ("GET",path)  if path.starts_with("/user-agent")=>{
            Response::user_agent(&http_request)
        },
        ("GET",path)  if path.starts_with("/files/")=>{
            Response::return_file(&http_request,&path[7..])
        },
        ("POST",path) if path.starts_with("/files/")=>{
            Response::create_file(&http_request,&path[7..])
        }
        _ => Response::not_found_response(&http_request)
    };
    if let Err(e) = stream.write_all(http_response.to_string().as_bytes()) {
        println!("Error writing response: {}", e);
        break;
    }
    if let Err(e)= stream.flush(){
        println!("Error flushing stream: {e}");
        break;
    }
    let mut close_connection:bool=false;
    for header in  &http_request.headers.headers{
        if header.header.to_lowercase()=="connection" && header.value.to_lowercase()=="close"{
            close_connection=true;
            break;
    }
    }
    if close_connection {
        break;
    }
}
    
}

