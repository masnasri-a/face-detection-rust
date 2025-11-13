# Face Recognition API

API untuk face recognition menggunakan Rust, Axum, dan OpenCV.

## Prerequisites

### macOS
```bash
# Install OpenCV
brew install opencv

# Install pkg-config
brew install pkg-config
```

### Environment Variables
Buat file `.env` (opsional):
```
HOST=0.0.0.0
PORT=3000
```

## Build & Run

```bash
# Build project
cargo build

# Run server
cargo run
```

Server akan berjalan di `http://localhost:3000`

## API Documentation

### Swagger UI
Akses dokumentasi interaktif Swagger UI di:
```
http://localhost:3000/swagger-ui
```

Swagger UI menyediakan:
- ✅ Dokumentasi lengkap semua endpoints
- ✅ Try it out - test API langsung dari browser
- ✅ Request/response schema detail
- ✅ Example values

### OpenAPI Spec
OpenAPI JSON specification tersedia di:
```
http://localhost:3000/api-docs/openapi.json
```

## API Endpoints

### 1. Add Face Data
**Endpoint:** `POST /add-face`

**Content-Type:** `multipart/form-data`

**Parameters:**
- `id` (text): User ID
- `photos` (files): Multiple image files (jpg, jpeg, png)

**Example using curl:**
```bash
curl -X POST http://localhost:3000/add-face \
  -F "id=user123" \
  -F "photos=@photo1.jpg" \
  -F "photos=@photo2.jpg" \
  -F "photos=@photo3.jpg"
```

**Response:**
```json
{
  "success": true,
  "message": "Face data added and model trained successfully",
  "data": {
    "user_id": "user123",
    "images_saved": 3
  }
}
```

### 2. Detect Face
**Endpoint:** `POST /detect-face`

**Content-Type:** `multipart/form-data`

**Parameters:**
- `photo` (file): Image file untuk deteksi

**Example using curl:**
```bash
curl -X POST http://localhost:3000/detect-face \
  -F "photo=@test_image.jpg"
```

**Response (face detected):**
```json
{
  "success": true,
  "message": "Face detected successfully",
  "data": {
    "user_id": "user123",
    "detected": true
  }
}
```

**Response (no match):**
```json
{
  "success": true,
  "message": "No matching face found",
  "data": {
    "user_id": null,
    "detected": false
  }
}
```

## Cara Kerja

1. **Add Face (`/add-face`):**
   - Upload multiple foto dengan user ID
   - Foto disimpan di folder `knowledge/{user_id}/`
   - Model langsung di-training otomatis setelah upload
   - Setiap kali ada data baru, model di-retrain

2. **Detect Face (`/detect-face`):**
   - Upload satu foto untuk deteksi
   - Sistem akan detect wajah dan mencocokkan dengan data yang sudah di-train
   - Return user ID jika match ditemukan

## Folder Structure

```
face-recognition-be/
├── knowledge/          # Data foto training (auto-created)
│   ├── user123/       # Folder per user ID
│   │   ├── photo1.jpg
│   │   └── photo2.jpg
│   └── user456/
│       └── photo1.jpg
├── temp/              # Temporary files (auto-created)
├── src/
│   ├── main.rs
│   └── face_recognition/
│       ├── mod.rs
│       ├── model.rs       # Face recognition logic
│       └── handlers.rs    # HTTP handlers
└── Cargo.toml
```

## Quick Start with Swagger UI

1. **Start Server:**
   ```bash
   cargo run
   ```

2. **Open Browser:**
   ```
   http://localhost:3000/swagger-ui
   ```

3. **Test API:**
   - Click on endpoint → "Try it out" → Upload files → "Execute"
   - Lihat response langsung di browser!

## Notes

- Folder `knowledge/` akan dibuat otomatis saat pertama kali add data
- Setiap user memiliki sub-folder sendiri di dalam `knowledge/`
- Model menggunakan LBPH (Local Binary Patterns Histograms) dari OpenCV
- Training dilakukan otomatis setiap kali ada penambahan data
- Untuk hasil terbaik, upload minimal 5-10 foto per user dengan berbagai angle
- **Swagger UI** tersedia di `/swagger-ui` untuk dokumentasi interaktif
- **OpenAPI spec** tersedia di `/api-docs/openapi.json`
