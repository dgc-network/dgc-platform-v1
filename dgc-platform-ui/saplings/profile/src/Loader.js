// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import classnames from 'classnames';
import './Loader.scss';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCheck, faTimes } from '@fortawesome/free-solid-svg-icons';
import proptypes from 'prop-types';

export function Loader({ state }) {
  return (
    <div className={classnames('loader', state && 'complete')}>
      <div className="dots">
        <div className="dot" />
        <div className="dot" />
        <div className="dot" />
      </div>
      <div
        className={classnames(
          'state',
          state === 'success' && 'success',
          state === 'failure' && 'failure'
        )}
      >
        <FontAwesomeIcon icon={state === 'success' ? faCheck : faTimes} />
      </div>
    </div>
  );
}

Loader.propTypes = {
  state: proptypes.oneOf(['success', 'failure'])
};

Loader.defaultProps = {
  state: undefined
};
