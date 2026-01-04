use axum::extract::Multipart;

/// Raw player form data parsed from multipart form
///
/// This struct represents the raw data extracted from the multipart form
/// before validation. All fields except name are optional in the form.
#[derive(Debug, Default)]
pub struct PlayerFormData {
    pub name: String,
    pub country_id: Option<i64>,
    pub photo_path: Option<String>,
    pub photo_url: Option<String>,
    pub birth_date: Option<String>,
    pub birth_place: Option<String>,
    pub height_cm: Option<i64>,
    pub weight_kg: Option<i64>,
    pub position: Option<String>,
    pub shoots: Option<String>,
}

/// Parse player form data from multipart request
///
/// This function extracts all player fields from the multipart form,
/// including file uploads. File uploads are handled separately and
/// the resulting path is stored in photo_path.
///
/// # Arguments
/// * `multipart` - The multipart form data
/// * `upload_dir` - Directory to save uploaded photos (e.g., "static/uploads/players")
/// * `old_photo_path` - Path to old photo for deletion (for updates only)
///
/// # Returns
/// * `Ok(PlayerFormData)` - The parsed form data
/// * `Err(String)` - Error message if file upload fails
pub async fn parse_player_form(
    multipart: &mut Multipart,
    upload_dir: &str,
    old_photo_path: Option<&str>,
) -> Result<PlayerFormData, String> {
    let mut form_data = PlayerFormData::default();

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "name" => {
                form_data.name = field.text().await.unwrap_or_default();
            }
            "country_id" => {
                let text = field.text().await.unwrap_or_default();
                form_data.country_id = text.parse().ok();
            }
            "photo_url" => {
                form_data.photo_url = Some(field.text().await.unwrap_or_default());
            }
            "birth_date" => {
                let text = field.text().await.unwrap_or_default();
                form_data.birth_date = if text.trim().is_empty() {
                    None
                } else {
                    Some(text)
                };
            }
            "birth_place" => {
                let text = field.text().await.unwrap_or_default();
                form_data.birth_place = if text.trim().is_empty() {
                    None
                } else {
                    Some(text)
                };
            }
            "height_cm" => {
                let text = field.text().await.unwrap_or_default();
                form_data.height_cm = if text.trim().is_empty() {
                    None
                } else {
                    text.parse().ok()
                };
            }
            "weight_kg" => {
                let text = field.text().await.unwrap_or_default();
                form_data.weight_kg = if text.trim().is_empty() {
                    None
                } else {
                    text.parse().ok()
                };
            }
            "position" => {
                let text = field.text().await.unwrap_or_default();
                form_data.position = if text.trim().is_empty() {
                    None
                } else {
                    Some(text)
                };
            }
            "shoots" => {
                let text = field.text().await.unwrap_or_default();
                form_data.shoots = if text.trim().is_empty() {
                    None
                } else {
                    Some(text)
                };
            }
            "photo_file" => {
                // Handle file upload
                let filename = field.file_name().unwrap_or("photo.jpg").to_string();
                let data = field.bytes().await.unwrap_or_default();

                if !data.is_empty() {
                    match crate::utils::save_uploaded_file(&data, &filename, upload_dir).await {
                        Ok(path) => {
                            // Delete old photo if it exists and is an uploaded file (for updates)
                            if let Some(old_path) = old_photo_path {
                                if old_path.starts_with("/static/uploads/") {
                                    let _ = crate::utils::delete_uploaded_file(old_path).await;
                                }
                            }
                            form_data.photo_path = Some(path);
                        }
                        Err(e) => {
                            tracing::error!("Failed to save uploaded file: {}", e);
                            return Err(
                                "Failed to save photo. Only image files (jpg, png, gif, webp) are allowed."
                                    .to_string(),
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(form_data)
}

/// Resolve the final photo path for a player
///
/// This helper determines which photo source to use based on priority:
/// 1. Newly uploaded file (if provided)
/// 2. Photo URL (if provided and not empty)
/// 3. Existing photo path (fallback for updates)
///
/// # Arguments
/// * `uploaded_photo` - Path to newly uploaded photo file
/// * `photo_url` - Photo URL from form
/// * `existing_photo` - Current photo path (for updates only)
///
/// # Returns
/// The final photo path to use, or None if no photo is provided
pub fn resolve_photo_path(
    uploaded_photo: Option<String>,
    photo_url: Option<String>,
    existing_photo: Option<String>,
) -> Option<String> {
    if uploaded_photo.is_some() {
        uploaded_photo
    } else if let Some(url) = photo_url {
        if url.trim().is_empty() {
            existing_photo
        } else {
            Some(url)
        }
    } else {
        existing_photo
    }
}
