use std::env;
use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPool, MySql};
use sqlx::FromRow;
use sqlx::types::Json;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Customer {
    json_column: Json<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
                let pool: MySqlPool = MySqlPool::builder()
                    .max_size(1)
                    .build(&env::var("DATABASE_URL")? as &str)
                    .await?;

                let customer_names: Vec<Customer> = sqlx::query_as::<MySql, Customer>(
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
