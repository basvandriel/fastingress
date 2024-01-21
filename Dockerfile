FROM rust:latest

# 2. Copy the files in your machine to the Docker image
COPY . .

# Build your program for release
RUN cargo build --release

EXPOSE 3000
ENV HOST_OUT=1

# Run the binary
CMD ["./target/release/fastingress"]