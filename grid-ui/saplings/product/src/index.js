// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import ReactDOM from 'react-dom';
import { registerApp } from 'splinter-saplingjs';

import './index.css';
import App from './App';

registerApp(domNode => {
  ReactDOM.render(<App />, domNode);
});
