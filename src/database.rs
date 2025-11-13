use anyhow::Result;
use chrono::Utc;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use sqlx::Row;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        // Parse the database URL and enable create_if_missing
        let options = SqliteConnectOptions::from_str(database_url)?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        let db = Self { pool };
        db.init_schema().await?;
        
        Ok(db)
    }

    async fn init_schema(&self) -> Result<()> {
        // Create users table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create face_images table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS face_images (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id TEXT NOT NULL,
                image_path TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create detection_logs table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS detection_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                detected_user_id TEXT,
                confidence REAL,
                image_path TEXT,
                detected_at TEXT NOT NULL DEFAULT (datetime('now'))
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_user_id ON users(user_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_face_images_user_id ON face_images(user_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_detection_logs_detected_at ON detection_logs(detected_at)")
            .execute(&self.pool)
            .await?;

        println!("Database schema initialized successfully");
        Ok(())
    }

    // Insert or update user
    pub async fn upsert_user(&self, user_id: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        sqlx::query(
            r#"
            INSERT INTO users (user_id, created_at, updated_at)
            VALUES (?, ?, ?)
            ON CONFLICT(user_id) DO UPDATE SET updated_at = ?
            "#,
        )
        .bind(user_id)
        .bind(&now)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Insert face image record
    pub async fn insert_face_image(&self, user_id: &str, image_path: &str) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO face_images (user_id, image_path)
            VALUES (?, ?)
            "#,
        )
        .bind(user_id)
        .bind(image_path)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    // Get all images for a user
    pub async fn get_user_images(&self, user_id: &str) -> Result<Vec<String>> {
        let rows = sqlx::query(
            r#"
            SELECT image_path FROM face_images
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let images: Vec<String> = rows
            .iter()
            .map(|row| row.get("image_path"))
            .collect();

        Ok(images)
    }

    // Log detection result
    pub async fn log_detection(
        &self,
        detected_user_id: Option<&str>,
        confidence: Option<f64>,
        image_path: Option<&str>,
    ) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO detection_logs (detected_user_id, confidence, image_path)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(detected_user_id)
        .bind(confidence)
        .bind(image_path)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    // Get user stats
    pub async fn get_user_stats(&self, user_id: &str) -> Result<(i64, i64)> {
        // Get image count
        let image_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM face_images WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        // Get detection count
        let detection_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM detection_logs WHERE detected_user_id = ?"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok((image_count, detection_count))
    }

    // Get all users
    pub async fn get_all_users(&self) -> Result<Vec<String>> {
        let rows = sqlx::query(
            r#"
            SELECT user_id FROM users
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let users: Vec<String> = rows
            .iter()
            .map(|row| row.get("user_id"))
            .collect();

        Ok(users)
    }

    // Get recent detections
    pub async fn get_recent_detections(&self, limit: i64) -> Result<Vec<DetectionLog>> {
        let rows = sqlx::query(
            r#"
            SELECT detected_user_id, confidence, image_path, detected_at
            FROM detection_logs
            ORDER BY detected_at DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let logs: Vec<DetectionLog> = rows
            .iter()
            .map(|row| DetectionLog {
                detected_user_id: row.get("detected_user_id"),
                confidence: row.get("confidence"),
                image_path: row.get("image_path"),
                detected_at: row.get("detected_at"),
            })
            .collect();

        Ok(logs)
    }
}

#[derive(Debug, Clone)]
pub struct DetectionLog {
    pub detected_user_id: Option<String>,
    pub confidence: Option<f64>,
    pub image_path: Option<String>,
    pub detected_at: String,
}
