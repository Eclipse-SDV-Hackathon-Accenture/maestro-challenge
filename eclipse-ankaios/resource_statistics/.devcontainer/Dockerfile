FROM ghcr.io/eclipse-ankaios/app-ankaios-dev:latest as dev
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get update && apt-get -y install \
    musl \
    musl-dev \
    musl-tools\
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
RUN rustup target add x86_64-unknown-linux-musl && rustup toolchain add stable-x86_64-unknown-linux-musl

# stage compile
FROM dev as compile
COPY . /workspaces/build
WORKDIR /workspaces/build
RUN cargo build --release

# stage prod
FROM docker.io/library/alpine:latest
COPY --from=compile /workspaces/build/target/x86_64-unknown-linux-musl/release/resource_statistics /usr/local/bin/resource_statistics
RUN chmod +x /usr/local/bin/resource_statistics

ENTRYPOINT [ "/usr/local/bin/resource_statistics" ]
