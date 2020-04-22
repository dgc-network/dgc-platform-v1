# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

FROM node:lts-alpine as build-stage

RUN apk update && apk add git

RUN npm config set unsafe-perm true

COPY . .

RUN cd /saplings/register-login && \
  npm config set unsafe-perm true && \
  npm install && \
  yarn deploy

RUN cd /saplings/product && \
  npm install && \
  yarn deploy

RUN cd /saplings/profile && \
  npm install && \
  yarn deploy

FROM httpd:2.4 as prod-stage

RUN echo "\
  \n\
  Header set Access-Control-Allow-Origin "*"\n\
  \n\
  " >>/usr/local/apache2/conf/httpd.conf

COPY --from=build-stage /sapling-dev-server/ /usr/local/apache2/htdocs/

EXPOSE 80/tcp
