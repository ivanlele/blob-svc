pub mod response;

use crate::data::pg::*;
use crate::data::Blob;
use crate::handlers::response::*;

use bytes::Buf;
use hyper::{Request, Response, Error};
use hyper::body::Bytes;
use http_body_util::{Full, BodyExt};
use sqlx::postgres::PgPool;
use sqlx::Error::RowNotFound;

pub async fn create_blob(req: Request<hyper::body::Incoming>, pool: &PgPool) -> Result<Response<Full<Bytes>>, Error> {
    let body = req.collect().await?.aggregate();
    let data: serde_json::Value = serde_json::from_reader(body.reader()).unwrap();
    
    match create(&data, pool).await {
        Err(e) => return Ok(get_internal_server_error_response(e)),
        Ok(_) => return Ok(get_no_content_response())
    }    
}

pub async fn get_blobs(_: Request<hyper::body::Incoming>, pool: &PgPool) -> Result<Response<Full<Bytes>>, Error> {
    match get_list(pool).await {
        Err(e) => return Ok(get_internal_server_error_response(e)),
        Ok(ok) => {
            let result = serde_json::to_string(&ok).unwrap();
            return Ok(get_ok_response(result));
        }
    }  
}

pub async fn get_blob(_: Request<hyper::body::Incoming>, pool: &PgPool, id: &i64) -> Result<Response<Full<Bytes>>, Error> {
    match get(id, pool).await {
        Err(e) => {
            match e {
                RowNotFound => Ok(get_bad_request()),
                _ => Ok(get_internal_server_error_response(e))
            }
        }
        Ok(ok) => {
            let result = serde_json::to_string(&ok).unwrap();
            return Ok(get_ok_response(result));
        }
    } 
}

pub async fn update_blob(req: Request<hyper::body::Incoming>, pool: &PgPool) -> Result<Response<Full<Bytes>>, Error> {
    let body = req.collect().await?.aggregate();

    let blob: Blob = serde_json::from_reader(body.reader()).unwrap();

    match update(&blob, pool).await {
        Err(e) => return Ok(get_internal_server_error_response(e)),
        Ok(_) => return Ok(get_no_content_response())
    } 
}

pub async fn delete_blob(_: Request<hyper::body::Incoming>, pool: &PgPool, id: &i64) -> Result<Response<Full<Bytes>>, Error> {
    match delete(&id, pool).await {
        Err(e) => return Ok(get_internal_server_error_response(e)),
        Ok(_) => return Ok(get_no_content_response())
    } 
}
