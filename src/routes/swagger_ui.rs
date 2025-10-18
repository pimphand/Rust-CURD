use axum::{response::Html, routing::get, Router};

pub fn swagger_ui_routes() -> Router {
    Router::new()
        .route("/swagger-ui", get(swagger_ui))
}

async fn swagger_ui() -> Html<&'static str> {
    Html(include_str!("../../static/swagger-ui.html"))
}
