// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React, { useState } from 'react';
import proptypes from 'prop-types';
import './ActionList.scss';

export function ActionList({ children }) {
  const [open, setOpen] = useState(0);
  return (
    <div className="action-list">
      <button
        className={`flat action-button${open ? ' open' : ''}`}
        onClick={() => setOpen(!open)}
      >
        Actions
        <div className="hamburger">
          <div className="top" />
          <div className="middle" />
          <div className="bottom" />
        </div>
      </button>
      <div className={`actions${open ? ' open' : ''}`}>{children}</div>
    </div>
  );
}

ActionList.propTypes = {
  children: proptypes.node.isRequired
};
