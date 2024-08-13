# docker/dev.Dockerfile
FROM node:latest

WORKDIR /app

COPY package.json ./

RUN npm install

ENV NEXT_TELEMETRY_DISABLED 1

CMD npm run dev

