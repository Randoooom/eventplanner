FROM rust:slim-buster as rust-build

RUN apt-get update
RUN apt-get install build-essential libssl-dev pkg-config clang lld protobuf-compiler -y
RUN rustup default nightly-2023-04-21-x86_64-unknown-linux-gnu

# precompile the dependencies as such without code changes
RUN USER=root cargo init --bin --name eventplanner
COPY ./Cargo.toml ./Cargo.toml

RUN cargo update
RUN cargo build --release
# remove unnecessary files
RUN rm -rf src/*

COPY ./src ./src
RUN rm -rf ./target/release/deps/eventplanner*
# build the release version
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11:latest as backend-standalone

COPY --from=rust-build /target/release/eventplanner .

CMD ./eventplanner

FROM node:16-alpine as build-static

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

FROM node:16-alpine as build-ssr

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

FROM node:16-buster-slim as ssr

COPY --from=rust-build /target/release/eventplanner ./
COPY --from=build-ssr /frontend/.output/ ./.output

CMD ./eventplanner && node .output/server/index.mjs

