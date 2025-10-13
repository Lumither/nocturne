FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN ["cargo", "build", "-r"]

FROM gcr.io/distroless/cc AS production
ARG ARCH
COPY --from=builder /app/target/release/server ./server
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libz.so.1 /usr/lib/${ARCH}-linux-gnu/
ENTRYPOINT ["./server"]