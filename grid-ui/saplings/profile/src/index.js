// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import { registerApp, registerConfigSapling } from 'splinter-saplingjs';
import App from './App';

registerConfigSapling('profile', () => {
  if (window.location.pathname === '/profile') {
    registerApp(domNode => {
      ReactDOM.render(<App />, domNode);
    });
  }
});
