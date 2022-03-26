FROM messense/rust-musl-cross:aarch64-musl

RUN apk add --update --no-cache wget musl-dev && \
    sed -i -e 's/v[[:digit:]]\..*\//edge\//g' /etc/apk/repositories && \
    apk add --update --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/edge/testing \
    rustup \
    bash \
    python3 \
    python2 \
    git \
    build-base \
    clang \
    cmake \
    llvm \
    gn \
    tar \
    libgcc \
    libstdc++ \
    curl \
    ninja && \
    apk upgrade

RUN curl -fLO https://github.com/oznu/alpine-node/releases/download/16.14.0/node-v16.14.0-linux-aarch64-alpine.tar.gz && \
    tar -xvf node-v16.14.0-linux-aarch64-alpine.tar.gz -C /usr --strip-components=1 --no-same-owner && \
    rm -rf node-v16.14.0-linux-aarch64-alpine.tar.gz && \
    npm install -g pnpm lerna yarn

RUN rustup update nightly && \
    rustup target add --toolchain beta aarch64-unknown-linux-musl
