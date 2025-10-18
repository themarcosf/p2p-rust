### Environment setup #########################################################

FROM rust:1.90.0-bookworm AS base

# Disable interactive prompts during apt
ENV DEBIAN_FRONTEND='noninteractive'

# Project root folder
ENV PROJECT_ROOT='/var/app'

# Update system and install helpers
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends curl git build-essential ca-certificates; \
    rm -rf /var/lib/apt/lists/*

# Create system group and a dedicated non-root user (app)
RUN groupadd -r app && useradd -m -r -g app -s /bin/bash app

# Create project root and set ownership
RUN mkdir -p "$PROJECT_ROOT" && chown -R app:app "$PROJECT_ROOT" /home/app

# Set working directory
WORKDIR $PROJECT_ROOT

# Add init.sh to profile.d for shell environment setup
COPY docker/init.sh /etc/profile.d/init.sh

### App setup #################################################################

# Copy source files
COPY --chown=app:app src $PROJECT_ROOT/src

# Switch to app user
USER app

### Rust setup ################################################################

# Ensure cargo bin is on PATH
ENV PATH="/usr/local/cargo/bin:${PATH}"

# Install Rust binary crates
RUN cargo install --locked evcxr_repl@0.21.1

# Set default profile to DEV
ENV PROFILE=DEV
