// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import { useServiceState } from '../state/service-context';
import ProductCard from './ProductCard';
import NotFound from './NotFound';
import Loading from './Loading';
import { getProperty } from '../data/property-parsing';
import { listProducts } from '../api/dgc-platform';
import './ProductsTable.scss';

function ProductsTable({ actions }) {
  const [products, setProducts] = useState([]);
  const { selectedService } = useServiceState();
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const getProducts = async () => {
      if (selectedService !== 'none') {
        setLoading(true);
        try {
          const productList = await listProducts(selectedService);
          setProducts(productList);
        } catch (e) {
          console.error(`Error listing products: ${e}`);
        }
      } else {
        setProducts([]);
      }
      setLoading(false);
    };

    getProducts();
  }, [selectedService]);

  const productCards = products.map(product => {
    return (
      <ProductCard
        key={product.product_id}
        gtin={product.product_id}
        name={getProperty('product_name', product.properties)}
        owner={product.owner}
        imageURL={getProperty('image_url', product.properties)}
      />
    );
  });

  const getContent = () => {
    if (loading) {
      return <Loading />;
    }
    if (products.length === 0) {
      return <NotFound message="No Products" />;
    }
    return <div className="products-table">{productCards}</div>;
  };

  return (
    <div className="products-table-container">
      <div className="products-table-header">
        <h5 className="title">Products</h5>
        <hr />
      </div>
      {getContent()}
      <button
        className="fab add-product"
        type="button"
        onClick={actions.addProduct}
      >
        <FontAwesomeIcon icon="plus" />
      </button>
    </div>
  );
}

ProductsTable.propTypes = {
  actions: PropTypes.object.isRequired
};

export default ProductsTable;
