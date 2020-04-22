// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import './Loading.scss';

function Loading() {
  return (
    <div className="spinner-wrapper">
      <FontAwesomeIcon className="spinner" icon="spinner" pulse size="4x" />;
    </div>
  );
}

export default Loading;
