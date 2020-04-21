// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import PropTypes from 'prop-types';

import './ProductProperty.scss';

function ProductProperty(props) {
  const { label, value, className } = props;

  return (
    <div className={`product-property ${className}`}>
      <div className="label">{label}</div>
      <div className="value">{value}</div>
    </div>
  );
}

ProductProperty.defaultProps = {
  className: ''
};

ProductProperty.propTypes = {
  className: PropTypes.string,
  label: PropTypes.string.isRequired,
  value: PropTypes.string.isRequired
};

export default ProductProperty;
