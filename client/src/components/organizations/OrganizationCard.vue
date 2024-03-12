<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-card class="organization-card__body">
    <ion-card-header class="card-content">
      <div class="organization-info">
        <ion-avatar class="orga-avatar body-lg">
          <span>{{ device.organizationId?.substring(0, 2) }}</span>
        </ion-avatar>
        <div class="orga-text">
          <ion-card-title class="card-title">
            <span class="title-h4">{{ device.organizationId }}</span>
            <span class="subtitles-sm">{{ device.humanHandle.label }}</span>
          </ion-card-title>
        </div>
      </div>
    </ion-card-header>
  </ion-card>
</template>

<script setup lang="ts">
import { AvailableDevice } from '@/parsec';
import { IonAvatar, IonCard, IonCardHeader, IonCardTitle } from '@ionic/vue';

defineProps<{
  device: AvailableDevice;
}>();
</script>

<style lang="scss" scoped>
.organization-card__body {
  user-select: none;
  box-shadow: none;
  background: none;
  margin: 0;

  .card-content {
    padding: 0;
  }

  .organization-info {
    display: flex;
    align-items: center;
    flex-direction: row;
    gap: 0.5rem;

    .orga-avatar {
      background-color: white;
      color: var(--parsec-color-light-primary-600);
      width: 3.25rem;
      height: 3.25rem;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      position: relative;
      z-index: 1;
      border: 1px solid var(--parsec-color-light-secondary-medium);
    }

    .card-title {
      display: flex;
      flex-direction: column;
      gap: 0.375rem;

      span:first-child {
        color: var(--parsec-color-light-primary-700);
      }

      span:last-child {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }
}
</style>
