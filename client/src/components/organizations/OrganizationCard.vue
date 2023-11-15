<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-card class="organization-card__body">
    <ion-card-header class="card-content">
      <div class="organization-info">
        <ms-avatar
          class="orga-avatar body-lg"
          :type="AvatarType.Icon"
          :data="device.organizationId"
        />

        <div class="orga-text">
          <ion-card-title class="card-title subtitles-normal">
            {{ device.organizationId }}
          </ion-card-title>
        </div>
      </div>
    </ion-card-header>
    <ion-card-content>
      {{ device.humanHandle.label }}
    </ion-card-content>
  </ion-card>
</template>

<script setup lang="ts">
import { IonCard, IonCardHeader, IonCardTitle, IonCardContent } from '@ionic/vue';
import { AvailableDevice } from '@/parsec';
import MsAvatar, { AvatarType } from '@/components/core/ms-avatar/MsAvatar.vue';

defineProps<{
  device: AvailableDevice;
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
    }

    .card-title {
      color: var(--parsec-color-light-primary-700);
    }
  }
}
</style>
