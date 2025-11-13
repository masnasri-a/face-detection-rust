# Setup Guide - Face Recognition API

## Install OpenCV di macOS

```bash
# Install Homebrew jika belum ada
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install OpenCV dan dependencies
brew install opencv pkg-config llvm

# Verify installation
pkg-config --modversion opencv4
```

## Set Environment Variables

Untuk memastikan Rust bisa menemukan OpenCV, tambahkan ke `.zshrc` atau `.bash_profile`:

```bash
# OpenCV paths
export PKG_CONFIG_PATH="/usr/local/opt/opencv/lib/pkgconfig:$PKG_CONFIG_PATH"
export DYLD_LIBRARY_PATH="/usr/local/opt/opencv/lib:$DYLD_LIBRARY_PATH"

# LLVM (untuk opencv-rust binding)
export PATH="/usr/local/opt/llvm/bin:$PATH"
export LDFLAGS="-L/usr/local/opt/llvm/lib"
export CPPFLAGS="-I/usr/local/opt/llvm/include"
```

Lalu reload shell:
```bash
source ~/.zshrc  # atau source ~/.bash_profile
```

## Build Project

```bash
cd /Volumes/External/nuratech/face-recognition-be
cargo build --release
```

## Run Server

```bash
cargo run --release
```

## Testing Endpoints

### 1. Test Add Face
```bash
# Siapkan beberapa foto wajah (minimal 5-10 foto untuk hasil terbaik)
curl -X POST http://localhost:3000/add-face \
  -F "id=john_doe" \
  -F "photos=@./photos/john1.jpg" \
  -F "photos=@./photos/john2.jpg" \
  -F "photos=@./photos/john3.jpg"
```

### 2. Test Detect Face
```bash
curl -X POST http://localhost:3000/detect-face \
  -F "photo=@./photos/test_john.jpg"
```

## Troubleshooting

### Error: opencv package not found
```bash
# Check opencv installation
brew list opencv

# Reinstall jika perlu
brew reinstall opencv
```

### Error: haarcascade file not found
File haarcascade biasanya ada di:
- `/usr/local/share/opencv4/haarcascades/`
- `/opt/homebrew/share/opencv4/haarcascades/` (Apple Silicon)

Update path di `src/face_recognition/model.rs` jika berbeda.

### Error: clang not found
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Atau install LLVM
brew install llvm
```

## Performance Tips

1. **Upload Foto Berkualitas:**
   - Resolusi minimal 640x480
   - Wajah terlihat jelas dan frontal
   - Lighting yang baik
   - Minimal 5-10 foto per user dengan berbagai angle

2. **Model Training:**
   - Setiap penambahan data akan trigger re-training
   - Training time tergantung jumlah foto
   - Untuk production, pertimbangkan async training

3. **Confidence Threshold:**
   - Default threshold: 80.0
   - Lower = more strict matching
   - Adjust di `src/face_recognition/model.rs` jika diperlukan
