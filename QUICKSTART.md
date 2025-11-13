# Face Recognition System - Quick Start

## 🚀 Quick Start

### 1. Start Backend (Terminal 1)

```bash
cd /Volumes/External/nuratech/face-recognition-be
cargo run
```

Backend will run on: `http://localhost:8080`

### 2. Start Frontend (Terminal 2)

```bash
cd /Volumes/External/nuratech/face-recognition-be/frontend
npm install  # First time only
npm run dev
```

Frontend will run on: `http://localhost:3000`

## 📱 Usage

### Add Face
1. Open `http://localhost:3000/add-face`
2. Enter User ID
3. Capture 5-10 photos
4. Click "Submit & Train"

### Detect Face
1. Open `http://localhost:3000/detect`
2. Click "Start Auto Detection"
3. System will identify faces in real-time

## 🔧 Ports

- Backend API: `8080`
- Frontend: `3000`
- Swagger UI: `http://localhost:8080/swagger-ui`

## ✅ Requirements

- Rust & Cargo
- Node.js 20+
- OpenCV installed
- Camera access

Enjoy! 🎉
