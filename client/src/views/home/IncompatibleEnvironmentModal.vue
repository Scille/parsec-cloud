<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal incompatible-environment">
    <ms-modal
      :theme="MsReportTheme.Error"
      :close-button="{ visible: false }"
    >
      <div class="incompatible-image-container">
        <!-- prettier-ignore -->
        <ms-image
          :image="(ResourcesManager.instance().get(Resources.LogoIcon, IncompatibleIcon) as string)"
          class="incompatible-image"
        />
      </div>
      <div class="incompatible-content">
        <ion-text class="incompatible-content__title title-h2">
          {{ $msTranslate('globalErrors.incompatibleEnvironment') }}
        </ion-text>
        <ion-text class="incompatible-content__description body-lg">
          {{ $msTranslate(message) }}
        </ion-text>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import IncompatibleIcon from '@/assets/images/logo-incompatible.svg?raw';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonPage, IonText } from '@ionic/vue';
import { MsImage, MsModal, MsReportTheme, Translatable } from 'megashark-lib';

defineProps<{
  message: Translatable;
}>();
</script>

<style scoped lang="scss">
.incompatible-image-container {
  display: flex;
  justify-content: center;
  max-width: 7rem;
  width: 100%;
  height: auto;
  align-self: center;
}
.incompatible-image {
  width: 100%;
  height: 100%;
}
.incompatible-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  text-align: center;

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &__description {
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>
