I couldn't get cross compilation to work so build this in an Alpine container. 
docker run --rm -it -v ./:mnt alpine sh

All deps needed are in the alpine.sh script so run that.

Build command:
RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release

Then build the Docker image on any system:
docker build -t smolest .

Run it:
docker run --rm -it --init -p 8000:8000 smolest