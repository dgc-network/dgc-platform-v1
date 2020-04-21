// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

module.exports = {
  mode: 'production',
  entry: './src/index.ts',
  output: {
    library: 'register-login',
    libraryTarget: 'umd',
    filename: 'register-login.js',
    globalObject: 'this'
  },
  resolve: {
    extensions: ['.ts', '.js']
  },
  module: {
    rules: [
      { test: /\.ts$/, loader: 'ts-loader' },
      {
        test: /\.html$/i,
        loader: 'html-loader'
      }
    ]
  }
};
