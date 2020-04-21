// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import { Link } from 'react-router-dom';
import PropTypes from 'prop-types';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import ProductProperty from './ProductProperty';
import './ProductCard.scss';

function ProductCard(props) {
  const { gtin, name, owner, imageURL } = props;
  return (
    <div className="product-card">
      <button type="button" className="product-card-edit-button">
        <FontAwesomeIcon className="icon" icon="pen-square" />
      </button>
      <Link className="link" to={`/product/products/${gtin}`}>
        <div className="product-card-content">
          <div className="product-card-properties">
            <ProductProperty label="GTIN" value={gtin} />
            <ProductProperty label="Product Name" value={name} />
            <ProductProperty label="Owner" value={owner} />
          </div>
          {imageURL && (
            <img className="product-card-image" src={imageURL} alt={name} />
          )}
        </div>
      </Link>
    </div>
  );
}

ProductCard.propTypes = {
  gtin: PropTypes.string.isRequired,
  name: PropTypes.string.isRequired,
  owner: PropTypes.string.isRequired,
  imageURL: PropTypes.string
};

ProductCard.defaultProps = {
  imageURL: null
};

export default ProductCard;
