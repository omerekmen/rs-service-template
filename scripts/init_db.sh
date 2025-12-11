#!/bin/bash
# Database initialization script
# Usage: ./scripts/init_db.sh <environment>
# Example: ./scripts/init_db.sh dev

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Variables
ENV=${1:-dev}

# Validate environment
if [[ ! "$ENV" =~ ^(dev|staging|prod)$ ]]; then
    echo -e "${RED}Error: Invalid environment. Use dev, staging, or prod.${NC}"
    exit 1
fi

echo -e "${GREEN}===============================================${NC}"
echo -e "${GREEN}Initializing database for $ENV environment${NC}"
echo -e "${GREEN}===============================================${NC}"

# Set database URL based on environment
case $ENV in
    dev)
        DB_URL="postgresql://postgres:postgres@localhost:5432/rs_service_dev"
        ;;
    staging)
        if [ -z "$STAGING_DATABASE_URL" ]; then
            echo -e "${RED}Error: STAGING_DATABASE_URL environment variable not set${NC}"
            exit 1
        fi
        DB_URL=$STAGING_DATABASE_URL
        ;;
    prod)
        echo -e "${RED}Error: Do not run database initialization directly in production!${NC}"
        echo -e "${YELLOW}Use Kubernetes migration job or manual process instead.${NC}"
        exit 1
        ;;
esac

echo -e "${YELLOW}Database URL: $DB_URL${NC}"

# Export for sqlx
export DATABASE_URL=$DB_URL

# Run migrations
echo -e "${YELLOW}Running migrations...${NC}"
sqlx migrate run --source crates/infrastructure/migrations

if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Migration failed${NC}"
    exit 1
fi

echo -e "${GREEN}===============================================${NC}"
echo -e "${GREEN}Database initialized successfully!${NC}"
echo -e "${GREEN}Environment: $ENV${NC}"
echo -e "${GREEN}===============================================${NC}"
