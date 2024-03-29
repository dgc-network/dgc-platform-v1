# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

FROM hyperledger/grid-dev:v1 as dgc-platform-daemon-builder

COPY cli /build/cli
COPY daemon /build/daemon
COPY sdk /build/sdk

# Build the dgc-platform-daemon package
WORKDIR /build/daemon
ARG CARGO_ARGS
ARG REPO_VERSION
RUN sed -i -e "0,/version.*$/ s/version.*$/version\ =\ \"${REPO_VERSION}\"/" Cargo.toml
RUN cargo deb --deb-version $REPO_VERSION $CARGO_ARGS

# Build the dgc-platform-cli package
WORKDIR /build/cli
ARG CARGO_ARGS
ARG REPO_VERSION
RUN sed -i -e "0,/version.*$/ s/version.*$/version\ =\ \"${REPO_VERSION}\"/" Cargo.toml
RUN cargo deb --deb-version $REPO_VERSION $CARGO_ARGS

# -------------=== dgc-platform-daemon docker build ===-------------
FROM ubuntu:bionic

ARG CARGO_ARGS
RUN echo "CARGO_ARGS = '$CARGO_ARGS'" > CARGO_ARGS

COPY --from=dgc-platform-daemon-builder /build/target/debian/dgc-platform-cli_*.deb /tmp
COPY --from=dgc-platform-daemon-builder /build/target/debian/dgc-platform-daemon_*.deb /tmp

RUN apt-get update \
 && apt-get install -y -q \
    postgresql-client \
 && dpkg --unpack /tmp/dgc-platform*.deb \
 && apt-get -f -y install

CMD ["dgc-platform-daemon"]
