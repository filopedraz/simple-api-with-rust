use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{body::json, Filter};

#[derive(Deserialize)]
struct SumRequest {
    a: i32,
    b: i32,
}

#[derive(Serialize)]
struct SumResponse {
    sum: i32,
}

#[tokio::main]
async fn main() {
    println!("Server started at http://localhost:3030");
    let sum = warp::post()
        .and(warp::path("sum"))
        .and(json())
        .and_then(handle_sum);

    warp::serve(sum).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_sum(body: SumRequest) -> Result<impl warp::Reply, Infallible> {
    let result = SumResponse {
        sum: body.a + body.b,
    };

    Ok(warp::reply::json(&result))
}
