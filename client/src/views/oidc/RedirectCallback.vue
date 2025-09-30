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
  bc.postMessage({ code: params.get('code'), state: params.get('state') });
  bc.close();
  window.close();
});
</script>

<style scoped lang="scss"></style>
