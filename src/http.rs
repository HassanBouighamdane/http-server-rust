use std::fmt::{self};
/*
This part contains the code for the request structs and implementations
*/

pub struct HttpRequest{
    request_line: Requestline,
    headers:Vec<RequestHeader>,
    body:RequestBody
}

pub struct Requestline{
    pub method:String,
    pub path:String,
    pub http_version:String
}

pub struct RequestHeader{

}
pub struct RequestBody{

}

impl Requestline{
    pub fn new(request_line:&String)->Requestline{
        Requestline { 
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
    fn new()->RequestHeader{
        RequestHeader {  }
    }
}
impl RequestBody {
    fn new()-> RequestBody{
        RequestBody {  }
    }
}
impl HttpRequest{
    fn new(request_line:Requestline,headers:Vec<RequestHeader>  ,body: RequestBody)->HttpRequest {
        HttpRequest { request_line, headers, body}
    }
}

impl fmt::Display for Requestline{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"{} {} {}",self.method,self.path,self.http_version)
    }
}

/*
This part contains the code for the Response structs and it's implementations
*/
pub struct HttpResponse{
    status_line:Statustline,
    headers:ResponseHeaders,
    body:ResponseBody
}
pub struct Statustline{
    http_version:String,
    status_code:i32,
    message:String
}
pub struct ResponseHeader{
    header:String,
    value:String
    
}

pub struct ResponseBody{
    body:String
}

pub struct ResponseHeaders(Vec<ResponseHeader>);

impl Statustline{
    pub fn new(http_version:String,status_code:i32,message:String)->Statustline{
        Statustline { 
            http_version, 
            status_code, 
            message
         }
    }
    
}
impl ResponseHeader{
    pub fn new(header:String,value:String)->ResponseHeader{
        ResponseHeader {
            header,
            value
          }
    }
}

impl ResponseBody{
    pub fn new(body:String)->ResponseBody{
        ResponseBody { 
            body
         }
    }
}

impl ResponseHeaders{
    pub fn new(headers:Vec<ResponseHeader>)->ResponseHeaders{
        ResponseHeaders(
            headers
        )
    }
}

impl HttpResponse{
    pub fn new(status_line:Statustline,headers:ResponseHeaders,body:ResponseBody)->HttpResponse{
                HttpResponse { 
                    status_line, 
                    headers, 
                    body }
            }
}

impl fmt::Display for Statustline{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{} {} {}\r\n",self.http_version,self.status_code,self.message)
    }
}
impl fmt::Display for ResponseHeader{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}: {}{}",self.header,self.value,"\r\n")
    }
}

impl fmt::Display for ResponseHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for header in &self.0 {
            write!(f, "{}", header)?;
        }
        Ok(())
    }
}
impl fmt::Display for ResponseBody{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.body)
    }
}

impl fmt::Display for HttpResponse{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}{}\r\n{}",self.status_line ,self.headers,self.body)
    }

}