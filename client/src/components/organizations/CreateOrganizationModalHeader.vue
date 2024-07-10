<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-buttons
    slot="end"
    class="closeBtn-container"
  >
    <ion-button
      slot="icon-only"
      class="closeBtn"
      v-show="!hideCloseButton"
      @click="$emit('closeClicked')"
    >
      <ion-icon
        :icon="close"
        size="large"
        class="closeBtn__icon"
      />
    </ion-button>
  </ion-buttons>
  <ion-header class="modal-header">
    <ion-title class="modal-header__title title-h2">
      {{ $msTranslate(title) }}
    </ion-title>
  </ion-header>
</template>

<script setup lang="ts">
import { IonButton, IonButtons, IonHeader, IonIcon, IonTitle } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';

defineProps<{
  title: Translatable;
  hideCloseButton?: boolean;
}>();

defineEmits<{
  (e: 'closeClicked'): void;
}>();
</script>

<style scoped lang="scss"></style>
