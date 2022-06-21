use warp::{Filter, http::Method};
mod handlers;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
    .allow_headers(vec!["content-type"])
    .allow_methods(vec!["POST", "GET"])
    .allow_any_origin();

    // POST /spellcheck/work => 200 OK with body collections, 400 Not Found
    let spell_check_post = warp::post().and(warp::path!("spellcheck" / String))
    .and_then(handlers::spell_checker);
    let routes = spell_check_post.with(cors);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 31337))
        .await;
}

