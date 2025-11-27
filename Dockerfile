FROM rust:latest AS build

ARG TARGETPLATFORM

WORKDIR /app
COPY . /app

RUN case ${TARGETPLATFORM} in \
    "linux/amd64")  echo "x86_64-unknown-linux-musl" > .toolchain ;; \
    "linux/arm64")  echo "aarch64-unknown-linux-musl" > .toolchain ;; \
    *)              echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac

RUN apt-get update
RUN apt-get install -y musl-tools

RUN rustup target add $(cat .toolchain)

RUN cargo build --release --target $(cat .toolchain)
RUN cp ./target/$(cat .toolchain)/release/vigil-local ./

FROM scratch

WORKDIR /usr/src/vigil-local

COPY --from=build /app/vigil-local /usr/local/bin/vigil-local

CMD [ "vigil-local", "-c", "/etc/vigil-local.cfg" ]
