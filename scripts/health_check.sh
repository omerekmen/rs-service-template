#!/bin/bash
# Health check script for Kubernetes deployments
# Usage: ./scripts/health_check.sh <environment>
# Example: ./scripts/health_check.sh dev

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Variables
ENV=${1:-dev}
NAMESPACE="rs-service-$ENV"

# Validate environment
if [[ ! "$ENV" =~ ^(dev|staging|prod)$ ]]; then
    echo -e "${RED}Error: Invalid environment. Use dev, staging, or prod.${NC}"
    exit 1
fi

echo -e "${GREEN}===============================================${NC}"
echo -e "${GREEN}Health check for $ENV environment${NC}"
echo -e "${GREEN}Namespace: $NAMESPACE${NC}"
echo -e "${GREEN}===============================================${NC}"

# Check if namespace exists
echo -e "${YELLOW}Checking namespace...${NC}"
if ! kubectl get namespace $NAMESPACE >/dev/null 2>&1; then
    echo -e "${RED}Error: Namespace $NAMESPACE does not exist${NC}"
    exit 1
fi
echo -e "${GREEN} Namespace exists${NC}"

# Check deployments
echo -e "${YELLOW}Checking deployments...${NC}"
kubectl get deployment -n $NAMESPACE
echo ""

# Check API deployment
echo -e "${YELLOW}Checking API deployment status...${NC}"
kubectl get deployment api -n $NAMESPACE -o wide
echo ""

# Check pod status
echo -e "${YELLOW}Checking pod status...${NC}"
kubectl get pods -l app=api -n $NAMESPACE
echo ""

# Check for unhealthy pods
UNHEALTHY_PODS=$(kubectl get pods -l app=api -n $NAMESPACE --field-selector=status.phase!=Running --no-headers 2>/dev/null | wc -l)
if [ $UNHEALTHY_PODS -gt 0 ]; then
    echo -e "${RED}Warning: Found $UNHEALTHY_PODS unhealthy pods${NC}"
    kubectl get pods -l app=api -n $NAMESPACE --field-selector=status.phase!=Running
else
    echo -e "${GREEN} All pods are healthy${NC}"
fi
echo ""

# Check services
echo -e "${YELLOW}Checking services...${NC}"
kubectl get svc -n $NAMESPACE
echo ""

# Try to call health endpoint (if accessible)
echo -e "${YELLOW}Checking health endpoint...${NC}"
POD=$(kubectl get pod -l app=api -n $NAMESPACE -o jsonpath='{.items[0].metadata.name}' 2>/dev/null)
if [ -n "$POD" ]; then
    echo -e "${YELLOW}Testing health endpoint on pod: $POD${NC}"
    kubectl exec $POD -n $NAMESPACE -- wget -qO- http://localhost:8080/health 2>/dev/null || \
        echo -e "${YELLOW}Warning: Could not reach health endpoint (pod may not be ready)${NC}"
else
    echo -e "${RED}No API pods found${NC}"
fi
echo ""

# Check HPA (if exists)
echo -e "${YELLOW}Checking Horizontal Pod Autoscaler...${NC}"
if kubectl get hpa api-hpa -n $NAMESPACE >/dev/null 2>&1; then
    kubectl get hpa api-hpa -n $NAMESPACE
else
    echo -e "${YELLOW}HPA not configured${NC}"
fi
echo ""

# Check recent events
echo -e "${YELLOW}Recent events (last 10):${NC}"
kubectl get events -n $NAMESPACE --sort-by='.lastTimestamp' | tail -n 10
echo ""

echo -e "${GREEN}===============================================${NC}"
echo -e "${GREEN}Health check complete!${NC}"
echo -e "${GREEN}===============================================${NC}"
