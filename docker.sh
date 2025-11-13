#!/bin/bash

# Face Recognition Docker Management Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Functions
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ $1${NC}"
}

check_docker() {
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install Docker first."
        exit 1
    fi
    
    if ! command -v docker-compose &> /dev/null; then
        print_error "Docker Compose is not installed. Please install Docker Compose first."
        exit 1
    fi
    
    print_success "Docker and Docker Compose are installed"
}

build_services() {
    print_info "Building services..."
    docker-compose build --no-cache
    print_success "Services built successfully"
}

start_services() {
    print_info "Starting services..."
    docker-compose up -d
    print_success "Services started successfully"
    
    print_info "Waiting for services to be healthy..."
    sleep 5
    
    print_info "Service Status:"
    docker-compose ps
}

stop_services() {
    print_info "Stopping services..."
    docker-compose down
    print_success "Services stopped successfully"
}

restart_services() {
    print_info "Restarting services..."
    docker-compose restart
    print_success "Services restarted successfully"
}

show_logs() {
    if [ -z "$1" ]; then
        docker-compose logs -f
    else
        docker-compose logs -f "$1"
    fi
}

show_status() {
    print_info "Service Status:"
    docker-compose ps
    echo ""
    print_info "Resource Usage:"
    docker stats --no-stream face-recognition-backend face-recognition-frontend
}

clean_all() {
    print_info "Cleaning up..."
    docker-compose down -v
    docker system prune -f
    print_success "Cleanup completed"
}

# Main script
case "$1" in
    build)
        check_docker
        build_services
        ;;
    start)
        check_docker
        start_services
        ;;
    stop)
        check_docker
        stop_services
        ;;
    restart)
        check_docker
        restart_services
        ;;
    logs)
        check_docker
        show_logs "$2"
        ;;
    status)
        check_docker
        show_status
        ;;
    clean)
        check_docker
        clean_all
        ;;
    rebuild)
        check_docker
        stop_services
        build_services
        start_services
        ;;
    *)
        echo "Face Recognition Docker Management"
        echo ""
        echo "Usage: $0 {build|start|stop|restart|logs|status|clean|rebuild}"
        echo ""
        echo "Commands:"
        echo "  build    - Build Docker images"
        echo "  start    - Start all services"
        echo "  stop     - Stop all services"
        echo "  restart  - Restart all services"
        echo "  logs     - Show logs (optional: specify service name)"
        echo "  status   - Show service status and resource usage"
        echo "  clean    - Stop and remove all containers and volumes"
        echo "  rebuild  - Stop, rebuild, and start all services"
        echo ""
        echo "Examples:"
        echo "  $0 build"
        echo "  $0 start"
        echo "  $0 logs backend"
        echo "  $0 status"
        exit 1
        ;;
esac

exit 0
