This was created for a fun challenge from a colleague:

Create the smallest possible Docker image for a web application that:

Replies HTTP 200 for /

Replies HTTP 200 "OK" for /health

I couldn't get cross compilation to work so build this in an Alpine container:

docker run --rm -it -v .:/mnt alpine sh

All deps needed are in the alpine.sh script so run that:

$ sh alpine.sh

Build command:

RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release

Then build the Docker image on any system:

docker build -t smolest .

Run it:

docker run --rm -it --init -p 8000:8000 smolest

To make this even smaller probably involves using no_std... good luck!
