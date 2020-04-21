// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

window.$CANOPY.registerConfigSapling('profile', () => {
  console.log('Registering Profile Sapling');

  if (window.location.pathname === '/profile') {
    window.$CANOPY.registerApp(function(domNode) {
      console.log('Rendering Profile JS App');
      domNode.innerHTML = `<h1>Profile<h1>`;
    });
  }
});
