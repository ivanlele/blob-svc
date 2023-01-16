use crate::data::{Blob};
use sqlx::postgres::PgPool;
use sqlx::postgres::{PgQueryResult};
use sqlx::Error;

pub async fn create(data: &serde_json::Value, pool: &PgPool) -> Result<PgQueryResult, Error> {
    let value = serde_json::to_string(&data).unwrap();

    let stmt = format!(r#"
INSERT INTO blobs (data) VALUES
('{}')
    "#, value);

    sqlx::query(&stmt).execute(pool).await
}


pub async fn update(blob: &Blob, pool: &PgPool) -> Result<PgQueryResult, Error> {
    let value = serde_json::to_string(&blob.data).unwrap();

    let stmt = format!(r#"
UPDATE blobs SET data = '{}' WHERE id = {}
    "#, value, blob.id);

    sqlx::query(&stmt).execute(pool).await
}

pub async fn delete(id: &i64, pool: &PgPool) -> Result<PgQueryResult, Error> {
    let stmt = format!(r#"
DELETE FROM blobs WHERE id = {}
    "#, id);

    sqlx::query(&stmt).execute(pool).await
}

pub async fn get_list(pool: &PgPool) -> Result<Vec<Blob>, Error> {
    sqlx::query_as!(
        Blob,
        r#"
SELECT id, data as "data: serde_json::Value"
FROM blobs
ORDER BY id
        "#
    ).fetch_all(pool).await
}

pub async fn get(id: &i64, pool: &PgPool) -> Result<Blob, Error> {
    sqlx::query_as!(
        Blob,
        r#"
SELECT id, data as "data: serde_json::Value"
FROM blobs
Where id = $1
        "#, id as _
    ).fetch_one(pool).await
}