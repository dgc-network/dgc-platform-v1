// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import React from 'react';

import CircuitDropdown from './CircuitDropdown';
import './FilterBar.scss';

function FilterBar() {
  return (
    <div className="filter-bar">
      <CircuitDropdown />
    </div>
  );
}

export default FilterBar;
