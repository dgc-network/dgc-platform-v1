// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

const rewire = require('rewire');

const defaults = rewire('react-scripts/scripts/build.js');
const config = defaults.__get__('config');

config.optimization.splitChunks = {
  cacheGroups: {
    default: false
  }
};

config.optimization.runtimeChunk = false;

// JS
config.output.filename = 'static/js/profile.js';
// CSS. "5" is MiniCssPlugin
config.plugins[5].options.moduleFilename = () => 'static/css/profile.css';
