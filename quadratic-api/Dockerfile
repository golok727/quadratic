FROM node:18-alpine AS builder
WORKDIR /quadratic
COPY package.json .
COPY package-lock.json .
COPY ./quadratic-api/package*.json ./quadratic-api/
COPY ./quadratic-shared/package*.json ./quadratic-shared/
RUN npm install
COPY quadratic-api ./quadratic-api
COPY quadratic-shared ./quadratic-shared

FROM node:18-slim AS runtime
WORKDIR /quadratic
COPY --from=builder /quadratic .
RUN apt-get update && apt install -y openssl
RUN npm run postinstall --workspace=quadratic-api
RUN npm run build:prod --workspace=quadratic-api
CMD ["npm", "start:prod"]
