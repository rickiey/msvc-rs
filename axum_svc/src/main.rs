

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use axum::extract::State;

use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{FromRow};
// use sqlx::types::chrono::{DateTime, Utc};

// use std::env;
#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();


    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:123456@localhost".to_string());

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/getuser", get(selectdb))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    // app.with_state(state)
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async  fn selectdb(State(pool): State<PgPool>) -> (StatusCode, Json<User>) {

    let user = sfromdb(State(pool));

    (StatusCode::CREATED, Json(user.await))

}

async  fn sfromdb(State(pool): State<PgPool>) -> User {


    let  row:Vec<User>= sqlx::query_as::<_,User>("SELECT id, name from test")
        .fetch_all(&pool).await.unwrap();

    println!("row 1 : {:?}", row);
    // (user, StatusCode::OK, "no_err".to_string())

    let user = User {
        id: 0,
        name: "test-name".to_string(),
    };
    user
}

fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        name: payload.name,
    };
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

// the output to our `create_user` handler
#[derive(Serialize,FromRow, Debug, PartialEq, Eq)]
struct User {
    id: i64,
    name: String,
}

/*
postgres@127:postgres> \d+ test
+------------+--------------------------+----------------------------------------------------+----------+--------------+-------------+
| Column     | Type                     | Modifiers                                          | Storage  | Stats target | Description |
|------------+--------------------------+----------------------------------------------------+----------+--------------+-------------|
| id         | bigint                   |  not null default nextval('test_id_seq'::regclass) | plain    | <null>       | <null>      |
| name       | character varying(250)   |                                                    | extended | <null>       | <null>      |
| created_at | timestamp with time zone |                                                    | plain    | <null>       | <null>      |
+------------+--------------------------+----------------------------------------------------+----------+--------------+-------------+
 */

//
// #[tokio::main]
// async fn main() {
//     // build our application with a single route
//     let app = Router::new().route("/", get(|| async { "{\"Hello\": \"World!\"}" }));
//
//     // run it with hyper on localhost:3000
//     // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//     //     .serve(app.into_make_service())
//     //     .await
//     //     .unwrap();
//
//     let ss = env::var("OUT_PATH");
//     match ss {
//         Ok(s) => {
//         println!("{:?}", s);
//         }
//         Err(e) => {
//             println!("{:?}", e.to_string());
//         }
//     }
//
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }