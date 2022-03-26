FROM ekidd/rust-musl-builder

ENV PATH="/aarch64-linux-musl-cross/bin:/usr/local/cargo/bin/rustup:/root/.cargo/bin:$PATH"

RUN curl -fsSL https://deb.nodesource.com/setup_17.x | sudo -E bash - && \
    sudo apt-get install -y nodejs && \
    sudo npm i -g yarn pnpm

RUN sudo wget https://musl.cc/aarch64-linux-musl-cross.tgz && \
    tar -xvf aarch64-linux-musl-cross.tgz && \
    rm aarch64-linux-musl-cross.tgz

RUN VERS=1.0.18 && \
    cd /home/rust/libs && \
    curl -LO https://download.libsodium.org/libsodium/releases/libsodium-$VERS.tar.gz && \
    tar xzf libsodium-$VERS.tar.gz && cd libsodium-$VERS && \
    CC=musl-gcc ./configure --with-pic --disable-pie --disable-shared --enable-static=yes --prefix=/usr/local/musl && \
    make && sudo make install && \
    cd .. && rm -rf zlib-$VERS.tar.gz zlib-$Vers
