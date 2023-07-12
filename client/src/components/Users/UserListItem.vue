<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item>
    {{ user }}
  </ion-item>
</template>

<script setup lang="ts">
import {
  IonItem
} from '@ionic/vue';

defineProps<{
  user: string
}>();
</script>

<style scoped lang="scss">

</style>
