use axum::response::{Html, Response};
use axum::routing::get;
use axum::Router;
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};

#[event(fetch, http)]
pub async fn main(
    request: HttpRequest, _env: Env, _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();

    let mut router = Router::new().route("/", get(index)).into_service();
    let response = router.call(request).await.unwrap();
    Ok(response)
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
