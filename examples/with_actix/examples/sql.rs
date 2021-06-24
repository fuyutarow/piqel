use once_cell::sync::Lazy;
use sqlx::postgres::PgRow;
use sqlx::prelude::Row;

struct Config {
    postgres_host: String,
    postgres_port: String,
    postgres_user: String,
    postgres_password: String,
    postgres_database: String,
}

impl Config {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_database
        )
    }
}

static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    postgres_host: std::env::var("POSTGRES_HOST").unwrap(),
    postgres_port: std::env::var("POSTGRES_PORT").unwrap(),
    postgres_user: std::env::var("POSTGRES_USER").unwrap(),
    postgres_password: std::env::var("POSTGRES_PASSWORD").unwrap(),
    postgres_database: std::env::var("POSTGRES_DB").unwrap(),
});

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
struct User {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
struct NewUser {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, sqlx::Type)]
#[repr(i32)]
enum PostVisibility {
    Public = 1,
    Private = 2,
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
struct Post {
    pub id: i64,
    pub visibility: PostVisibility,
    pub user_id: i64,
    pub title: String,
    pub body: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
struct NewPost {
    pub user_id: i64,
    pub visibility: PostVisibility,
    pub title: String,
    pub body: Option<String>,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let database_url = "postgres://user:password@localhost:8132/app";
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        // .connect(&CONFIG.database_url())
        .connect(&database_url)
        .await?;

    let users = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", users.len());
    println!("{:?}", users);

    let mut transaction = pool.begin().await?;

    let new_user = NewUser {
        name: "testtest".to_owned(),
    };
    let user = sqlx::query_as::<_, User>(
        r#"
insert into users (name)
values ($1)
returning *
"#,
    )
    .bind(&new_user.name)
    .fetch_one(&mut transaction)
    .await?;

    let new_post = NewPost {
        user_id: user.id,
        visibility: PostVisibility::Public,
        title: format!("Title-{}", user.id),
        body: None,
    };

    let post = sqlx::query_as::<_, Post>(
        r#"
insert into posts (user_id, visibility, title, body)
values ($1, $2, $3, $4)
returning *
"#,
    )
    .bind(&new_post.user_id)
    .bind(&new_post.visibility)
    .bind(&new_post.title)
    .bind(&new_post.body)
    .fetch_one(&mut transaction)
    .await?;

    println!("{:?}", user);
    println!("{:?}", post);

    let users = sqlx::query(
        r#"
select
  users.id, users.name, users.created_at, users.updated_at,
  posts.id, posts.user_id, posts.title, posts.body, posts.visibility, posts.created_at, posts.updated_at
from users
inner join posts on users.id = posts.user_id
"#,
    )
        .map(|row: PgRow| {
            (
                User {
                    id: row.get(0),
                    name: row.get(1),
                    created_at: row.get(2),
                    updated_at: row.get(3),
                },
                Post {
                    id: row.get(4),
                    user_id: row.get(5),
                    title: row.get(6),
                    body: row.get(7),
                    visibility: row.get(8),
                    created_at: row.get(9),
                    updated_at: row.get(10),
                },
            )
        })
    .fetch_all(&mut transaction)
    .await?;

    println!("{:#?}", users);

    Ok(())
}
