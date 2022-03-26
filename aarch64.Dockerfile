FROM ekidd/rust-musl-builder

RUN curl -fsSL https://deb.nodesource.com/setup_17.x | sudo -E bash - && \
    sudo apt-get install -y nodejs && \
    sudo npm i -g yarn pnpm

RUN VERS=1.0.18 && \
    cd /home/rust/libs && \
    curl -LO https://download.libsodium.org/libsodium/releases/libsodium-$VERS.tar.gz && \
    tar xzf libsodium-$VERS.tar.gz && cd libsodium-$VERS && \
    CC=musl-gcc ./configure --static --prefix=/usr/local/musl && \
    make && sudo make install && \
    cd .. && rm -rf zlib-$VERS.tar.gz zlib-$Vers
