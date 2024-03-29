// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';
import { useParams, Link } from 'react-router-dom';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import { useServiceState } from '../state/service-context';
import { getProperty } from '../data/property-parsing';
import ProductProperty from './ProductProperty';
import { fetchProduct } from '../api/dgc-platform';
import NotFound from './NotFound';
import Loading from './Loading';
import './ProductInfo.scss';

function ProductInfo() {
  const { id } = useParams();
  const [product, setProduct] = useState({ properties: [] });
  const { selectedService } = useServiceState();
  const [loading, setLoading] = useState(false);
  const [notFound, setNotFound] = useState('');

  useEffect(() => {
    const getProduct = async () => {
      if (selectedService === 'none') {
        setNotFound('Select a service');
      } else {
        setLoading(true);
        try {
          const productResponse = await fetchProduct(selectedService, id);
          setProduct(productResponse);
          setNotFound('');
        } catch (e) {
          console.error(`Error fetching product: ${e}`);
          setNotFound('Product not found on this service');
          setProduct({ properties: [] });
        }
      }
      setLoading(false);
    };

    getProduct();
  }, [selectedService, id]);

  const imageURL = getProperty('image_url', product.properties);

  const getContent = () => {
    if (loading) {
      return <Loading />;
    }
    if (notFound) {
      return <NotFound message={notFound} />;
    }
    return (
      <>
        {imageURL && <img src={imageURL} alt="" className="product-image" />}
        <ProductOverview
          gtin={product.product_id || 'Unknown'}
          productName={
            getProperty('product_name', product.properties) || 'Unknown'
          }
          owner={product.owner || 'Unknown'}
        />
        <ProductProperties propertiesList={product.properties} />
      </>
    );
  };

  return <div className="product-info-container">{getContent()}</div>;
}

function ProductOverview(props) {
  const { gtin, productName, owner } = props;

  return (
    <div className="product-overview-container">
      <Link className="back-link" to="/product">
        <FontAwesomeIcon icon="chevron-left" />
        <span className="back-link-text">Back</span>
      </Link>
      <ProductProperty className="large light" label="GTIN" value={gtin} />
      <ProductProperty
        className="large light"
        label="Product Name"
        value={productName}
      />
      <ProductProperty className="large light" label="Owner" value={owner} />
    </div>
  );
}

ProductOverview.propTypes = {
  gtin: PropTypes.string.isRequired,
  productName: PropTypes.string.isRequired,
  owner: PropTypes.string.isRequired
};

function ProductProperties(props) {
  const { propertiesList } = props;

  const primaryProperties = [
    { name: 'brand_name', data_type: 'STRING', label: 'Brand Name' },
    {
      name: 'product_description',
      data_type: 'STRING',
      label: 'Product Description'
    },
    { name: 'gpc', label: 'GPC' },
    { name: 'net_content', label: 'Net Content' },
    { name: 'target_market', label: 'Target Market' }
  ];

  const productProperties = primaryProperties.map(property => {
    const propertyValue = getProperty(property.name, propertiesList);

    if (propertyValue) {
      return (
        <ProductProperty
          className="large"
          label={property.label}
          value={propertyValue}
        />
      );
    }
    return <></>;
  });

  return (
    <div className="product-properties-container">
      <div className="product-properties-header">
        <h5 className="title">Product Info</h5>
        <hr />
      </div>
      <div className="product-properties-list">{productProperties}</div>
      <button type="button" className="full-info-button">
        VIEW FULL PRODUCT INFO
      </button>
    </div>
  );
}

ProductProperties.propTypes = {
  propertiesList: PropTypes.arrayOf(PropTypes.object).isRequired
};

export default ProductInfo;
