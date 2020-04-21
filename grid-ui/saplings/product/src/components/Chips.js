// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import PropTypes from 'prop-types';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import './Chips.scss';

export function Chips({ children }) {
  return <div className="chips">{children}</div>;
}

export function Chip({ label, removeFn, data, deleteable }) {
  return (
    <div className="chip-group">
      <div className="chip">
        <span className="label">{label}</span>
        {deleteable && (
          <FontAwesomeIcon icon="times" className="delete" onClick={removeFn} />
        )}
      </div>
      <div className="chip-data">{data}</div>
    </div>
  );
}

Chips.propTypes = {
  children: PropTypes.node
};

Chips.defaultProps = {
  children: undefined
};

Chip.propTypes = {
  label: PropTypes.string,
  removeFn: PropTypes.func,
  data: PropTypes.oneOfType([PropTypes.string, PropTypes.object]),
  deleteable: PropTypes.bool
};

Chip.defaultProps = {
  label: '',
  removeFn: undefined,
  data: undefined,
  deleteable: false
};
