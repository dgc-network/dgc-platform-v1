// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import PropTypes from 'prop-types';
import classnames from 'classnames';
import './OverlayModal.scss';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faTimes } from '@fortawesome/free-solid-svg-icons';

export function OverlayModal({ open, closeFn, children }) {
  return (
    <div className={classnames('overlay-modal', 'modal', open && 'open')}>
      <FontAwesomeIcon
        icon={faTimes}
        className="close"
        onClick={closeFn}
        tabIndex="0"
      />
      <div className="content">{children}</div>
    </div>
  );
}

OverlayModal.defaultProps = {
  open: false,
  children: undefined
};

OverlayModal.propTypes = {
  open: PropTypes.bool,
  closeFn: PropTypes.func.isRequired,
  children: PropTypes.node
};
