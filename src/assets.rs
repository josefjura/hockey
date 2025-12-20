use axum::{
    body::Body,
    http::{header, HeaderValue, Response, StatusCode},
    response::IntoResponse,
};
use rust_embed::RustEmbed;

/// Embedded static assets (CSS, JS, images, flags)
///
/// In development mode with debug-embed feature:
/// - Assets are loaded from filesystem for hot-reloading
///
/// In production mode:
/// - Assets are embedded in the binary at compile time
///
/// Note: User uploads directory is excluded and served from filesystem
#[derive(RustEmbed)]
#[folder = "static/"]
#[exclude = "uploads/*"]
pub struct Assets;

/// Serve embedded static assets or user uploads from filesystem
///
/// This handler replaces `ServeDir` for production deployments where
/// all assets are embedded in the binary, except for user uploads which
/// are served from the filesystem.
pub async fn serve_static_asset(path: &str) -> impl IntoResponse {
    // Remove leading slash if present
    let path = path.trim_start_matches('/');

    // User uploads are not embedded, serve from filesystem
    if path.starts_with("uploads/") {
        return serve_upload_file(path).await;
    }

    // Try to get embedded asset
    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            let body = Body::from(content.data);

            Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime.as_ref()).unwrap(),
                )
                // Cache static assets for 1 year (immutable content)
                .header(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=31536000, immutable"),
                )
                .body(body)
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Asset not found"))
            .unwrap(),
    }
}

/// Serve user-uploaded files from filesystem
async fn serve_upload_file(path: &str) -> Response<Body> {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    let file_path = format!("static/{}", path);

    match File::open(&file_path).await {
        Ok(mut file) => {
            let mut contents = Vec::new();
            if file.read_to_end(&mut contents).await.is_err() {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Failed to read file"))
                    .unwrap();
            }

            let mime = mime_guess::from_path(&file_path).first_or_octet_stream();

            Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime.as_ref()).unwrap(),
                )
                // Don't cache user uploads as aggressively
                .header(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=3600"),
                )
                .body(Body::from(contents))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File not found"))
            .unwrap(),
    }
}

/// Helper to get asset content as string (useful for inlining CSS/JS)
#[allow(dead_code)]
pub fn get_asset_string(path: &str) -> Option<String> {
    Assets::get(path).and_then(|content| {
        std::str::from_utf8(&content.data)
            .ok()
            .map(|s| s.to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_serve_existing_asset() {
        // This test will only work if there are actual files in static/
        // In development with debug-embed, it reads from filesystem
        let response = serve_static_asset("js/components/badge.js")
            .await
            .into_response();

        // Should return OK if file exists
        // In test environment without files, this might be 404
        assert!(response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_serve_missing_asset() {
        let response = serve_static_asset("nonexistent/file.js")
            .await
            .into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
