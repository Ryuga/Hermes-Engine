FROM rustlang/rust:nightly-bookworm AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rustlang/rust:nightly-bookworm AS builder
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

# Deployment
FROM debian:bookworm-slim
WORKDIR /app

# Install Runtimes and Repo Tools
RUN apt-get update && apt-get install -y \
    python3 \
    nodejs \
    default-jre-headless \
    curl \
    gnupg \
    procps \
    && rm -rf /var/lib/apt/lists/*

# Setup Isolate Repository
RUN mkdir -p /etc/apt/keyrings && \
    curl https://www.ucw.cz/isolate/debian/signing-key.asc > /etc/apt/keyrings/isolate.asc

RUN echo "Types: deb\n\
URIs: http://www.ucw.cz/isolate/debian/\n\
Suites: bookworm-isolate\n\
Components: main\n\
Architectures: amd64\n\
Signed-By: /etc/apt/keyrings/isolate.asc" > /etc/apt/sources.list.d/isolate.sources

# Update and Install Isolate
RUN apt-get update && apt-get install -y isolate && rm -rf /var/lib/apt/lists/*

# Configuration & Permissions
COPY config/isolate.conf /etc/isolate.conf
RUN chown root:root /usr/bin/isolate && chmod 4755 /usr/bin/isolate

# Runtime Environment Setup
ENV JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64
ENV PATH="${JAVA_HOME}/bin:${PATH}"

# Ensure directories match your Rust PathBuf logic
RUN mkdir -p /tmp && chmod 1777 /tmp && \
    mkdir -p /var/lib/isolate && chmod 700 /var/lib/isolate

# Deploy Hermes
COPY --from=builder /app/target/release/Hermes /usr/bin/Hermes

CMD ["/usr/bin/Hermes"]