<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="container">
    <ms-informative-text
      :icon="caretForward"
      :text="$t('JoinOrganization.instructions.finish.first')"
    />
    <ms-informative-text
      :icon="caretForward"
      :text="$t('JoinOrganization.instructions.finish.second')"
    />
  </div>
</template>

<script setup lang="ts">
import {
  caretForward,
} from 'ionicons/icons';
import MsInformativeText from '@/components/core/ms-text/MsInformativeText.vue';
</script>

<style scoped lang="scss">
.container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}
</style>
