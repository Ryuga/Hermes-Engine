FROM --platform=linux/amd64 rustlang/rust:nightly-bookworm AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo build --release

FROM --platform=linux/amd64 ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    openjdk-21-jdk \
    python3 \
    nodejs \
    && mkdir -p /etc/apt/keyrings

RUN curl https://www.ucw.cz/isolate/debian/signing-key.asc > /etc/apt/keyrings/isolate.asc

RUN echo "Types: deb\n\
URIs: http://www.ucw.cz/isolate/debian/\n\
Suites: noble-isolate\n\
Components: main\n\
Architectures: amd64\n\
Signed-By: /etc/apt/keyrings/isolate.asc" > /etc/apt/sources.list.d/isolate.sources

RUN apt-get update && apt-get install -y isolate

WORKDIR /app

COPY --from=builder /app/target/release/Hermes .
COPY config/isolate.conf /etc/isolate.conf

RUN chmod u+s /usr/bin/isolate

ENV JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64

CMD ["./Hermes"]