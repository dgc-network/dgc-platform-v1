// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

import { getSharedConfig } from 'splinter-saplingjs';
import { get } from './requests';

const { splinterURL } = getSharedConfig().canopyConfig;

const getNodeID = async () => {
  const result = await get(`${splinterURL}/status`);

  if (result.ok) {
    return result.json.node_id;
  }
  throw Error(result.data);
};

export const listScabbardServices = async () => {
  const nodeID = await getNodeID();
  const result = await get(`${splinterURL}/admin/circuits`);
  const services = [];

  if (result.ok) {
    result.json.data.forEach(circuit => {
      circuit.roster.forEach(service => {
        if (
          service.service_type === 'scabbard' &&
          service.allowed_nodes.includes(nodeID)
        ) {
          services.push(`${circuit.id}::${service.service_id}`);
        }
      });
    });
  }
  return services;
};
