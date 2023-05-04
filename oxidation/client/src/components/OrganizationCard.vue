<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-card class="organization-card__body">
    <ion-card-header class="card-content">
      <div class="organization-info">
        <ion-avatar class="orga-avatar body-lg">
          <span>{{ device.organizationId?.substring(0, 2) }}</span>
        </ion-avatar>
        <!-- txt -->
        <div class="orga-text">
          <ion-card-title class="card-title subtitles-normal">
            {{ device.organizationId }}
          </ion-card-title>
          <ion-card-subtitle>
            <p class="subtitles-sm">
              {{ device.humanHandle }}
            </p>
          </ion-card-subtitle>
        </div>
      </div>
    </ion-card-header>
  </ion-card>
</template>

<script setup lang="ts">
import { IonGrid, IonCol, IonRow, IonAvatar } from '@ionic/vue';
import { AvailableDevice } from '../plugins/libparsec/definitions';

defineProps<{
  device: AvailableDevice
}>();

</script>

<style lang="scss" scoped>
.organization-card__body {
  user-select: none;
  box-shadow: none;
  background: none;
  padding: 1.5rem 1rem;
  margin: 0;

  .card-content {
    padding: 0px;
  }

  .organization-info {
    display: flex;
    flex-direction: row;
    gap: .5rem;

    .orga-avatar {
      background-color: white;
      color: var(--parsec-color-light-primary-600);
      width: 52px;
      height: 52px;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      position: relative;
      z-index: 1;
    }
    .card-title{
      color: var(--parsec-color-light-primary-700);
    }

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
