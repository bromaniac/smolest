FROM scratch
ADD target/x86_64-unknown-linux-musl/release/smolest ./app/smolest
WORKDIR /app
EXPOSE 8000
CMD ["/app/smolest"]

