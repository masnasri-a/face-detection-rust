# Swagger/OpenAPI Documentation

## Overview

Face Recognition API sekarang dilengkapi dengan Swagger UI untuk dokumentasi interaktif dan testing.

## Akses Swagger UI

Setelah server berjalan, buka browser dan akses:
```
http://localhost:3000/swagger-ui
```

## Fitur Swagger UI

### 1. **Interactive Documentation**
- Lihat semua endpoint yang tersedia
- Detail request/response schema
- Example values untuk setiap field

### 2. **Try It Out**
Anda bisa test API langsung dari Swagger UI:

#### Test Add Face:
1. Klik endpoint `POST /add-face`
2. Klik tombol "Try it out"
3. Upload file:
   - Field `id`: Masukkan user ID (contoh: "user001")
   - Field `photos`: Klik "Choose Files" dan pilih multiple foto
4. Klik "Execute"
5. Lihat response di bawah

#### Test Detect Face:
1. Klik endpoint `POST /detect-face`
2. Klik tombol "Try it out"
3. Upload file:
   - Field `photo`: Klik "Choose File" dan pilih satu foto
4. Klik "Execute"
5. Lihat response - akan menunjukkan user_id jika terdeteksi

### 3. **Schema Documentation**

Swagger menampilkan schema detail untuk:

#### ApiResponse\<AddFaceResponse\>
```json
{
  "success": true,
  "message": "string",
  "data": {
    "user_id": "string",
    "images_saved": 0
  }
}
```

#### ApiResponse\<DetectFaceResponse\>
```json
{
  "success": true,
  "message": "string",
  "data": {
    "user_id": "string or null",
    "detected": true
  }
}
```

## OpenAPI Specification

OpenAPI JSON spec tersedia di:
```
http://localhost:3000/api-docs/openapi.json
```

Anda bisa menggunakan spec ini untuk:
- Generate client code (berbagai bahasa)
- Import ke Postman Collection
- API testing tools lainnya
- CI/CD integration

## Import ke Postman

1. Buka Postman
2. Klik "Import"
3. Pilih "Link" tab
4. Paste URL: `http://localhost:3000/api-docs/openapi.json`
5. Klik "Continue" dan "Import"

Semua endpoint akan otomatis tersedia di Postman Collection!

## Generate Client Code

Menggunakan OpenAPI Generator:

```bash
# Install openapi-generator
npm install -g @openapitools/openapi-generator-cli

# Generate Python client
openapi-generator-cli generate \
  -i http://localhost:3000/api-docs/openapi.json \
  -g python \
  -o ./clients/python

# Generate JavaScript/TypeScript client
openapi-generator-cli generate \
  -i http://localhost:3000/api-docs/openapi.json \
  -g typescript-axios \
  -o ./clients/typescript

# Generate Java client
openapi-generator-cli generate \
  -i http://localhost:3000/api-docs/openapi.json \
  -g java \
  -o ./clients/java
```

## Dependencies

Swagger diimplementasikan menggunakan:
- `utoipa` - OpenAPI documentation generation
- `utoipa-swagger-ui` - Swagger UI integration untuk Axum

```toml
[dependencies]
utoipa = { version = "5.2", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "8.0", features = ["axum"] }
```

## Customization

Untuk mengubah metadata API, edit `src/main.rs`:

```rust
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Face Recognition API",
        version = "1.0.0",
        description = "Your custom description",
        contact(
            name = "Your Name",
            email = "your@email.com"
        )
    )
)]
struct ApiDoc;
```

## Production Notes

Untuk production, pertimbangkan:

1. **Disable Swagger di Production** (opsional):
```rust
#[cfg(debug_assertions)]
let app = app.merge(SwaggerUi::new("/swagger-ui")...);
```

2. **Rate Limiting**: Tambahkan rate limiting untuk endpoints
3. **Authentication**: Implementasi auth middleware
4. **HTTPS**: Gunakan HTTPS di production

## Screenshot

Swagger UI akan menampilkan:
- ✅ List semua endpoints dengan HTTP methods
- ✅ Tag grouping (Face Recognition)
- ✅ Request body schema dengan file upload
- ✅ Response codes (200, 400, 500)
- ✅ Try it out button untuk testing
- ✅ Example responses

## Troubleshooting

### Swagger UI tidak muncul
- Pastikan server berjalan
- Cek console untuk error messages
- Pastikan dependencies sudah di-build: `cargo build`

### OpenAPI spec tidak valid
- Check `cargo build` untuk compile errors
- Pastikan semua structs memiliki `#[derive(ToSchema)]`
- Pastikan path annotations benar

## Resources

- [utoipa Documentation](https://github.com/juhaku/utoipa)
- [OpenAPI Specification](https://swagger.io/specification/)
- [Swagger UI](https://swagger.io/tools/swagger-ui/)
