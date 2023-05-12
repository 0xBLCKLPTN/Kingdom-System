FROM rust:1.67

COPY ./ ./

RUN cargo build --release
EXPOSE 5800:5800
CMD ["./target/release/api"]
