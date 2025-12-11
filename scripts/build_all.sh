#!/bin/bash
# Build all service Docker images
# Usage: ./scripts/build_all.sh

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Variables
REGISTRY="ghcr.io/your-org"
IMAGE_NAME="rs-service-template"
VERSION=$(git describe --tags --always --dirty 2>/dev/null || echo "dev")
SERVICES=("api" "worker" "grpc" "cli")

echo -e "${GREEN}===============================================${NC}"
echo -e "${GREEN}Building all service Docker images${NC}"
echo -e "${GREEN}Version: $VERSION${NC}"
echo -e "${GREEN}===============================================${NC}"

# Build each service
for service in "${SERVICES[@]}"; do
    echo -e "${YELLOW}Building $service service...${NC}"
    docker build \
        --build-arg SERVICE_NAME=$service \
        -t $REGISTRY/$IMAGE_NAME-$service:$VERSION \
        -t $REGISTRY/$IMAGE_NAME-$service:latest \
        .

    if [ $? -ne 0 ]; then
        echo -e "${RED}Error: Build failed for $service${NC}"
        exit 1
    fi

    echo -e "${GREEN} $service built successfully${NC}"
    echo ""
done

echo -e "${GREEN}===============================================${NC}"
echo -e "${GREEN}All services built successfully!${NC}"
echo -e "${GREEN}===============================================${NC}"
echo ""

# Show images
echo -e "${YELLOW}Built images:${NC}"
for service in "${SERVICES[@]}"; do
    echo "  $REGISTRY/$IMAGE_NAME-$service:$VERSION"
    echo "  $REGISTRY/$IMAGE_NAME-$service:latest"
done
echo ""

echo -e "${YELLOW}To push all images, run:${NC}"
echo "  make docker-push-all"
echo "  or"
echo "  docker push $REGISTRY/$IMAGE_NAME-api:$VERSION && \\"
echo "  docker push $REGISTRY/$IMAGE_NAME-worker:$VERSION && \\"
echo "  docker push $REGISTRY/$IMAGE_NAME-grpc:$VERSION && \\"
echo "  docker push $REGISTRY/$IMAGE_NAME-cli:$VERSION"
