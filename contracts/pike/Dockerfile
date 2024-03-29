# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

FROM hyperledger/grid-dev:v1 as dgc-platform-pike-builder

COPY contracts/pike /build/contracts/pike
COPY sdk/ /build/sdk/

# Do a release build to cache dependencies
WORKDIR /build/contracts/pike

# Build the contract
ARG REPO_VERSION
RUN sed -i -e "0,/version.*$/ s/version.*$/version\ =\ \"${REPO_VERSION}\"/" Cargo.toml
RUN cargo build --target wasm32-unknown-unknown --release

# Build a scar file
# Copy the packaging directory
COPY contracts/pike/packaging/scar/* \
     /build/contracts/pike/packaging/scar/

# Copy the contract to the packaging directory
RUN cp /build/target/wasm32-unknown-unknown/release/dgc-platform-pike-tp.wasm \
    packaging/scar

WORKDIR /build/contracts/pike/packaging/scar

# Create .scar file
RUN tar -jcvf /tmp/dgc-platform-pike_${REPO_VERSION}.scar .

# -------------=== pike submitter build ===-------------

FROM hyperledger/sawtooth-sabre-cli

COPY --from=dgc-platform-pike-builder \
     /build/target/wasm32-unknown-unknown/release/dgc-platform-pike-tp.wasm /tmp

# Copy the contract definition
COPY contracts/pike/pike.yaml /tmp

COPY --from=dgc-platform-pike-builder \
    /tmp/dgc-platform-pike*.scar /tmp
