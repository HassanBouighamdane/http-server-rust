use crate::http::{ HttpResponse, ResponseBody, ResponseHeader, ResponseHeaders, Statustline};

pub fn success_response()->HttpResponse{
    let status_line=Statustline::new(String::from("HTTP/1.1"), 200, String::from("OK"));
    let headers=ResponseHeaders::new(vec![]);
    let body=ResponseBody::new(String::from(""));
    let response=HttpResponse::new(status_line,headers,body);
    
    response
}

pub fn not_found_response()->HttpResponse{
    let status_line=Statustline::new(String::from("HTTP/1.1"), 404, String::from("Not Found"));
    let headers=ResponseHeaders::new(vec![]);
    let body=ResponseBody::new(String::from(""));
    let response=HttpResponse::new(status_line,headers,body);
    
    response
}

pub fn echo_text(text:&str)-> HttpResponse{
    //Response status area
    let status_line=Statustline::new(String::from("HTTP/1.1"), 200, String::from("OK"));
    //header area
    let content_type_header=ResponseHeader::new(String::from("Content-Type"),String::from("text/plain"));
    let content_length_header=ResponseHeader::new(String::from("Content-Length"),String::from(text.len().to_string()));

    let headers=ResponseHeaders::new(vec![content_type_header,content_length_header]);
    //body area
    let body=ResponseBody::new(text.to_string());
    let response=HttpResponse::new(status_line,headers,body);
 
    response
}