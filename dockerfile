# Use the official Solana image as a base
FROM solanalabs/solana:v1.18.17

# Install necessary packages
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libudev-dev \
    llvm \
    clang \
    cmake \
    libprotobuf-dev \
    protobuf-compiler \
    openssl \
    git \
    curl \
    libclang-dev

# Install Rust and Anchor
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install --git https://github.com/project-serum/anchor --tag v0.30.1 anchor-cli

# Set the working directory
WORKDIR /workdir

# Copy the current directory contents into the container
COPY . /workdir

# Build the Solana program
RUN cd programs/solana-itus && anchor build -- --features idl-build
