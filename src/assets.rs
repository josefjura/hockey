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

            let content_type = HeaderValue::from_str(mime.as_ref())
                .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                // Cache static assets for 1 year (immutable content)
                .header(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=31536000, immutable"),
                )
                .body(body)
                .unwrap_or_else(|_| {
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Failed to build response"))
                        .expect("Fallback response should always build")
                })
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Asset not found"))
            .unwrap_or_else(|_| {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Failed to build response"))
                    .expect("Fallback response should always build")
            }),
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
                    .unwrap_or_else(|_| {
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from("Failed to build response"))
                            .expect("Fallback response should always build")
                    });
            }

            let mime = mime_guess::from_path(&file_path).first_or_octet_stream();

            let content_type = HeaderValue::from_str(mime.as_ref())
                .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                // Don't cache user uploads as aggressively
                .header(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=3600"),
                )
                .body(Body::from(contents))
                .unwrap_or_else(|_| {
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Failed to build response"))
                        .expect("Fallback response should always build")
                })
        }
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File not found"))
            .unwrap_or_else(|_| {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Failed to build response"))
                    .expect("Fallback response should always build")
            }),
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
        // Test with CSS file that exists in the repository
        let response = serve_static_asset("css/theme.css").await.into_response();

        // Verify file is served successfully
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "Expected OK status for existing asset css/theme.css"
        );

        // Verify Content-Type header is set correctly for CSS
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .expect("Content-Type header should be present");
        assert!(
            content_type.to_str().unwrap().contains("text/css"),
            "Content-Type should be text/css for CSS files"
        );

        // Verify Cache-Control header is set for browser caching
        let cache_control = response
            .headers()
            .get(header::CACHE_CONTROL)
            .expect("Cache-Control header should be present");
        assert!(
            cache_control.to_str().unwrap().contains("max-age"),
            "Cache-Control should include max-age directive"
        );
    }

    #[tokio::test]
    async fn test_serve_missing_asset() {
        let response = serve_static_asset("nonexistent/file.js")
            .await
            .into_response();
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "Expected NOT_FOUND status for missing asset"
        );
    }

    #[tokio::test]
    async fn test_mime_type_detection() {
        // Test that different file types get correct MIME types
        let test_cases = vec![
            ("css/theme.css", "text/css"),
            ("css/components.css", "text/css"),
        ];

        for (path, expected_mime) in test_cases {
            let response = serve_static_asset(path).await.into_response();

            // Only test MIME type if file exists
            if response.status() == StatusCode::OK {
                let content_type = response
                    .headers()
                    .get(header::CONTENT_TYPE)
                    .unwrap_or_else(|| panic!("Content-Type header missing for {}", path));
                assert!(
                    content_type.to_str().unwrap().contains(expected_mime),
                    "Expected {} for {}, got {:?}",
                    expected_mime,
                    path,
                    content_type
                );
            }
        }
    }
}
