use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryAs;
use sqlx::postgres::{PgPool, Postgres};
use sqlx::types::Json;
use sqlx::FromRow;
use std::env;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Customer {
    json_column: Json<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool: PgPool = PgPool::builder()
        .max_size(1)
        .build(&env::var("DATABASE_URL")? as &str)
        .await?;

    // Works fine
    let customer_names: Vec<Customer> = sqlx::query_as::<Postgres, Customer>(
        "SELECT
            json_column
        FROM
            test",
    )
    .fetch_all(&pool)
    .await?;

    println!("{:?}", customer_names);

    // Comile error: the trait
    // `sqlx::result_ext::ResultExt<sqlx_core::types::json::Json<std::vec::Vec<std::string::String>>>`
    // is not implemented for `std::result::Result<std::option::Option<serde_json::value::Value>, sqlx_core::error::Error>`
    let customer_names: Vec<Customer> = sqlx::query_as!(
        Customer,
        "SELECT
            json_column
        FROM
            test"
    )
    .fetch_all(&pool)
    .await?;

    println!("{:?}", customer_names);
    Ok(())
}
