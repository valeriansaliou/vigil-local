FROM rust:latest AS build

ARG PREBUILT_TAG
ARG TARGETPLATFORM

ENV PREBUILT_TAG=$PREBUILT_TAG

WORKDIR /app
COPY . /app

RUN case ${TARGETPLATFORM} in \
    "linux/amd64")  echo "x86_64" > .arch && echo "x86_64-unknown-linux-musl" > .toolchain ;; \
    "linux/arm64")  echo "aarch64" > .arch && echo "aarch64-unknown-linux-musl" > .toolchain ;; \
    *)              echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac

# Run full build?
RUN if [ -z "$PREBUILT_TAG" ]; then \
    apt-get update && \
        apt-get install -y musl-tools && \
        rustup target add $(cat .toolchain) \
    ; fi
RUN if [ -z "$PREBUILT_TAG" ]; then \
    cargo build --release --target $(cat .toolchain) && \
        mkdir -p ./vigil-local/ && \
        mv ./target/$(cat .toolchain)/release/vigil-local ./vigil-local/ \
    ; fi

# Pull pre-built binary?
RUN if [ ! -z "$PREBUILT_TAG" ]; then \
    wget https://github.com/valeriansaliou/vigil-local/releases/download/$PREBUILT_TAG/$PREBUILT_TAG-$(cat .arch).tar.gz && \
        tar -xzf $PREBUILT_TAG-$(cat .arch).tar.gz \
    ; fi

FROM scratch

WORKDIR /usr/src/vigil-local

COPY --from=build /app/vigil-local/vigil-local /usr/local/bin/vigil-local

CMD [ "vigil-local", "-c", "/etc/vigil-local.cfg" ]
