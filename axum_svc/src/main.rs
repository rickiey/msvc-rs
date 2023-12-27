use axum::{
    routing::get,
    Router,
};

use std::env;
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "{\"Hello\": \"World!\"}" }));

    // run it with hyper on localhost:3000
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    let ss = env::var("OUT_PATH");
    match ss {
        Ok(s) => {
        println!("{:?}", s);
        }
        Err(e) => {
            println!("{:?}", e.to_string());
        }
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}