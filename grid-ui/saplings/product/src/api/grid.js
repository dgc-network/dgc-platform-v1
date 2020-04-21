// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import { getSharedConfig } from 'splinter-saplingjs';

import { get } from './requests';

const { gridURL } = getSharedConfig().appConfig;

export const listProducts = async serviceID => {
  const result = await get(`${gridURL}/product?service_id=${serviceID}`);

  if (result.ok) {
    return result.json;
  }
  throw Error(result.data);
};

export const fetchProduct = async (serviceID, productID) => {
  const result = await get(
    `${gridURL}/product/${productID}?service_id=${serviceID}`
  );

  if (result.ok) {
    return result.json;
  }
  throw Error(result.data);
};
