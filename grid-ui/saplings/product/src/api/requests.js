// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

function errorResponse(request, message) {
  return {
    ok: false,
    status: request.status,
    statusText: request.statusText,
    headers: request.getAllResponseHeaders(),
    data: message || request.responseText,
    json: JSON.parse(message || request.responseText)
  };
}

export function get(url) {
  return new Promise(resolve => {
    const request = new XMLHttpRequest();
    request.open('GET', url, true);
    request.timeout = 5000;

    request.onload = () => {
      return resolve({
        ok: request.status >= 200 && request.status < 300,
        status: request.status,
        statusText: request.statusText,
        headers: request.getAllResponseHeaders(),
        data: request.responseText,
        json: JSON.parse(request.responseText)
      });
    };

    request.onError = () => {
      resolve(errorResponse());
    };

    request.ontimeout = () => {
      resolve(errorResponse(request, 'Request took longer than expected.'));
    };

    request.send();
  });
}
