# Builder stage
FROM rust:slim AS builder

# Install dependencies
# See https://stackoverflow.com/questions/65553557/why-rust-is-failing-to-build-command-for-openssl-sys-v0-9-60-even-after-local-in
# See https://stackoverflow.com/questions/65538591/run-protoc-command-into-docker-container
RUN apt-get update && \
    apt-get install -y bash && \
    apt-get install -y libssl-dev && \
    apt-get install -y pkg-config && \
    update-ca-certificates

# Create a working directory
WORKDIR /build

# Copy the projects into the image
COPY . . 


# Build workspcae
RUN cargo update && cargo clean && cargo build --release --workspace

# -----
# Take a debian image as base (must be the same as the builder image base, to have the same libc)
# See https://stackoverflow.com/questions/69010070/rust-linux-version-glibc-not-found-compile-for-different-glibc-libc6-version
# More information on the builder image: https://hub.docker.com/layers/library/rust/slim/images/sha256-ed7795c6eaccae53be35939e883e8c3de0197b21e8eddbd9f04b0c4bc757c094?context=explore
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y bash && \
    apt-get install -y file && \
    apt-get install -y binutils && \
    apt-get install -y libssl-dev && \
    apt-get install -y pkg-config && \
    apt-get install -y ca-certificates && \
    update-ca-certificates

# Define some labels
LABEL org.opencontainers.image.title="RITE - Rust Import/Transform/Export" \
      org.opencontainers.image.description="RITE - Rust Import/Transform/Export" \
      org.opencontainers.image.base.name="debian:bookworm-slim" \
      org.opencontainers.image.vendor="CIAO Systems GmbH"

# Create a non-root user for security
RUN groupadd -r rite && useradd -r -g rite rite


# Setup the application directory
WORKDIR /app

# Copy the built binary from the builder image to the WORKDIR
COPY --from=builder /build/target/release/*.so .
COPY --from=builder /build/target/release/rite .

# Set the library path
ENV LD_LIBRARY_PATH=/lib:/lib64:/app

# Create mount point for input files
RUN mkdir /data 
## && chown rite:rite /data

COPY --from=builder /build/log4rs.yaml /data
RUN ln -s /data/log4rs.yaml /app/log4rs.yaml

RUN mkdir /logs 
## && chown -R rite:rite /logs && chmod -R g+w /logs

VOLUME ["/logs"]


# Switch to non-root user
# USER rite

# Command to run the binary
ENTRYPOINT ["/app/rite", "-f"]


