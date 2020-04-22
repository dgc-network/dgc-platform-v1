// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

/**
 * Wrapper function to set up XHR.
 * @param {string}      method    HTTP method for the request
 * @param {string}      url       endpoint to make the request to
 * @param {object}      data      Byte array representation of the request body
 * @param {function}    headerFn  Function to set the correct request headers
 */
export async function http(method, url, data, headerFn) {
  return new Promise((resolve, reject) => {
    const request = new XMLHttpRequest();
    request.open(method, url);
    if (headerFn) {
      headerFn(request);
    }
    request.onload = () => {
      if (request.status >= 200 && request.status < 300) {
        resolve(request.response);
      } else {
        reject(request.response);
      }
    };
    request.onerror = () => {
      reject(
        Error(
          'The server has encountered an error. Please contact the administrator.'
        )
      );
    };
    request.send(data);
  });
}
