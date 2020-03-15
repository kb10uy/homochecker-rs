# Build container
FROM rust:1.42-slim-buster

WORKDIR /build
COPY . .
RUN apt-get update \
    && apt-get install -y pkg-config libssl-dev \
    && cargo install --path .

# Running container
FROM debian:bullseye-slim
LABEL maintainer="kb10uy"

RUN apt-get update && apt-get install -y libssl1.1
COPY --from=0 /usr/local/cargo/bin/homochecker-rs /usr/local/bin/homochecker-rs

EXPOSE 4546
CMD ["/usr/local/bin/homochecker-rs"]
