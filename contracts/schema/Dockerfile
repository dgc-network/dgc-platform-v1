# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

FROM hyperledger/grid-dev:v1 as dgc-platform-schema-builder

COPY contracts/schema /build/contracts/schema
COPY sdk/ /build/sdk/

# Do a release build to cache dependencies
WORKDIR /build/contracts/schema

# Build the contract
ARG REPO_VERSION
RUN sed -i -e "0,/version.*$/ s/version.*$/version\ =\ \"${REPO_VERSION}\"/" Cargo.toml
RUN cargo build --target wasm32-unknown-unknown --release

# Build a scar file
# Copy the packaging directory
COPY contracts/schema/packaging/scar/* \
     /build/contracts/schema/packaging/scar/

# Copy the contract to the packaging directory
RUN cp /build/target/wasm32-unknown-unknown/release/dgc-platform-schema-tp.wasm \
    packaging/scar

WORKDIR /build/contracts/schema/packaging/scar

# Create .scar file
RUN tar -jcvf /tmp/dgc-platform-schema_${REPO_VERSION}.scar .

# -------------=== schema submitter build ===-------------

FROM hyperledger/sawtooth-sabre-cli

COPY --from=dgc-platform-schema-builder \
     /build/target/wasm32-unknown-unknown/release/dgc-platform-schema-tp.wasm /tmp

# Copy the contract definition
COPY contracts/schema/schema.yaml /tmp

COPY --from=dgc-platform-schema-builder \
    /tmp/dgc-platform-schema*.scar /tmp
