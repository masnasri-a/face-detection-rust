# Dockerfile for Backend (Rust + OpenCV)
FROM rust:1.75-slim-bookworm

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    clang \
    libclang-dev \
    llvm-dev \
    # OpenCV dependencies
    libopencv-dev \
    libopencv-core-dev \
    libopencv-imgproc-dev \
    libopencv-imgcodecs-dev \
    libopencv-objdetect-dev \
    libopencv-face-dev \
    # Additional tools
    wget \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables for OpenCV
ENV OPENCV_INCLUDE_PATHS=/usr/include/opencv4 \
    OPENCV_LINK_PATHS=/usr/lib/x86_64-linux-gnu \
    PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src
COPY schema.sql ./

# Build dependencies and application
RUN cargo build --release

# Create directories for runtime
RUN mkdir -p /app/knowledge /app/temp

# Expose port
EXPOSE 8080

# Set environment variables
ENV HOST=0.0.0.0
ENV PORT=8080
ENV DATABASE_URL=sqlite:face_recognition.db
ENV RUST_LOG=info

# Run the binary
CMD ["./target/release/face-recognition-be"]
