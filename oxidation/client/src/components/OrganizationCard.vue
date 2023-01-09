<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item>
    <ion-card
      class="organization-card"
      v-bind:class="{ 'organization-card-hover': hoverEnabled }"
    >
      <ion-card-content>
        <ion-grid>
          <ion-row class="ion-align-items-center">
            <ion-col size="auto">
              <ion-avatar>
                <span>{{ device.organization_id?.substring(0, 2) }}</span>
              </ion-avatar>
            </ion-col>
            <ion-col size="auto">
              <p class="organization-label">
                {{ device.organization_id }}
              </p>
              <p>
                {{ device.human_handle }}
              </p>
            </ion-col>
          </ion-row>
          <ion-row v-if="lastLogin">
            <ion-col size="auto">
              <p>
                {{ $t('OrganizationCard.lastLogin') }} {{ $d(lastLogin, 'long') }}
              </p>
            </ion-col>
          </ion-row>
        </ion-grid>
      </ion-card-content>
    </ion-card>
  </ion-item>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';
import { IonCard, IonCardContent, IonGrid, IonCol, IonRow, IonItem, IonAvatar } from '@ionic/vue';
import { AvailableDevice } from '../plugins/libparsec/definitions';

defineProps<{
  device: AvailableDevice
  hoverEnabled: boolean
  lastLogin: Date
}>();

</script>

<style lang="scss" scoped>
.organization-card {
  background: #F9F9FB;
  margin: 1em 1.5em;
  ion-avatar {
    background: white;
    color: #0058cc;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 2em;
    text-transform: uppercase;
  }

  .organization-label {
    color: #004299;
    font-size: 1.5em;
  }
}

.organization-card-hover:hover {
  background: #E5F1FF;
  cursor: pointer;
}

</style>
