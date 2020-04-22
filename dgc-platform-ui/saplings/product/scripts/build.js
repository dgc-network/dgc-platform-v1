// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

const rewire = require('rewire');

const defaults = rewire('react-scripts/scripts/build.js');
// eslint-disable-next-line no-underscore-dangle
const config = defaults.__get__('config');

config.optimization.splitChunks = {
  cacheGroups: {
    default: false
  }
};

config.optimization.runtimeChunk = false;

// JS
config.output.filename = 'static/js/product.js';
// CSS. "5" is MiniCssPlugin
config.plugins[5].options.moduleFilename = () => 'static/css/product.css';
