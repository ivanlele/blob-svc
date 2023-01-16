use sqlx::postgres::PgPool;

pub async fn migrate(pool: &PgPool) {
    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS blobs (
    id BIGSERIAL PRIMARY KEY,
    data JSONB NOT NULL
);
        "#,
    ).execute(pool).await.unwrap();
    println!("Assets migrated");
}