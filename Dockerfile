# ---- Base image with Rust ----
FROM rust:1.76-bullseye as builder

# Install system dependencies required by Tauri
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    build-essential \
    pkg-config \
    libglib2.0-dev \
    libgdk-pixbuf-2.0-dev \
    libpango1.0-dev \
    libatk1.0-dev \
    libcairo2-dev \
    libx11-dev \
    libxcb1-dev \
    libxrandr-dev \
    libxi-dev \
    libxext-dev \
    libxfixes-dev \
    libxcomposite-dev \
    libxcursor-dev \
    libxdamage-dev \
    libxrender-dev \
    libxtst-dev \
    libnss3 \
    libasound2 \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

# ---- Install Node (for frontend) ----
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs

# ---- Install Tauri CLI ----
RUN cargo install tauri-cli

# ---- App directory ----
WORKDIR /app

# Copy package files first (better caching)
COPY package.json package-lock.json* bun.lockb* ./

# Install frontend deps
RUN npm install
# or: RUN bun install

# Copy rest of project
COPY . .

# ---- Build frontend ----
RUN npm run build
# or: bun run build

# ---- Build Tauri app ----
RUN cargo tauri build

# ---- Final minimal image ----
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-0 \
    libgtk-3-0 \
    libayatana-appindicator3-1 \
    libnss3 \
    libasound2 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy built binary
COPY --from=builder /app/src-tauri/target/release/bundle /app/bundle

# Default command (optional)
CMD ["ls", "/app/bundle"]