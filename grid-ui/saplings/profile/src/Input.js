// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import proptypes from 'prop-types';
import './Input.scss';

export function Input({ type, id, label, name, value, onChange, required }) {
  return (
    <div className="dgc-platform-input">
      <input
        type={type}
        id={id}
        aria-label={`${label} field`}
        placeholder={`${label}`}
        name={name}
        value={value}
        onChange={onChange}
        required={required}
      />
      <label htmlFor={id}>{label}</label>
    </div>
  );
}

Input.propTypes = {
  type: proptypes.string,
  id: proptypes.string,
  label: proptypes.string,
  name: proptypes.string,
  value: proptypes.any,
  onChange: proptypes.func.isRequired,
  required: proptypes.bool
};

Input.defaultProps = {
  type: 'text',
  id: undefined,
  label: undefined,
  name: undefined,
  value: null,
  required: false
};
