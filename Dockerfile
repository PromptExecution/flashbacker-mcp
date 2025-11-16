# Flashbacker - Claude Code state management
# For b00t datum registry integration

FROM node:22-alpine

# Install build dependencies for native modules
RUN apk add --no-cache \
    python3 \
    make \
    g++ \
    git \
    bash

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./
COPY tsconfig.json ./

# Copy source and templates
COPY src/ ./src/
COPY bin/ ./bin/
COPY templates/ ./templates/
COPY scripts/ ./scripts/

# Install and build
RUN npm ci --production=false && \
    npm run build && \
    npm link

# Copy flashbacker-mcp server
COPY flashbacker-mcp ./
RUN chmod +x flashbacker-mcp

# Create workspace
RUN mkdir -p /workspace
WORKDIR /workspace

# Verify installation
RUN flashback --version

# Labels for b00t datum registry
LABEL maintainer="flashbacker"
LABEL description="Claude Code state management with session continuity"
LABEL version="2.4.1"
LABEL b00t.datum="flashbacker"
LABEL b00t.type="docker"

# Default: show help
CMD ["flashback", "--help"]
