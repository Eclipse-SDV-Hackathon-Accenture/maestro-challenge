FROM docker.io/alpine:3.18.4 as compile
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apk update && apk add --update-cache \
    curl \
    # Development tools
    protobuf \
    protobuf-dev \
    protoc \
    build-base \
    && curl --proto '=https' --tlsv1.2 -sS https://sh.rustup.rs | sh -s -- -y > /dev/null \
    && rm -rf /var/cache/apk/*

COPY . /workspaces/build
WORKDIR /workspaces/build
RUN cargo build --release

# stage prod
FROM docker.io/alpine:3.18.4
COPY --from=compile /workspaces/build/target/release/resource_statistics /usr/local/bin/resource_statistics
RUN chmod +x /usr/local/bin/resource_statistics
ENTRYPOINT ["/usr/local/bin/resource_statistics"]