
pub fn success_response()->String{
    let status_line="HTTP/1.1 200 OK".to_string();
    let crlt="\r\n".to_string();
    let headers="".to_string();
    let content="".to_string();
    let response=status_line+
        &crlt+
        &headers+
        &crlt+
        &content;
    response
}

pub fn not_found_response()->String{
    let status_line="HTTP/1.1 404 NOT FOUND".to_string();
    let crlt="\r\n".to_string();
    let headers="".to_string();
    let content="".to_string();
    let response=status_line+
        &crlt+
        &headers+
        &crlt+
        &content;
    response
}