<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="shamir-container">
    <div class="shamir-content">
      <ion-button
        :disabled="tab === ShamirTab.Self"
        @click="tab = ShamirTab.Self"
        fill="clear"
      >
        SELF
      </ion-button>
      <ion-button
        :disabled="tab === ShamirTab.Others"
        @click="tab = ShamirTab.Others"
        fill="clear"
      >
        OTHERS
      </ion-button>
      <div v-if="tab === ShamirTab.Self">
        <shamir-self />
      </div>
      <div v-if="tab === ShamirTab.Others">
        <shamir-others />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ShamirOthers from '@/components/profile/ShamirOthers.vue';
import ShamirSelf from '@/components/profile/ShamirSelf.vue';
import { IonButton } from '@ionic/vue';
import { ref } from 'vue';

enum ShamirTab {
  Self = 'shamir-tab-self',
  Others = 'shamir-tab-others',
}

const tab = ref<ShamirTab>(ShamirTab.Self);
</script>

<style scoped lang="scss">
.shamir-container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.shamir-content {
  display: flex;
  flex-direction: column;
}
</style>
