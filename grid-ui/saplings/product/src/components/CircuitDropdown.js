// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React, { useState, useRef, useEffect } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import { useServiceState, useServiceDispatch } from '../state/service-context';
import useOnClickOutside from '../hooks/on-click-outside';
import { listScabbardServices } from '../api/splinter';
import './CircuitDropdown.scss';

const CircuitDropdown = () => {
  const { services, selectedService } = useServiceState();
  const serviceDispatch = useServiceDispatch();
  const [listOpen, setListOpen] = useState(false);
  const [headerText, setHeaderText] = useState();

  const caretUp = <FontAwesomeIcon icon="caret-up" />;
  const caretDown = <FontAwesomeIcon icon="caret-down" />;

  const toggleDropdown = () => {
    if (listOpen || services.length > 0) {
      setListOpen(!listOpen);
    }
  };

  const handleSelect = serviceID => {
    setListOpen(false);
    serviceDispatch({
      type: 'select',
      payload: {
        serviceID
      }
    });
  };

  const handleSelectNone = () => {
    setListOpen(false);
    serviceDispatch({
      type: 'selectNone'
    });
  };

  const listItems = services.map(serviceID => (
    <div
      className="dd-list-item"
      role="button"
      tabIndex="0"
      onClick={() => handleSelect(serviceID)}
      onKeyPress={() => handleSelect(serviceID)}
    >
      {serviceID}
      {serviceID === selectedService && <FontAwesomeIcon icon="check" />}
    </div>
  ));

  const ref = useRef();
  useOnClickOutside(ref, () => setListOpen(false));

  useEffect(() => {
    if (services.length > 0) {
      if (selectedService === 'none') {
        setHeaderText('Select a service');
      } else {
        setHeaderText(selectedService);
      }
    } else {
      setHeaderText('No services available');
    }
  }, [selectedService, services]);

  useEffect(() => {
    const getServices = async () => {
      try {
        const servicesList = await listScabbardServices();
        serviceDispatch({
          type: 'setServices',
          payload: {
            services: servicesList
          }
        });
      } catch (e) {
        console.error(`Error listing services: ${e}`);
      }
    };

    getServices();
  }, [serviceDispatch]);

  return (
    <div className="dd-wrapper" ref={ref}>
      <div
        className={`dd-header ${services.length === 0 && 'disabled'}`}
        role="button"
        tabIndex="0"
        onClick={() => toggleDropdown(!listOpen)}
        onKeyPress={() => toggleDropdown(!listOpen)}
      >
        <div className="dd-header-text">{headerText}</div>
        {listOpen ? caretUp : caretDown}
      </div>
      {listOpen && (
        <ul className="dd-list">
          <div
            className="dd-list-item"
            role="button"
            tabIndex="0"
            onClick={handleSelectNone}
            onKeyPress={handleSelectNone}
          >
            None
            {selectedService === 'none' && <FontAwesomeIcon icon="check" />}
          </div>
          {listItems}
        </ul>
      )}
    </div>
  );
};

export default CircuitDropdown;
