# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

FROM hyperledger/grid-dev:v1 as dgc-platform-cli-builder

COPY cli /build/cli
COPY daemon /build/daemon
COPY sdk /build/sdk

# Build the dgc-platform-cli package
WORKDIR /build/cli
ARG CARGO_ARGS
ARG REPO_VERSION
RUN sed -i -e "0,/version.*$/ s/version.*$/version\ =\ \"${REPO_VERSION}\"/" Cargo.toml
RUN cargo deb --deb-version $REPO_VERSION $CARGO_ARGS

# -------------=== dgc-platform-cli docker build ===-------------
FROM hyperledger/sawtooth-sabre-cli

ARG CARGO_ARGS
RUN echo "CARGO_ARGS = '$CARGO_ARGS'" > CARGO_ARGS

COPY --from=dgc-platform-cli-builder /build/target/debian/dgc-platform-cli_*.deb /tmp

RUN apt-get update \
 && dpkg --unpack /tmp/dgc-platform-cli*.deb \
 && apt-get -f -y install

CMD ["tail -f /dev/null"]
