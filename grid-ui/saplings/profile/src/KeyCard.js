// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import PropTypes from 'prop-types';
import classnames from 'classnames';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faPencilAlt } from '@fortawesome/free-solid-svg-icons';

import './KeyCard.scss';

export function KeyCard({ userKey, isActive, setActiveFn, editFn }) {
  const setActive = () => {
    try {
      setActiveFn();
    } catch (err) {
      return null;
    }
    return null;
  };

  return (
    <div className={classnames('key-card', isActive && 'active')}>
      <div className="header">
        <span className="name">{userKey.display_name}</span>
        <FontAwesomeIcon icon={faPencilAlt} className="edit" onClick={editFn} />
      </div>
      <div className="keys">
        <span className="label">Public key:</span>
        <span className="key" id={`public-key-${userKey.public_key}`}>
          {userKey.public_key}
        </span>
      </div>
      {!isActive && (
        <button className="set-active" onClick={setActive}>
          Set active
        </button>
      )}
      {isActive && <div className="active">Active</div>}
    </div>
  );
}

KeyCard.defaultProps = {
  isActive: false
};

KeyCard.propTypes = {
  userKey: PropTypes.object.isRequired,
  isActive: PropTypes.bool,
  setActiveFn: PropTypes.func.isRequired,
  editFn: PropTypes.func.isRequired
};
