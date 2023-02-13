FROM ubuntu

COPY ./dockerentry.sh .
COPY ./target/release/infinityauth_backend_rust .
#CMD ["./infinityauth_backend_rust"]
ENTRYPOINT ./dockerentry.sh