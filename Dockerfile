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
    build-essential \
    python3 \
    nodejs \
    default-jdk-headless \
    curl \
    gnupg \
    procps \
    && rm -rf /var/lib/apt/lists/*

# Runtime Environment Setup
ENV JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64
ENV PATH="${JAVA_HOME}/bin:${PATH}"

# Cache Pre-Compiled Java STL
RUN mkdir -p /tmp/java_share && \
    java -Xshare:dump -XX:SharedArchiveFile=/tmp/java_share/classes.jsa && \
    echo "public class A {}" > /tmp/A.java && \
    java -XX:DumpLoadedClassList=/tmp/java_share/javac.classlist \
         --add-modules=jdk.compiler \
         -m jdk.compiler/com.sun.tools.javac.Main /tmp/A.java && \
    rm /tmp/A.java /app/A.class 2>/dev/null || true && \
    java -Xshare:dump \
         -XX:SharedClassListFile=/tmp/java_share/javac.classlist \
         -XX:SharedArchiveFile=/tmp/java_share/javac.jsa \
         --add-modules=jdk.compiler

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
COPY assets/isolate.conf /etc/isolate
RUN chown root:root /usr/bin/isolate && chmod 4755 /usr/bin/isolate

# Ensure directories match your Rust PathBuf logic
RUN mkdir -p /tmp && chmod 1777 /tmp && \
    mkdir -p /var/lib/isolate && chmod 700 /var/lib/isolate

# Copy config
COPY config.json /app/config.json
# Copy .env if present
COPY .env* /app/

# Deploy Hermes
COPY --from=builder /app/target/release/Hermes /usr/bin/Hermes

COPY entrypoint.sh /usr/bin/entrypoint.sh
RUN chmod +x /usr/bin/entrypoint.sh

ENTRYPOINT ["/usr/bin/entrypoint.sh"]
CMD ["/usr/bin/Hermes"]
