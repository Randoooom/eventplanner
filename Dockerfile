FROM --platform=linux/amd64 rust:slim-buster AS rust-build

RUN apt-get update
RUN apt-get install build-essential libssl-dev pkg-config clang lld protobuf-compiler gcc-aarch64-linux-gnu libc6-dev-arm64-cross musl-tools -y

# precompile the dependencies as such without code changes
RUN USER=root cargo init --bin --name eventplanner
COPY ./Cargo.toml ./Cargo.toml

RUN cargo update

RUN rm -rf src/*
COPY ./src ./src

ARG TARGETARCH

RUN if [ "$TARGETARCH" = "arm64" ]; then \
        mkdir .cargo; \
        echo '[target.aarch64-unknown-linux-gnu]\nlinker = "aarch64-linux-gnu-gcc"' >> .cargo/config.toml; \
        rustup target add aarch64-unknown-linux-gnu; \
        cargo build --release --target aarch64-unknown-linux-gnu; \
        mv target/aarch64-unknown-linux-gnu/release/eventplanner target/release/eventplanner; \
    else \
        cargo build --release; \
    fi

FROM --platform=linux/amd64 node:18-alpine AS build-static

# prepare pnpm
RUN npm i -g pnpm

# install the dependencies
COPY ./frontend/package.json ./frontend/
COPY ./frontend/pnpm-lock.yaml ./frontend/
COPY ./frontend/.npmrc ./frontend/
WORKDIR ./frontend
RUN pnpm i --frozen-lockfile

# build static target
COPY ./frontend/ ./
RUN pnpm run generate

FROM --platform=linux/amd64 node:18-alpine AS build-ssr

# prepare pnpm
RUN npm i -g pnpm

# install the dependencies
COPY ./frontend/package.json ./frontend/
COPY ./frontend/pnpm-lock.yaml ./frontend/
COPY ./frontend/.npmrc ./frontend/
WORKDIR ./frontend
RUN pnpm i --frozen-lockfile

# build
COPY ./frontend/ ./
RUN pnpm run build

FROM nginx:latest as static

RUN apt-get update
RUN apt-get install libssl-dev ca-certificates -y

COPY --from=rust-build /target/release/eventplanner ./
COPY --from=build-static /frontend/.output/ ./.output/

COPY ./static.conf /etc/nginx/conf.d/default.conf
RUN ln -sf /dev/stdout /var/log/nginx/access.log && ln -sf /dev/stderr /var/log/nginx/error.log

ARG NUXT_PUBLIC_SURREALDB_ENDPOINT

CMD ./eventplanner \
    && find /.output/public -type f -exec sed -i 's@NUXT_PUBLIC_SURREALDB_ENDPOINT@'"$NUXT_PUBLIC_SURREALDB_ENDPOINT"'@' {} + \
    && nginx -g "daemon off;"

FROM node:18-buster-slim AS ssr

COPY --from=rust-build /target/release/eventplanner ./
COPY --from=build-ssr /frontend/.output/ ./.output

CMD ./eventplanner && node .output/server/index.mjs

