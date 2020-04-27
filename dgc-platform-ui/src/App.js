// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import { library } from '@fortawesome/fontawesome-svg-core';
import { faLeaf, faUserCircle } from '@fortawesome/free-solid-svg-icons';
import { CanopyProvider, SideNav } from 'splinter-canopyjs';
import './App.scss';

window.$CANOPY = {};

library.add(faLeaf);
library.add(faUserCircle);

function AppWithProvider() {
  return (
    <CanopyProvider
      saplingURL={process.env.REACT_APP_SAPLING_URL}
      splinterURL={process.env.REACT_APP_SPLINTER_URL}
      appConfig={{ gridURL: process.env.REACT_APP_DGC_PLATFORM_URL }}
    >
      <SideNav />
    </CanopyProvider>
  );
}
export default AppWithProvider;
