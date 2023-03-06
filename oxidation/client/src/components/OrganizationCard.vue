<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="organization-card">
    <ion-grid>
      <ion-row class="ion-align-items-center">
        <ion-col size="auto">
          <ion-avatar>
            <span>{{ device.organizationId?.substring(0, 2) }}</span>
          </ion-avatar>
        </ion-col>
        <ion-col
          size="auto"
          class="organization-info"
        >
          <p class="organization-label">
            {{ device.organizationId }}
          </p>
          <p>
            {{ device.humanHandle }}
          </p>
        </ion-col>
      </ion-row>
    </ion-grid>
  </div>
</template>

<script setup lang="ts">
import { IonGrid, IonCol, IonRow, IonAvatar } from '@ionic/vue';
import { AvailableDevice } from '../plugins/libparsec/definitions';

defineProps<{
  device: AvailableDevice
}>();

</script>

<style lang="scss" scoped>
.organization-card {
  user-select: none;

  ion-avatar {
    background: white;
    color: var(--parsec-color-light-primary-600);
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 2em;
    text-transform: uppercase;
  }

  .organization-info {
    text-align: left;

    p {
      margin: 0px;
      margin-bottom: 2px;
    }

    .organization-label {
      color: var(--parsec-color-light-primary-700);
      font-size: 1.5em;
    }
  }
}

</style>
