# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

# Dockerfile for running unit tests and lint on the dgc-platform UI
FROM node:lts-alpine

WORKDIR /dgc-platform-ui

COPY package*.json ./

RUN apk add git

# Gives npm permission to run the prepare script in splinter-canopyjs as root
RUN npm config set unsafe-perm true && npm install

COPY . .
