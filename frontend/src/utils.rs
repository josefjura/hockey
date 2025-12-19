use serde::{Deserialize, Deserializer};
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

/// Deserialize an optional string, treating empty strings as None
pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => Ok(Some(s.to_string())),
    }
}

/// Deserialize an optional i64, treating empty strings as None
pub fn empty_string_as_none_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => s.parse().map(Some).map_err(serde::de::Error::custom),
    }
}

/// Deserialize an optional i32, treating empty strings as None
pub fn empty_string_as_none_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => s.parse().map(Some).map_err(serde::de::Error::custom),
    }
}

/// Save uploaded file to the specified directory
/// Returns the relative path to the saved file (e.g., "/uploads/players/uuid.jpg")
pub async fn save_uploaded_file(
    data: &[u8],
    filename: &str,
    upload_dir: &str,
) -> Result<String, std::io::Error> {
    // Ensure upload directory exists
    fs::create_dir_all(upload_dir).await?;

    // Extract file extension
    let extension = Path::new(filename)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("jpg");

    // Validate extension (only allow images)
    let allowed_extensions = ["jpg", "jpeg", "png", "gif", "webp"];
    if !allowed_extensions.contains(&extension.to_lowercase().as_str()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid file type. Only image files are allowed.",
        ));
    }

    // Generate unique filename using UUID
    let unique_filename = format!("{}.{}", Uuid::new_v4(), extension);
    let file_path = format!("{}/{}", upload_dir, unique_filename);

    // Write file to disk
    let mut file = fs::File::create(&file_path).await?;
    file.write_all(data).await?;
    file.flush().await?;

    // Return the web-accessible path (served under /static)
    // file_path is "static/uploads/players/uuid.jpg" -> web path is "/static/uploads/players/uuid.jpg"
    let web_path = format!("/{}", file_path);
    Ok(web_path)
}

/// Delete uploaded file from disk
pub async fn delete_uploaded_file(file_path: &str) -> Result<(), std::io::Error> {
    // Convert web path back to filesystem path
    // web path is "/static/uploads/..." -> fs path is "static/uploads/..."
    let fs_path = file_path.strip_prefix('/').unwrap_or(file_path);

    if fs::metadata(fs_path).await.is_ok() {
        fs::remove_file(fs_path).await?;
    }

    Ok(())
}
