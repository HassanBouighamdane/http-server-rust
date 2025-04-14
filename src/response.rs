use std::{fs::{self, OpenOptions}, io::Write};
    

use crate::{http::{http_request::{ HttpRequest, RequestBody}, http_response::{ HttpResponse, ResponseBody, ResponseHeader, ResponseHeaders, Statustline}}, utils::{ extract_compression_schemas, extract_directory_from_env}};

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

pub fn echo_text(http_request:&HttpRequest,text:&str)-> HttpResponse{
    //Response status area
    let status_line=Statustline::new(String::from("HTTP/1.1"), 200, String::from("OK"));
    //header area
    let content_type_header=ResponseHeader::new(String::from("Content-Type"),String::from("text/plain"));
    let content_length_header=ResponseHeader::new(String::from("Content-Length"),String::from(text.len().to_string()));
    let compression_schemas=extract_compression_schemas(&http_request.headers.headers);
    
    let headers=if compression_schemas.contains(&String::from("gzip")){
                let content_encoding_header=ResponseHeader::new(
                    String::from("Content-Encoding"),
                    String::from("gzip")
                );
                ResponseHeaders::new(vec![content_encoding_header,content_type_header,content_length_header])
            }
            else{
                ResponseHeaders::new(vec![content_type_header,content_length_header])
            };
    //body area
    let body=ResponseBody::new(text.to_string());

    let response=HttpResponse::new(status_line,headers,body);
    println!("{}",response);
    response
}

pub fn user_agent(http_request:&HttpRequest)->HttpResponse{
    let status_line=Statustline::new(String::from("HTTP/1.1"), 200, String::from("OK"));
    let mut user_agent_value=String::new();
    for header in &http_request.headers.headers{
        if header.header=="User-Agent"{
            user_agent_value=header.value.clone();
            break;
        }
    }

    let content_type_header=ResponseHeader::new(String::from("Content-Type"),String::from("text/plain"));
    let content_length_header=ResponseHeader::new(String::from("Content-Length"),String::from(user_agent_value.len().to_string()));

    let response_headers=ResponseHeaders::new(vec![content_type_header,content_length_header]);
    //body area
    let body=ResponseBody::new(user_agent_value);
    let response=HttpResponse::new(status_line,response_headers,body);
    
    response
}

pub fn return_file(file_name:&str)-> HttpResponse{
    let directory=extract_directory_from_env();
    
    let file_path=match directory{
        Some(dir)=>{
            let mut path_buf = dir.clone();
            path_buf.push(file_name);
            path_buf
        },
        None=> {
            return self::not_found_response();
        }
    };
    let file=fs::read_to_string(file_path);
                                        
    match file{
        Ok(file_content)=>{
            let status_line=Statustline::new(String::from("HTTP/1.1"), 200, String::from("OK"));
            //header area
            let  content_type_header=ResponseHeader::new(String::from("Content-Type"),String::from("application/octet-stream"));
            let  content_length_header=ResponseHeader::new(String::from("Content-Length"),String::from(file_content.len().to_string()));
        
            let  headers=ResponseHeaders::new(vec![content_type_header,content_length_header]);
            //body area
            let  body=ResponseBody::new(String::from(file_content));
            let response=HttpResponse::new(status_line,headers,body);
            response
        },
        Err(_e)=>{
            return self::not_found_response()
        }
    }
}

pub fn create_file(body:RequestBody,file_name:&str)->HttpResponse{
    let directory=extract_directory_from_env();
    
    let file_path=match directory{
        Some(dir)=>{
            let mut path_buf=dir;
            path_buf.push(file_name);
            path_buf
        },
        None=>{
            return self::not_found_response();
        }
    };

    let file=OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(&file_path);
    match file{
        Ok(mut f)=>{
            let file_content=body.body;
            if let Err(e)= f.write_all(file_content.as_bytes()){
                eprintln!("Failed to write to file {}: {}", file_path.display(), e);
                return self::internal_server_error_response();
            }
        },
        Err(e)=>{     
            eprintln!("Failed to open file {}: {}", file_path.display(), e);
            return self::internal_server_error_response();
        }
    }
    let status_line=Statustline::new(String::from("HTTP/1.1"), 201, String::from("Created"));
    //header area
    //let  content_type_header=ResponseHeader::new(String::from("Content-Type"),String::from("application/octet-stream"));
    //let  content_length_header=ResponseHeader::new(String::from("Content-Length"),String::from(body.body().len().to_string()));
    let  headers=ResponseHeaders::new(vec![]);
    //body area
    let  body=ResponseBody::new(String::new());
    let response=HttpResponse::new(status_line,headers,body);

    response
}

fn internal_server_error_response()-> HttpResponse{
    let status_line=Statustline::new(String::from("HTTP/1.1"), 500, String::from("Internal Server Error"));
    let headers=ResponseHeaders::new(vec![]);
    let body=ResponseBody::new(String::new());

    HttpResponse::new(status_line, headers, body)
}