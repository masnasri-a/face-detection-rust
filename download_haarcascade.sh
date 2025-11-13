#!/bin/bash

# Download haarcascade file for face detection
# This is needed if OpenCV is not installed or haarcascade files are missing

echo "Downloading haarcascade_frontalface_default.xml..."

curl -L -o haarcascade_frontalface_default.xml \
  "https://raw.githubusercontent.com/opencv/opencv/master/data/haarcascades/haarcascade_frontalface_default.xml"

if [ -f haarcascade_frontalface_default.xml ]; then
    echo "✅ Successfully downloaded haarcascade_frontalface_default.xml"
    echo "File saved to: $(pwd)/haarcascade_frontalface_default.xml"
else
    echo "❌ Failed to download haarcascade file"
    exit 1
fi
