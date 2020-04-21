// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import PropTypes from 'prop-types';

import './NotFound.scss';

function NotFound(props) {
  const { message } = props;

  return (
    <div className="not-found-wrapper">
      <h3 className="not-found-message">{message}</h3>
    </div>
  );
}

NotFound.propTypes = {
  message: PropTypes.string.isRequired
};

export default NotFound;
