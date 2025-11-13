use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;
use utoipa::ToSchema;
use uuid::Uuid;

use super::model::FaceRecognitionModel;
use crate::database::Database;

pub type SharedModel = Arc<Mutex<FaceRecognitionModel>>;
pub type SharedDb = Arc<Database>;

/// Generic API response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    /// Indicates if the request was successful
    success: bool,
    /// Human-readable message describing the result
    message: String,
    /// Optional response data
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

/// Response for adding face data
#[derive(Debug, Serialize, ToSchema)]
pub struct AddFaceResponse {
    /// User ID that was registered
    user_id: String,
    /// Number of images successfully saved
    images_saved: usize,
}

/// Response for face detection
#[derive(Debug, Serialize, ToSchema)]
pub struct DetectFaceResponse {
    /// Detected user ID (null if no match found)
    user_id: Option<String>,
    /// Whether a face was detected
    detected: bool,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };
        
        (status, Json(self)).into_response()
    }
}

/// Add face data for training
///
/// Upload multiple photos for a user and automatically train the face recognition model.
/// Images are stored in `knowledge/{user_id}/` directory.
#[utoipa::path(
    post,
    path = "/add-face",
    request_body(content_type = "multipart/form-data", description = "Upload face photos with user ID"),
    responses(
        (status = 200, description = "Face data added successfully", body = ApiResponse<AddFaceResponse>),
        (status = 400, description = "Bad request - missing required fields"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Face Recognition"
)]
pub async fn add_face(
    State((model, db)): State<(SharedModel, SharedDb)>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<AddFaceResponse>>, (StatusCode, String)> {
    let mut user_id: Option<String> = None;
    let mut saved_count = 0;
    let mut saved_paths: Vec<String> = Vec::new();
    let knowledge_path = "knowledge";

    // Ensure knowledge directory exists
    fs::create_dir_all(knowledge_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();

        if name == "id" {
            // Get user ID
            let data = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            user_id = Some(data);
        } else if name == "photos" {
            // Get the user_id first
            let uid = user_id.clone().ok_or((
                StatusCode::BAD_REQUEST,
                "User ID must be provided before photos".to_string(),
            ))?;

            // Create user directory
            let user_dir = format!("{}/{}", knowledge_path, uid);
            fs::create_dir_all(&user_dir)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            // Get filename
            let filename = field
                .file_name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));

            // Get file data
            let data = field
                .bytes()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            // Save file
            let file_path = format!("{}/{}", user_dir, filename);
            fs::write(&file_path, data)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            saved_paths.push(file_path.clone());
            saved_count += 1;
            println!("Saved image: {}", file_path);
        }
    }

    let uid = user_id.ok_or((
        StatusCode::BAD_REQUEST,
        "User ID is required".to_string(),
    ))?;

    if saved_count == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "No photos were uploaded".to_string(),
        ));
    }

    // Save to database
    db.upsert_user(&uid)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;

    for image_path in &saved_paths {
        db.insert_face_image(&uid, image_path)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    }

    println!("Saved {} images to database for user {}", saved_count, uid);

    // Train model after adding new data
    let mut model_guard = model.lock().await;
    model_guard
        .train(knowledge_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Training failed: {}", e)))?;

    println!("Model retrained successfully");

    Ok(Json(ApiResponse {
        success: true,
        message: "Face data added and model trained successfully".to_string(),
        data: Some(AddFaceResponse {
            user_id: uid,
            images_saved: saved_count,
        }),
    }))
}

/// Detect face and identify user
///
/// Upload an image to detect and identify a face. Returns the user ID if a match is found.
#[utoipa::path(
    post,
    path = "/detect-face",
    request_body(content_type = "multipart/form-data", description = "Upload photo for face detection"),
    responses(
        (status = 200, description = "Detection completed", body = ApiResponse<DetectFaceResponse>),
        (status = 400, description = "Bad request - no image uploaded"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Face Recognition"
)]
pub async fn detect_face(
    State((model, db)): State<(SharedModel, SharedDb)>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<DetectFaceResponse>>, (StatusCode, String)> {
    // Save uploaded image temporarily
    let temp_dir = "temp";
    fs::create_dir_all(temp_dir)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let temp_filename = format!("{}/{}.jpg", temp_dir, Uuid::new_v4());
    let mut image_saved = false;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();

        if name == "photo" || name == "image" {
            let data = field
                .bytes()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            fs::write(&temp_filename, data)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            image_saved = true;
            break;
        }
    }

    if !image_saved {
        return Err((
            StatusCode::BAD_REQUEST,
            "No image uploaded. Use 'photo' or 'image' as field name".to_string(),
        ));
    }

    // Predict using model
    let mut model_guard = model.lock().await;
    
    let result = model_guard
        .predict(&temp_filename)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Detection failed: {}", e)))?;

    let detected = result.is_some();

    // Log detection to database
    db.log_detection(
        result.as_deref(),
        None, // confidence will be added later
        Some(&temp_filename),
    )
    .await
    .map_err(|e| {
        eprintln!("Failed to log detection: {}", e);
    })
    .ok();

    // Clean up temp file
    let _ = fs::remove_file(&temp_filename);

    Ok(Json(ApiResponse {
        success: true,
        message: if detected {
            "Face detected successfully".to_string()
        } else {
            "No matching face found".to_string()
        },
        data: Some(DetectFaceResponse {
            user_id: result,
            detected,
        }),
    }))
}
