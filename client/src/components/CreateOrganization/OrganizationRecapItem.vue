<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-label class="recap-item__label">
    {{ title }}
  </ion-label>
  <ion-text class="recap-item__text">
    {{ text }}
  </ion-text>
  <ion-button
    fill="clear"
  >
    {{ $t('CreateOrganization.modify') }}
    <ion-icon
      :icon="pencil"
      fill="none"
      size="small"
    />
  </ion-button>
</template>

<script setup lang="ts">
import {
  IonText,
  IonLabel,
  IonButton,
  IonIcon,
} from '@ionic/vue';
import {
  pencil,
} from 'ionicons/icons';

defineProps<{
  title: string,
  text: string,
}>();
</script>

<style scoped lang="scss">

</style>
