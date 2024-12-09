FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN ["cargo", "build", "-r"]

FROM gcr.io/distroless/cc AS production
ARG ARCH
COPY --from=builder /app/target/release/api_server ./api_server
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libz.so.1 /usr/lib/${ARCH}-linux-gnu/
ENTRYPOINT ["./api_server"]