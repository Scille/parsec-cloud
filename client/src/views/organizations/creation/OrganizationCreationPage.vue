<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="creation-page page-modal-container">
    <ms-image
      :image="LogoIconGradient"
      class="logo"
    />
    <ms-spinner title="CreateOrganization.loading" />
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage } from '@ionic/vue';
import { MsImage, MsSpinner, LogoIconGradient } from 'megashark-lib';
</script>

<style scoped lang="scss">
.creation-page {
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 1rem;

  .logo {
    width: 3rem;
  }
}
</style>
