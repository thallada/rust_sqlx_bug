use std::env;

use sqlx::mysql::MySqlPool;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Customer {
    customer_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool: MySqlPool = MySqlPool::builder()
        .max_size(1)
        .build(&env::var("DATABASE_URL")?)
        .await?;

    let customer_names: Vec<Customer> = sqlx::query_as!(
        Customer,
        "SELECT
            customers.name AS customer_name
        FROM
            customers"
    )
    .fetch_all(&pool)
    .await?;

    println!("{:?}", customer_names);

    Ok(())
}
