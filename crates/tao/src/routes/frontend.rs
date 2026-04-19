use axum::{
    http,
    response::{self, IntoResponse},
};
use rust_embed::Embed;

static INDEX_HTML: &str = "index.html";

#[derive(Embed)]
#[folder = "../../frontend/build"]
struct FrontendAssets;

#[axum::debug_handler]
pub async fn static_handler(uri: http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match FrontendAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(http::header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn index_html() -> response::Response {
    match FrontendAssets::get(INDEX_HTML) {
        Some(content) => response::Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> response::Response {
    (http::StatusCode::NOT_FOUND, "404").into_response()
}
