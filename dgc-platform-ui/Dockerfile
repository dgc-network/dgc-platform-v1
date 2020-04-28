# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

# build stage
FROM node:lts-alpine as build-stage

RUN apk update && apk add python g++ git make && rm -rf /var/cache/apk/*
WORKDIR /dgc_platform_ui
COPY package*.json ./
RUN npm config set unsafe-perm true && npm install
COPY . .
ARG REACT_APP_SPLINTER_URL
ENV REACT_APP_SPLINTER_URL $REACT_APP_SPLINTER_URL
ARG REACT_APP_SAPLING_URL
ENV REACT_APP_SAPLING_URL $REACT_APP_SAPLING_URL
ARG REACT_APP_DGC_PLATFORM_URL
ENV REACT_APP_DGC_PLATFORM_URL $REACT_APP_DGC_PLATFORM_URL
RUN yarn build
WORKDIR /dgc_platform_ui/build
ARG REPO_VERSION
RUN tar c -z . -f ../dgc_platform_ui_v${REPO_VERSION}.tar.gz

# prod stage
FROM httpd:2.4 as prod-stage

ARG SPLINTER_URL
ARG SAPLING_URL
ARG DGC_PLATFORM_URL

COPY --from=build-stage /dgc_platform_ui/dgc_platform_ui_*.tar.gz /tmp
RUN tar -xzvf /tmp/dgc_platform_ui_*.tar.gz -C /usr/local/apache2/htdocs/
COPY /configs/apache/httpd.conf /usr/local/apache2/conf/httpd.conf

RUN echo "\
  \n\
  ProxyPass /splinterd $SPLINTER_URL\n\
  ProxyPassReverse /splinterd $SPLINTER_URL\n\
  ProxyPass /sapling-dev-server $SAPLING_URL\n\
  ProxyPassReverse /sapling-dev-server $SAPLING_URL\n\
  ProxyPass /dgc-platform-daemon $DGC_PLATFORM_URL\n\
  ProxyPassReverse /dgc-platform-daemon $DGC_PLATFORM_URL\n\
  \n\
  " >>/usr/local/apache2/conf/httpd.conf

EXPOSE 80/tcp
