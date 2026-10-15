<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="sso-provider"
    :class="{ connected: isConnected }"
    @click="$emit('ssoSelected', provider)"
  >
    <img
      v-if="provider.icon"
      :src="provider.icon"
      class="sso-icon"
    />
    <span>
      {{ provider.name }}
    </span>
    <div
      class="connected"
      v-if="isConnected"
    >
      <ion-icon
        :icon="checkmarkCircle"
        class="connected-icon"
      />
      <ion-text class="button-large connected-text">
        {{ $msTranslate('Authentication.method.sso.connected') }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { AvailableSSO } from '@/parsec';
import { IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';

defineEmits<{
  (e: 'ssoSelected', provider: AvailableSSO): void;
}>();

defineProps<{
  provider: AvailableSSO;
  isConnected?: boolean;
}>();
</script>

<style scoped lang="scss">
.sso-provider {
  border: 2px solid black;
  border-radius: 5;
  margin-bottom: 5px;
  padding: 10px;

  .sso-icon {
    width: 24px;
    height: 24px;
  }
}

.connected {
  border-color: green;
}
</style>
