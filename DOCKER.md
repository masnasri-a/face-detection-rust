# Docker Deployment Guide

## 🐳 Quick Start with Docker

### Prerequisites
- Docker 20.10+
- Docker Compose 2.0+

### Build and Run

```bash
# Build and start all services
docker-compose up --build

# Or run in detached mode
docker-compose up -d --build
```

### Access Services

- **Frontend**: http://localhost:3000
- **Backend API**: http://localhost:8080
- **Swagger UI**: http://localhost:8080/swagger-ui

## 📦 Services

### Backend (Port 8080)
- Rust application with Axum
- OpenCV for face recognition
- SQLite database
- LBPH face recognition algorithm

### Frontend (Port 3000)
- Next.js 16 with React 19
- TailwindCSS 4
- Camera integration
- Real-time face detection

## 🔧 Commands

### Start Services
```bash
# Start all services
docker-compose up

# Start in background
docker-compose up -d

# Build and start
docker-compose up --build
```

### Stop Services
```bash
# Stop all services
docker-compose down

# Stop and remove volumes
docker-compose down -v
```

### View Logs
```bash
# All services
docker-compose logs

# Specific service
docker-compose logs backend
docker-compose logs frontend

# Follow logs
docker-compose logs -f
```

### Restart Services
```bash
# Restart all
docker-compose restart

# Restart specific service
docker-compose restart backend
docker-compose restart frontend
```

### Rebuild
```bash
# Rebuild all images
docker-compose build

# Rebuild specific service
docker-compose build backend
docker-compose build frontend

# Rebuild without cache
docker-compose build --no-cache
```

## 📁 Volumes

### Backend Volumes
- `./knowledge:/app/knowledge` - Face training data
- `./temp:/app/temp` - Temporary detection files
- `./face_recognition.db:/app/face_recognition.db` - SQLite database

### Persistent Data
Data is stored on host machine, so it persists between container restarts.

## 🔍 Health Checks

Both services have health checks configured:

### Backend
- Endpoint: `http://localhost:8080/`
- Interval: 30s
- Timeout: 10s
- Retries: 3

### Frontend
- Endpoint: `http://localhost:3000/`
- Interval: 30s
- Timeout: 10s
- Retries: 3

Check health status:
```bash
docker-compose ps
```

## 🌐 Networking

Services communicate through `face-recognition-network` bridge network.

Frontend connects to backend via `http://backend:8080` internally.

## 🐛 Troubleshooting

### Backend won't start
```bash
# Check logs
docker-compose logs backend

# Common issues:
# 1. OpenCV installation failed - check Dockerfile
# 2. Port 8080 already in use
# 3. Rust compilation errors
```

### Frontend won't start
```bash
# Check logs
docker-compose logs frontend

# Common issues:
# 1. Port 3000 already in use
# 2. npm install failed
# 3. Can't connect to backend
```

### Database issues
```bash
# Remove database and restart
docker-compose down
rm face_recognition.db
docker-compose up -d
```

### Permission issues
```bash
# Fix permissions for mounted volumes
sudo chown -R $(whoami):$(whoami) knowledge temp
```

### Rebuild from scratch
```bash
# Remove everything and rebuild
docker-compose down -v
docker-compose build --no-cache
docker-compose up -d
```

## 📊 Resource Usage

### Backend Container
- Memory: ~500MB
- CPU: Variable (depends on face recognition workload)
- Disk: ~2GB (with OpenCV)

### Frontend Container
- Memory: ~100MB
- CPU: Low
- Disk: ~500MB

## 🔒 Security Notes

### Production Deployment

1. **Change default ports**
```yaml
ports:
  - "8080:8080"  # Change to custom port
```

2. **Add authentication**
- Implement JWT or API keys
- Add CORS restrictions

3. **Use HTTPS**
- Add reverse proxy (nginx/traefik)
- Use Let's Encrypt certificates

4. **Secure database**
- Use PostgreSQL instead of SQLite
- Add database credentials

5. **Environment variables**
```bash
# Create .env file
echo "DATABASE_URL=postgresql://user:pass@db:5432/facedb" > .env
```

## 🚀 Production Setup

### With Nginx Reverse Proxy

```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - backend
      - frontend

  backend:
    # ... backend config
    expose:
      - "8080"
    # Remove ports mapping

  frontend:
    # ... frontend config
    expose:
      - "3000"
    # Remove ports mapping
```

### Using PostgreSQL

```yaml
services:
  db:
    image: postgres:15
    environment:
      POSTGRES_DB: facedb
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: changeme
    volumes:
      - postgres_data:/var/lib/postgresql/data

  backend:
    environment:
      DATABASE_URL: postgresql://postgres:changeme@db:5432/facedb
    depends_on:
      - db

volumes:
  postgres_data:
```

## 📝 Development vs Production

### Development
```bash
# Use local development
cargo run  # Backend
npm run dev  # Frontend
```

### Staging/Testing
```bash
# Use Docker Compose
docker-compose up
```

### Production
```bash
# Use Docker Compose with production config
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## 🔄 Updates

### Update Backend
```bash
# Pull latest code
git pull

# Rebuild backend only
docker-compose build backend
docker-compose up -d backend
```

### Update Frontend
```bash
# Pull latest code
git pull

# Rebuild frontend only
docker-compose build frontend
docker-compose up -d frontend
```

### Update Both
```bash
git pull
docker-compose up -d --build
```

## 📈 Monitoring

### Container Stats
```bash
# Real-time stats
docker stats

# Specific container
docker stats face-recognition-backend
```

### Logs Analysis
```bash
# Last 100 lines
docker-compose logs --tail=100

# Errors only
docker-compose logs | grep ERROR
```

## 🧪 Testing

### Test Backend API
```bash
# Health check
curl http://localhost:8080/

# Add face
curl -X POST http://localhost:8080/add-face \
  -F "id=test" \
  -F "photos=@photo.jpg"

# Detect face
curl -X POST http://localhost:8080/detect-face \
  -F "photo=@test.jpg"
```

### Test Frontend
```bash
# Open in browser
open http://localhost:3000
```

## 📚 Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [OpenCV Docker](https://hub.docker.com/r/opencvcourses/opencv-docker)
- [Rust Docker](https://hub.docker.com/_/rust)

## 💡 Tips

1. **Faster Builds**: Use Docker BuildKit
   ```bash
   DOCKER_BUILDKIT=1 docker-compose build
   ```

2. **Multi-stage Builds**: Already implemented in Dockerfiles

3. **Layer Caching**: Dependencies cached separately from source code

4. **Volume Performance**: Use named volumes for better performance
   ```yaml
   volumes:
     - knowledge_data:/app/knowledge
   ```

5. **Resource Limits**: Add resource constraints
   ```yaml
   deploy:
     resources:
       limits:
         cpus: '2'
         memory: 2G
   ```

---

**Last Updated**: November 13, 2025
