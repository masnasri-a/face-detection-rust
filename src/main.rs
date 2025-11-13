mod face_recognition;
mod database;

use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{CorsLayer, Any};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use database::Database;
use face_recognition::{
    FaceRecognitionModel,
    handlers::{add_face, detect_face, SharedModel, SharedDb, ApiResponse, AddFaceResponse, DetectFaceResponse},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        face_recognition::handlers::add_face,
        face_recognition::handlers::detect_face,
    ),
    components(
        schemas(ApiResponse<AddFaceResponse>, ApiResponse<DetectFaceResponse>, AddFaceResponse, DetectFaceResponse)
    ),
    tags(
        (name = "Face Recognition", description = "Face recognition and detection endpoints")
    ),
    info(
        title = "Face Recognition API",
        version = "1.0.0",
        description = "API for face recognition and detection using OpenCV and LBPH algorithm",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    // Initialize database
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:face_recognition.db".to_string());
    let db = match Database::new(&database_url).await {
        Ok(db) => {
            println!("Database initialized successfully");
            db
        }
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return;
        }
    };
    let shared_db: SharedDb = Arc::new(db);

    // Initialize face recognition model
    let model = match FaceRecognitionModel::new() {
        Ok(m) => {
            println!("Face recognition model initialized successfully");
            m
        }
        Err(e) => {
            eprintln!("Failed to initialize face recognition model: {}", e);
            eprintln!("Make sure OpenCV is installed and haarcascade file is available");
            return;
        }
    };

    let shared_model: SharedModel = Arc::new(Mutex::new(model));

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router with face recognition endpoints
    let app = Router::new()
        .route("/", get(|| async { "Face Recognition API - Visit /swagger-ui for documentation" }))
        .route("/add-face", post(add_face))
        .route("/detect-face", post(detect_face))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .with_state((shared_model, shared_db));

    println!("Face Recognition API listening on {}", addr);
    println!("Endpoints:");
    println!("  GET  /swagger-ui         - Swagger UI documentation");
    println!("  POST /add-face           - Add face data (multipart: id, photos)");
    println!("  POST /detect-face        - Detect face (multipart: photo)");

    // Run server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}