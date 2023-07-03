<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-list>
    <ion-item>
      <ion-icon
        :icon="caretForward"
        slot="start"
      />
      {{ $t('JoinOrganization.instructions.finish.first') }}
    </ion-item>
    <ion-item>
      <ion-icon
        :icon="caretForward"
        slot="start"
      />
      {{ $t('JoinOrganization.instructions.finish.second') }}
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import {
  caretForward
} from 'ionicons/icons';
import { IonList, IonItem, IonIcon } from '@ionic/vue';
</script>

<style scoped lang="scss">
ion-icon {
  color: var(--parsec-color-light-primary-400);
}
</style>
