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

# Install runtimes
RUN apt-get update && apt-get install -y \
    python3 \
    nodejs \
    default-jre-headless \
    libcap-dev \
    make \
    gcc \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install isolate
RUN git clone https://github.com/ioi/isolate.git /tmp/isolate \
    && cd /tmp/isolate \
    && make \
    && make install BINDIR=/usr/bin CONFIGDIR=/etc \
    && rm -rf /tmp/isolate \

# Copy isolate.conf to /etc
COPY config/isolate.conf /etc/isolate.conf

RUN chown root:root /usr/bin/isolate && chmod 4755 /usr/bin/isolate

# Ensure Java has a valid home and is on the path
ENV JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64
ENV PATH="${JAVA_HOME}/bin:${PATH}"


# Explicitly create the temp directory Java often wants
RUN mkdir -p /tmp && chmod 1777 /tmp

# Create dir for isolate boxes
RUN mkdir -p /var/lib/isolate && chmod 700 /var/lib/isolate



COPY --from=builder /app/target/release/Hermes /usr/bin/Hermes

CMD ["Hermes"]