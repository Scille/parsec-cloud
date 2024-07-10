<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="creation-page">
    <ms-image :image="LogoIconGradient" />
    <ms-spinner title="CreateOrganization.loading" />
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage } from '@ionic/vue';
import { MsImage, MsSpinner, LogoIconGradient } from 'megashark-lib';
</script>

<style scoped lang="scss">
.creation-page {
  padding: 2.5rem;
  display: flex;
  height: auto;
  width: 100%;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 1rem;
}
</style>
