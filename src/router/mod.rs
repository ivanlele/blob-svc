use crate::handlers::*;
use crate::handlers::response::{get_not_found, get_bad_request};

use bytes::{Bytes};
use http_body_util::{Full};
use hyper::{Request, Response, Error, Method};
use sqlx::postgres::PgPool;

pub async fn route(r: Request<hyper::body::Incoming>, pool: PgPool) -> Result<Response<Full<Bytes>>, Error> {    
    let paths = r.uri().path().split("/").collect::<Vec<&str>>();
    let paths_len = paths.len();

    if paths_len > 3 || paths_len < 2{
        return Ok(get_not_found());
    }

    let mut id: i64 = 0;
    if paths_len == 3 {
        match paths[2].parse::<i64>() {
            Ok(ok) => {
                id = ok;
            },
            Err(_) => {
                return Ok(get_bad_request());
            }
        }
    }

    match (r.method(), paths[1], id) {
        (&Method::POST, "blobs", 0) => create_blob(r, &pool).await,
        (&Method::GET, "blobs", num) if num != 0 => get_blob(r, &pool, &id).await,
        (&Method::GET, "blobs", 0) => get_blobs(r, &pool).await,
        (&Method::PUT, "blobs", 0) => update_blob(r, &pool).await,
        (&Method::DELETE, "blobs", num) if num != 0 => delete_blob(r, &pool, &id).await,
        _ => {
            Ok(get_not_found())
        }
    }
}
