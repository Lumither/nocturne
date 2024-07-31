# docker/dev.Dockerfile
FROM oven/bun:latest

WORKDIR /app

COPY package.json ./
COPY bun.lockb ./

RUN bun install

ENV NEXT_TELEMETRY_DISABLED 1

CMD bun run dev

