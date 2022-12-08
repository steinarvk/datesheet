FROM clux/muslrust:stable as builder

WORKDIR /build/

RUN mkdir src && echo "fn main() {print!(\"Hello world\");} // dummy file" > src/main.rs
COPY Cargo.lock Cargo.toml /build/
RUN cargo build --target x86_64-unknown-linux-musl --release && rm ./target/x86_64-unknown-linux-musl/release/datesheet

COPY scripts/ /build/scripts/
RUN ./scripts/download-tiny-font.sh

COPY src/ /build/src/
RUN touch src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release && mv ./target/x86_64-unknown-linux-musl/release/datesheet /build/build-output

FROM gruebel/upx:latest as upx
COPY --from=builder /build/build-output /build/upx-build-output
RUN upx /build/upx-build-output

FROM scratch

WORKDIR /

COPY --from=upx /build/upx-build-output /datesheet

CMD ["/datesheet"]
