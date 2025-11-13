# Test Examples

## Using cURL

### 1. Add Face Data
```bash
# Single user with multiple photos
curl -X POST http://localhost:3000/add-face \
  -F "id=user001" \
  -F "photos=@./test_photos/user001_1.jpg" \
  -F "photos=@./test_photos/user001_2.jpg" \
  -F "photos=@./test_photos/user001_3.jpg" \
  -F "photos=@./test_photos/user001_4.jpg" \
  -F "photos=@./test_photos/user001_5.jpg"

# Add another user
curl -X POST http://localhost:3000/add-face \
  -F "id=user002" \
  -F "photos=@./test_photos/user002_1.jpg" \
  -F "photos=@./test_photos/user002_2.jpg" \
  -F "photos=@./test_photos/user002_3.jpg"
```

### 2. Detect Face
```bash
# Test with known face
curl -X POST http://localhost:3000/detect-face \
  -F "photo=@./test_photos/test_face.jpg"

# Expected response (if match found):
# {
#   "success": true,
#   "message": "Face detected successfully",
#   "data": {
#     "user_id": "user001",
#     "detected": true
#   }
# }

# Expected response (if no match):
# {
#   "success": true,
#   "message": "No matching face found",
#   "data": {
#     "user_id": null,
#     "detected": false
#   }
# }
```

## Using HTTPie

### Install HTTPie
```bash
brew install httpie
```

### Add Face
```bash
http -f POST http://localhost:3000/add-face \
  id=user001 \
  photos@./test_photos/user001_1.jpg \
  photos@./test_photos/user001_2.jpg
```

### Detect Face
```bash
http -f POST http://localhost:3000/detect-face \
  photo@./test_photos/test_face.jpg
```

## Using Python

```python
import requests

# Add face
def add_face(user_id, photo_paths):
    url = "http://localhost:3000/add-face"
    
    files = [('photos', open(photo, 'rb')) for photo in photo_paths]
    data = {'id': user_id}
    
    response = requests.post(url, data=data, files=files)
    
    for file in files:
        file[1].close()
    
    return response.json()

# Detect face
def detect_face(photo_path):
    url = "http://localhost:3000/detect-face"
    
    with open(photo_path, 'rb') as f:
        files = {'photo': f}
        response = requests.post(url, files=files)
    
    return response.json()

# Usage
result = add_face('user001', [
    './photos/user001_1.jpg',
    './photos/user001_2.jpg',
    './photos/user001_3.jpg'
])
print(result)

result = detect_face('./photos/test.jpg')
print(result)
```

## Using JavaScript/Node.js

```javascript
const FormData = require('form-data');
const fs = require('fs');
const axios = require('axios');

// Add face
async function addFace(userId, photoPaths) {
    const form = new FormData();
    form.append('id', userId);
    
    photoPaths.forEach(path => {
        form.append('photos', fs.createReadStream(path));
    });
    
    const response = await axios.post('http://localhost:3000/add-face', form, {
        headers: form.getHeaders()
    });
    
    return response.data;
}

// Detect face
async function detectFace(photoPath) {
    const form = new FormData();
    form.append('photo', fs.createReadStream(photoPath));
    
    const response = await axios.post('http://localhost:3000/detect-face', form, {
        headers: form.getHeaders()
    });
    
    return response.data;
}

// Usage
(async () => {
    const addResult = await addFace('user001', [
        './photos/user001_1.jpg',
        './photos/user001_2.jpg',
        './photos/user001_3.jpg'
    ]);
    console.log(addResult);
    
    const detectResult = await detectFace('./photos/test.jpg');
    console.log(detectResult);
})();
```

## Testing Workflow

1. **Prepare Test Photos:**
   ```bash
   mkdir -p test_photos
   # Add photos of different people
   ```

2. **Add Multiple Users:**
   ```bash
   # User 1
   curl -X POST http://localhost:3000/add-face \
     -F "id=alice" \
     -F "photos=@./test_photos/alice1.jpg" \
     -F "photos=@./test_photos/alice2.jpg" \
     -F "photos=@./test_photos/alice3.jpg"
   
   # User 2
   curl -X POST http://localhost:3000/add-face \
     -F "id=bob" \
     -F "photos=@./test_photos/bob1.jpg" \
     -F "photos=@./test_photos/bob2.jpg" \
     -F "photos=@./test_photos/bob3.jpg"
   ```

3. **Test Detection:**
   ```bash
   # Should return alice
   curl -X POST http://localhost:3000/detect-face \
     -F "photo=@./test_photos/alice_test.jpg"
   
   # Should return bob
   curl -X POST http://localhost:3000/detect-face \
     -F "photo=@./test_photos/bob_test.jpg"
   
   # Should return null (unknown person)
   curl -X POST http://localhost:3000/detect-face \
     -F "photo=@./test_photos/unknown.jpg"
   ```

## Notes

- Gunakan foto berkualitas baik dengan wajah yang jelas
- Minimal 5-10 foto per user untuk hasil optimal
- Foto dengan berbagai angle dan ekspresi lebih baik
- Format yang didukung: JPG, JPEG, PNG
