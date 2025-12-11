#!/bin/bash
# Deploy script for rs-service-template
# Usage: ./scripts/deploy.sh <environment> <service>
# Example: ./scripts/deploy.sh dev api

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Variables
ENV=${1:-dev}
SERVICE=${2:-api}
REGISTRY="ghcr.io/your-org"
IMAGE_NAME="rs-service-template"

# Validate environment
if [[ ! "$ENV" =~ ^(dev|staging|prod)$ ]]; then
    echo -e "${RED}Error: Invalid environment. Use dev, staging, or prod.${NC}"
    exit 1
fi

# Validate service
if [[ ! "$SERVICE" =~ ^(api|worker|grpc|cli)$ ]]; then
    echo -e "${RED}Error: Invalid service. Use api, worker, grpc, or cli.${NC}"
    exit 1
fi

# Get current git commit/tag
VERSION=$(git describe --tags --always --dirty 2>/dev/null || echo "dev")

echo -e "${GREEN}===============================================${NC}"
echo -e "${GREEN}Deploying $SERVICE to $ENV environment${NC}"
echo -e "${GREEN}Version: $VERSION${NC}"
echo -e "${GREEN}===============================================${NC}"

# Build Docker image
echo -e "${YELLOW}Step 1: Building Docker image...${NC}"
docker build \
    --build-arg SERVICE_NAME=$SERVICE \
    -t $REGISTRY/$IMAGE_NAME-$SERVICE:$VERSION \
    -t $REGISTRY/$IMAGE_NAME-$SERVICE:latest \
    .

if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Docker build failed${NC}"
    exit 1
fi

# Push to registry
echo -e "${YELLOW}Step 2: Pushing to registry...${NC}"
docker push $REGISTRY/$IMAGE_NAME-$SERVICE:$VERSION
docker push $REGISTRY/$IMAGE_NAME-$SERVICE:latest

if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Docker push failed${NC}"
    exit 1
fi

# Update Kubernetes deployment
echo -e "${YELLOW}Step 3: Updating Kubernetes deployment...${NC}"
NAMESPACE="rs-service-$ENV"

# Update image in deployment
kubectl set image deployment/$SERVICE \
    $SERVICE=$REGISTRY/$IMAGE_NAME-$SERVICE:$VERSION \
    -n $NAMESPACE

if [ $? -ne 0 ]; then
    echo -e "${RED}Error: kubectl set image failed${NC}"
    exit 1
fi

# Wait for rollout
echo -e "${YELLOW}Step 4: Waiting for rollout to complete...${NC}"
kubectl rollout status deployment/$SERVICE -n $NAMESPACE --timeout=300s

if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Rollout failed or timed out${NC}"
    exit 1
fi

# Verify deployment
echo -e "${YELLOW}Step 5: Verifying deployment...${NC}"
kubectl get pods -l app=$SERVICE -n $NAMESPACE

echo -e "${GREEN}===============================================${NC}"
echo -e "${GREEN}Deployment complete!${NC}"
echo -e "${GREEN}Service: $SERVICE${NC}"
echo -e "${GREEN}Environment: $ENV${NC}"
echo -e "${GREEN}Version: $VERSION${NC}"
echo -e "${GREEN}===============================================${NC}"
