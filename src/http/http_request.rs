use std::fmt::{self};
/*
This part contains the code for the request structs and implementations
*/

pub struct HttpRequest{
    pub request_line: Requestline,
    pub headers:RequestHeaders,
    pub body:RequestBody
}

pub struct Requestline{
    pub method:String,
    pub path:String,
    pub http_version:String
}

pub struct RequestHeader{
    pub header:String,
    pub value:String

}
pub struct RequestBody{

}
pub struct RequestHeaders{
    pub headers:Vec<RequestHeader>
}

impl Requestline{
    pub fn new(request_line:&String)->Self{
        Self { 
            method:Self::get_method(request_line),
            path:Self::get_path(request_line),
            http_version:Self::get_http_version(request_line)
            }
    }
    fn get_method(request_line:&String)->String{
        request_line.split(" ").nth(0).unwrap().to_string()
    }
     fn get_path(request_line:&String)->String{
        request_line.split(" ").nth(1).unwrap().to_string()
    }
     fn get_http_version(request_line:&String)->String{
        request_line.split(" ").nth(2).unwrap().to_string()
    }
}

impl RequestHeader{
    pub fn new(header_line:&String)->Self{
        Self {
            header:Self::get_header(header_line),
            value:Self::get_value(header_line)
          }
    }
    fn get_header(header_line:&String)->String{
        header_line.split(": ").nth(0).unwrap().to_string()
    }
    fn get_value(header_line:&String)->String{
        header_line.split(": ").nth(1).unwrap().to_string()
    }
}
impl RequestBody {
    pub fn new()-> Self{
        Self {  }
    }
}
impl RequestHeaders{
    pub fn new(headers:Vec<RequestHeader>)-> Self{
        Self{
            headers    
    }
}
}

impl HttpRequest{
    pub fn new(request_line:Requestline,headers:RequestHeaders,body:RequestBody)-> Self{
        Self { request_line, headers, body}
    }
}

impl fmt::Display for Requestline{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"{} {} {}",self.method,self.path,self.http_version)
    }
}


