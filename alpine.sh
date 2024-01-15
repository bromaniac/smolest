apk add curl build-base libgcc
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup target add x86_64-unknown-linux-musl
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
