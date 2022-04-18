FROM rust

RUN apt update -y && apt install zsh wget cmake -y
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# install webassembly toolkit
RUN git clone --recursive https://github.com/WebAssembly/wabt  \
    && cd wabt && mkdir build && cd build \
    && git submodule update --init \
    && cmake .. && cmake --build . \
    && mv /wa* /usr/bin && rm -rf ../../wabt

RUN wget https://github.com/bytecodealliance/wasmtime/releases/download/v0.35.1/wasmtime-v0.35.1-aarch64-linux.tar.xz \
    && tar -xf wasmtime-v0.35.1-aarch64-linux.tar.xz \
    && mv wasmtime-v0.35.1-aarch64-linux/wasmtime /usr/bin && rm -rf wasmtime-v0.35.1-aarch64-linux
