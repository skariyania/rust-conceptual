use std::time::Duration;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Error, FromRow, Postgres,
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db_connection_str = std::env::var("DATABASE_URL").unwrap();

    //connection pool implementation
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("cannot connect to database");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/pg-health", get(fetch_user))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "200"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    // Json(payload): Json<CreateUser>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<User>)> {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: String::from("payload_username"),
    };

    let resulted = sqlx::query("insert into user_detail(username) values('admin')")
        .execute(&pool)
        .await
        .unwrap();
    println!("{:?}", resulted);
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok(Json(user))

    // Ok(
    //     "user created".to_string(),
    //     (StatusCode::CREATED, Json(user)),
    // )
}

impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let id: String = row.row.try_get("id")?;
        let date: String = row.try_get("username")?;

        Ok(User { id, username })
    }
}

async fn fetch_user(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    // Query with macro
    let res: User =
        sqlx::query_scalar("select id, username from user_detail where username ='admin'")
            .fetch_one(&pool)
            .await
            .map_err(internal_error_handler)?;
    // let user_id = res.ok_or("unknown user")?;
    // res

    // let note_responses = res
    //     .iter()
    //     .map(|note| to_note_response(&note))
    //     .collect::<Vec<NoteModelResponse>>();

    // let json_response = serde_json::json!({
    //     "status": "ok",
    //     "count": note_responses.len(),
    //     "notes": note_responses
    // });

    Ok(String::from(res))
}

// async fn create_user_sql(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
//     sqlx::query_scalar("insert into user_detail('username') values('admin')")
//         .fetch_one(&pool)
//         .await
//         .map_err(internal_error_handler)
// }

fn internal_error_handler<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
