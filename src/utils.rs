use std::{env, io::{BufRead, BufReader, Read,Result}, net::TcpStream, path::PathBuf};
use flate2::{read::GzEncoder, Compression};
use crate::http::http_request::{HttpRequest, RequestBody, RequestHeader, RequestHeaders, Requestline};

pub fn extract_directory_from_env()->Option<PathBuf>{
    let args:Vec<String>=env::args().collect();

    let mut directory=None;

    for i in 0..args.len()-1{
        if args[i]=="--directory"{
            directory=Some(PathBuf::from(&args[i+1]));
            break;
    }
}
   directory
}

pub fn parse_http_request(buf_reader:&mut BufReader<&TcpStream>)-> HttpRequest{
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


pub fn extract_compression_schemas(request_headers_vec:&Vec<RequestHeader>)->Vec<String>{

    let compression_schema=request_headers_vec.iter()
   .find(|h| h.header.to_lowercase()=="accept-encoding")
   .map(|h|&h.value);

   let compression_schemas=match compression_schema{
    Some(schemas)=>{
        schemas.split(",")
                .map(|s| s.trim().to_string())
                .collect()
    }
    None=>{
        Vec::new()
    }
   };
   compression_schemas
}

pub fn compress_to_gzip(text:&str)->Result<Vec<u8>>{
    let mut ret_vec = Vec::new();
    let bytestring =text.as_bytes();
    let mut gz = GzEncoder::new(&bytestring[..], Compression::fast());
    gz.read_to_end(&mut ret_vec)?;
    Ok(ret_vec)
}