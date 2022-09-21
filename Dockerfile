FROM ubuntu:latest

ARG USER
ARG UID
ARG GID

RUN apt-get update && apt-get install -y git less vim clang curl && \
  curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none --default-host x86_64-unknown-linux-gnu && \
  source $HOME/.cargo/env && \
  rustup toolchain install stable-2021-11-01 && \
  rustup default stable-2021-11-01 && \
  rustup target add wasm32-unknown-unknown

RUN useradd --uid ${UID} --gid ${GID} ${USER}
