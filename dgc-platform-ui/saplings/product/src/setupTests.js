// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import '@testing-library/jest-dom/extend-expect';

window.$CANOPY = {};

window.$CANOPY.getSharedConfig = () => {
  return {
    canopyConfig: {
      splinterURL: 'testSplinterURL',
      saplingURL: 'testSaplingURL'
    },
    appConfig: {
      gridURL: 'testGridURL'
    }
  };
};
