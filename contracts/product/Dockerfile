# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

FROM hyperledger/grid-dev:v1 as dgc-platform-product-builder

COPY contracts/product /build/contracts/product
COPY sdk/ /build/sdk/

# Do a release build to cache dependencies
WORKDIR /build/contracts/product

# Build the contract
ARG REPO_VERSION
RUN sed -i -e "0,/version.*$/ s/version.*$/version\ =\ \"${REPO_VERSION}\"/" Cargo.toml
RUN cargo build --target wasm32-unknown-unknown --release

# Build a scar file
# Copy the packaging directory
COPY contracts/product/packaging/scar/* \
     /build/contracts/product/packaging/scar/

# Copy the contract to the packaging directory
RUN cp /build/target/wasm32-unknown-unknown/release/dgc-platform-product-tp.wasm \
    packaging/scar

WORKDIR /build/contracts/product/packaging/scar

# Create .scar file
RUN tar -jcvf /tmp/dgc-platform-product_${REPO_VERSION}.scar .

# -------------=== product submitter build ===-------------

FROM hyperledger/sawtooth-sabre-cli

COPY --from=dgc-platform-product-builder \
     /build/target/wasm32-unknown-unknown/release/dgc-platform-product-tp.wasm /tmp

# Copy the contract definition
COPY contracts/product/product.yaml /tmp

COPY --from=dgc-platform-product-builder \
    /tmp/dgc-platform-product*.scar /tmp
