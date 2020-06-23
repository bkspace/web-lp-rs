FROM rust:1.44-alpine AS builder
LABEL maintainer="bkspace <sammyisseyegh@gmail.com>"
LABEL version="1.0.0"

WORKDIR /code
RUN apk update \
    && apk add build-base openssl-dev zlib-dev  \
    && rm -rf /var/cache/apk/*
COPY . .
RUN cargo build --release

FROM alpine
LABEL maintainer="bkspace <sammyisseyegh@gmail.com>"
LABEL version="1.0.0"

RUN apk --update add libgcc gcc make g++ zlib-dev

# Download and install GLPK
RUN wget ftp://ftp.gnu.org/gnu/glpk/glpk-4.65.tar.gz
RUN tar -xzvf glpk-4.65.tar.gz
RUN cd glpk-4.65 && ./configure && make install

COPY --from=builder /code/target/release/service /usr/local/bin

EXPOSE 8080
CMD ["service"]
