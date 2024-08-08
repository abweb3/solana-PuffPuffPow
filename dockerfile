FROM rust:1.57.0 as builder

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build --release --features idl-build

FROM gcr.io/distroless/cc
COPY --from=builder /usr/src/myapp/target/release/myapp /usr/local/bin/myapp

CMD ["myapp"]
