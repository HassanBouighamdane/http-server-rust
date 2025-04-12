
use std::fmt::{self};
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
    pub fn new(http_version:String,status_code:i32,message:String)->Self{
        Self { 
            http_version, 
            status_code, 
            message
         }
    }
    
}
impl ResponseHeader{
    pub fn new(header:String,value:String)->Self{
        Self {
            header,
            value
          }
    }
}

impl ResponseBody{
    pub fn new(body:String)->Self{
        Self { 
            body
         }
    }
}

impl ResponseHeaders{
    pub fn new(headers:Vec<ResponseHeader>)->Self{
        Self(
            headers
        )
    }
}

impl HttpResponse{
    pub fn new(status_line:Statustline,headers:ResponseHeaders,body:ResponseBody)->Self{
                Self { 
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