// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

export const getProperty = (name, propertyList) => {
  const property = propertyList.find(p => p.name === name);
  if (property === undefined) {
    return null;
  }

  switch (property.data_type.toLowerCase()) {
    case 'string':
      return property.string_value;
    case 'number':
      return property.number_value;
    default:
      throw Error(`unsupported property type: ${property.data_type}`);
  }
};
