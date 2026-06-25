<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <span />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';

onMounted(async () => {
  // This component is just used for the redirection from the SSO,
  // no need to display anything since it's closed as soon as we
  // get the params.
  const params = new URLSearchParams(location.search);
  const bc = new BroadcastChannel('openbao-oidc');
  if (params.has('code') && params.has('state')) {
    bc.postMessage({ ok: true, value: { code: params.get('code'), state: params.get('state') } });
  } else {
    bc.postMessage({ ok: false, error: { error: params.get('error') ?? 'unknown', description: params.get('error_description') } });
  }
  bc.close();
  window.close();
});
</script>

<style scoped lang="scss"></style>
