use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct Categories {
    pub(crate) id: i32,
    pub(crate) name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct Posts {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) category_id: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize, sqlx::FromRow)]
pub struct UpdatePost {
    pub(crate) current_title: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) name: String,
}

pub async fn select_posts() -> Result<Vec<Posts>, Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let postsing =
        sqlx::query_as::<_, Posts>("select id, title, description, category_id from posts")
            .fetch_all(&pool)
            .await
            .unwrap();

    Ok(postsing)
}
