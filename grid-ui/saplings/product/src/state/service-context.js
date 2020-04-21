// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import PropTypes from 'prop-types';

const ServiceStateContext = React.createContext();
const ServiceDispatchContext = React.createContext();

const serviceReducer = (state, action) => {
  switch (action.type) {
    case 'select': {
      return { ...state, selectedService: action.payload.serviceID };
    }
    case 'selectNone': {
      return { ...state, selectedService: 'none' };
    }
    case 'setServices': {
      return { ...state, services: action.payload.services };
    }
    default:
      throw new Error(`unhandled action type: ${action.type}`);
  }
};

function ServiceProvider({ children }) {
  const [state, dispatch] = React.useReducer(serviceReducer, {
    selectedService: 'none',
    services: []
  });

  return (
    <ServiceStateContext.Provider value={state}>
      <ServiceDispatchContext.Provider value={dispatch}>
        {children}
      </ServiceDispatchContext.Provider>
    </ServiceStateContext.Provider>
  );
}

ServiceProvider.propTypes = {
  children: PropTypes.node.isRequired
};

function useServiceState() {
  const context = React.useContext(ServiceStateContext);
  if (context === undefined) {
    throw new Error('useServiceState must be used within a ServiceProvider');
  }
  return context;
}

function useServiceDispatch() {
  const context = React.useContext(ServiceDispatchContext);
  if (context === undefined) {
    throw new Error('useServiceDispatch must be used within a ServiceProvider');
  }
  return context;
}

export { ServiceProvider, useServiceState, useServiceDispatch };
