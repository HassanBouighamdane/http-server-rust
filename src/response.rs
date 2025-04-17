use std::{fs::{self, OpenOptions}, io::Write};
    

use crate::{http::{http_request:: HttpRequest, http_response::{ HttpResponse, ResponseBody, ResponseHeader, ResponseHeaders, Statustline}}, utils::{ compress_to_gzip, extract_compression_schemas, extract_content_type, extract_directory_from_env}};

pub fn success_response(_http_request:&HttpRequest,close_connection:bool)->HttpResponse{
    let status_line=Statustline::new(String::from("HTTP/1.1"), 200, String::from("OK"));
   
    let mut headers_vec=Vec::new();
    if close_connection{
        headers_vec.push(ResponseHeader::new(String::from("Connection"), String::from("close")));
    }
    let headers=ResponseHeaders::new(headers_vec);
    let body=ResponseBody::new(String::from(""));
    let response=HttpResponse::new(status_line,headers,body);
    
    response
}

pub fn not_found_response(_http_request:&HttpRequest,close_connection:bool)->HttpResponse{
    let status_line=Statustline::new(String::from("HTTP/1.1"), 404, String::from("Not Found"));
    let mut headers_vec=Vec::new();
    if close_connection{
        headers_vec.push(ResponseHeader::new(String::from("Connection"), String::from("close")));
    }
    let headers=ResponseHeaders::new(headers_vec);
    let body=ResponseBody::new(String::from(""));
    let response=HttpResponse::new(status_line,headers,body);
    
    response
}

pub fn echo_text(http_request:&HttpRequest,text:&str,close_connection:bool)-> HttpResponse{
    //Response status area
    let status_line=Statustline::new(String::from("HTTP/1.1"), 200, String::from("OK"));
    //header area
    let mut headers_vec=Vec::new();
    let content_type_header=match extract_content_type(&http_request.headers){
        Some(content_type)=>{
            ResponseHeader::new(String::from("Content-Type"),content_type.clone())
        },
        None=>{
            ResponseHeader::new(String::from("Content-Type"),String::from("text/plain"))
        }
    };
    headers_vec.push(content_type_header);
    if close_connection{
        headers_vec.push(ResponseHeader::new(String::from("Connection"), String::from("close")));
    }
    let compression_schemas=extract_compression_schemas(&http_request.headers);
    
    let (headers,body)=if compression_schemas.contains(&String::from("gzip")){
                let content_encoding_header=ResponseHeader::new(
                    String::from("Content-Encoding"),
                    String::from("gzip")
                );
                headers_vec.push(content_encoding_header);
                let compressed_body=compress_to_gzip(text);
                let body_response=match compressed_body{
                    Ok(b)=>{

                        let content_length_header = ResponseHeader::new(
                            String::from("Content-Length"),
                            b.len().to_string()
                        );
                        headers_vec.push(content_length_header);
                        let headers=ResponseHeaders::new(headers_vec);
                        let binary_string = unsafe { String::from_utf8_unchecked(b) };
                        (headers, ResponseBody::new(binary_string))
                    }Err(_e)=>{
                        headers_vec.push( ResponseHeader::new(
                            String::from("Content-Length"),
                            "0".to_string()
                        ));
                        let headers=ResponseHeaders::new(headers_vec);
                        (headers, ResponseBody::new(String::new()))
                    }
                };
                body_response
            }
            else{
                headers_vec.push(ResponseHeader::new(
                    String::from("Content-Length"),
                    text.len().to_string()
                ));
                let headers=ResponseHeaders::new(headers_vec);
                (headers, ResponseBody::new(text.to_string()))
            };

    let response=HttpResponse::new(status_line,headers,body);
    response
}

