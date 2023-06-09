use sqlx::postgres::PgPoolOptions;
use std::fmt::Error;

pub async fn delete_post_database(to_delete: String) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let to_delete = to_delete.parse::<i32>().unwrap();

    sqlx::query("delete from posts where id=$1")
        .bind(to_delete)
        .execute(&pool)
        .await
        .expect("Unable toasdasd");
    Ok(())
}

pub async fn update_post_database(
    title: &String,
    description: &String,
    id: &&i32,
    category_id: &&i32,
) -> Result<(), Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    sqlx::query("update posts set title=$1 ,description=$2, category_id=$3 where id=$4")
        .bind(title)
        .bind(description)
        .bind(category_id)
        .bind(id)
        .execute(&pool)
        .await
        .expect("Unable toasdasd");
    Ok(())
}
