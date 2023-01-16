use hyper::body::Bytes;
use http_body_util::Full;
use hyper::{Response, StatusCode, header};


pub fn get_internal_server_error_response<E: std::fmt::Debug>(err: E) -> Response<Full<Bytes>> {
    println!("{:?}", err);
    return Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Full::new(Bytes::from(""))).unwrap()
}

pub fn get_no_content_response() -> Response<Full<Bytes>> {
    return Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Full::new(Bytes::from(""))).unwrap()
}

pub fn get_ok_response(data: String) -> Response<Full<Bytes>> {
    return Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from(data))).unwrap();     
}

pub fn get_bad_request() -> Response<Full<Bytes>> {
    return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(Bytes::from(""))).unwrap(); 
}

pub fn get_not_found() -> Response<Full<Bytes>> {
    return Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(Bytes::from(""))).unwrap(); 
}