pub fn user_agent(http_request:&HttpRequest,close_connection:bool)->HttpResponse{
    let status_line=Statustline::new(String::from("HTTP/1.1"), 200, String::from("OK"));
    let mut user_agent_value=String::new();
    let mut headers_vec =Vec::new();
    for header in &http_request.headers.headers{
        if header.header=="User-Agent"{
            user_agent_value=header.value.clone();
            break;
        }
    }
    
    if close_connection{
        headers_vec.push(ResponseHeader::new(String::from("Connection"), String::from("close")));
    }
    headers_vec.push(ResponseHeader::new(String::from("Content-Type"),String::from("text/plain")));
    headers_vec.push(ResponseHeader::new(String::from("Content-Length"),String::from(user_agent_value.len().to_string())));
    let headers=ResponseHeaders::new(headers_vec);

    //body area
    let body=ResponseBody::new(user_agent_value);
    let response=HttpResponse::new(status_line,headers,body);
    
    response
}

pub fn return_file(http_request:&HttpRequest,file_name:&str,close_connection:bool)-> HttpResponse{
    let directory=extract_directory_from_env();
    let mut headers_vec=Vec::new();
    if close_connection{
        headers_vec.push(ResponseHeader::new(String::from("Connection"), String::from("close")));
    }
    let file_path=match directory{
        Some(dir)=>{
            let mut path_buf = dir.clone();
            path_buf.push(file_name);
            path_buf
        },
        None=> {
            return self::not_found_response(http_request,close_connection);
        }
    };
    let file=fs::read_to_string(file_path);
                                        
    match file{
        Ok(file_content)=>{
            let status_line=Statustline::new(String::from("HTTP/1.1"), 200, String::from("OK"));
            //header area
            let  content_type_header=ResponseHeader::new(String::from("Content-Type"),String::from("application/octet-stream"));
            let  content_length_header=ResponseHeader::new(String::from("Content-Length"),String::from(file_content.len().to_string()));
            headers_vec.push(content_length_header);
            headers_vec.push(content_type_header);
            let  headers=ResponseHeaders::new(headers_vec);
            //body area
            let  body=ResponseBody::new(String::from(file_content));
            let response=HttpResponse::new(status_line,headers,body);
            response
        },
        Err(_e)=>{
            return self::not_found_response(http_request,close_connection)
        }
    }
}

pub fn create_file(http_request:&HttpRequest,file_name:&str,close_connection:bool)->HttpResponse{
    let directory=extract_directory_from_env();
    let mut headers_vec=Vec::new();
    if close_connection{
        headers_vec.push(ResponseHeader::new(String::from("Connection"), String::from("close")));
    }
    let file_path=match directory{
        Some(dir)=>{
            let mut path_buf=dir;
            path_buf.push(file_name);
            path_buf
        },
        None=>{
            return self::not_found_response(http_request,close_connection);
        }
    };

    let file=OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(&file_path);
    match file{
        Ok(mut f)=>{
            let file_content=&http_request.body.body;
            if let Err(e)= f.write_all(file_content.as_bytes()){
                eprintln!("Failed to write to file {}: {}", file_path.display(), e);
                return self::internal_server_error_response(close_connection);
            }
        },
        Err(e)=>{     
            eprintln!("Failed to open file {}: {}", file_path.display(), e);
            return self::internal_server_error_response(close_connection);
        }
    }
    let status_line=Statustline::new(String::from("HTTP/1.1"), 201, String::from("Created"));
    //header area
    //let  content_type_header=ResponseHeader::new(String::from("Content-Type"),String::from("application/octet-stream"));
    //let  content_length_header=ResponseHeader::new(String::from("Content-Length"),String::from(body.body().len().to_string()));
    let  headers=ResponseHeaders::new(headers_vec);
    //body area
    let  body=ResponseBody::new(String::new());
    let response=HttpResponse::new(status_line,headers,body);

    response
}

fn internal_server_error_response(close_connection:bool)-> HttpResponse{
    let status_line=Statustline::new(String::from("HTTP/1.1"), 500, String::from("Internal Server Error"));
    let mut headers_vec=Vec::new();
    if close_connection{
        headers_vec.push(ResponseHeader::new(String::from("Connection"), String::from("close")));
    }
    let headers=ResponseHeaders::new(headers_vec);
    let body=ResponseBody::new(String::new());

    HttpResponse::new(status_line, headers, body)
}