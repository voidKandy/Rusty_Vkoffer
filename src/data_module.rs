#![allow(unused)]
use anyhow::Result;
use dotenv::dotenv;
use sqlx::Connection;
use sqlx::FromRow;
use sqlx::Row;
use std::env;
use std::error::Error;

pub mod sql_module {
    use super::*;

    #[derive(Debug, FromRow)]
    pub struct Password {
        pub id: String,
        pub username: String,
        pub password: String,
        pub service: String,
    }

    pub async fn create_pool() -> Result<sqlx::PgPool> {
        dotenv().ok();
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = sqlx::postgres::PgPool::connect(&url).await?;
        Ok(pool)
    }

    pub async fn read(pool: &sqlx::PgPool) -> Result<Vec<Password>> {
        let q = "SELECT id, username, password, service FROM password";
        let query = sqlx::query_as::<_, Password>(q);

        Ok(query.fetch_all(pool).await?)
    }

    pub async fn read_by_key(key: &str, val: &str, pool: &sqlx::PgPool) -> Result<Vec<Password>> {
        let q = format!(
            "SELECT id, username, password, service FROM password WHERE {} = $1",
            key
        );
        let query = sqlx::query_as::<_, Password>(&q).bind(val);
        Ok(query.fetch_all(pool).await?)
    }

    pub async fn drop_by_key(key: &str, val: &str, pool: &sqlx::PgPool) -> Result<()> {
        let query = format!("DELETE FROM password WHERE {} = $1", key);
        let res = sqlx::query(&query).bind(val).execute(pool).await;
        match res {
            Ok(qres) => println!("Deleted password(s) where {} = {}", key, val),
            Err(error) => println!("{error}"),
        }
        Ok(())
    }

    pub async fn insert(password: &Password, pool: &sqlx::PgPool) -> Result<()> {
        let query =
            "INSERT INTO password (id, username, password, service) VALUES ($1, $2, $3, $4)";

        sqlx::query(query)
            .bind(&password.id)
            .bind(&password.username)
            .bind(&password.password)
            .bind(&password.service)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update(password: &Password, pool: &sqlx::PgPool) -> Result<()> {
        let query = "UPDATE password SET username = $2, password = $3, service = $4 WHERE id = $1";

        sqlx::query(query)
            .bind(&password.id)
            .bind(&password.username)
            .bind(&password.password)
            .bind(&password.service)
            .execute(pool)
            .await?;

        Ok(())
    }
}
