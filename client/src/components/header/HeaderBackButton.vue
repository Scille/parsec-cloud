<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="back-button-container">
    <ion-button
      fill="clear"
      @click="goBack()"
      class="back-button"
    >
      <ion-icon :icon="chevronBack" />

      <ion-label
        v-if="!short"
        class="back-button__label"
      >
        {{ $t('HeaderPage.previous') }}
      </ion-label>
    </ion-button>

    <div
      v-if="short"
      class="vertical-spacer"
    />
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { chevronBack } from 'ionicons/icons';
import { IonLabel, IonIcon, IonButton } from '@ionic/vue';

defineProps<{
  short: boolean;
}>();

const router = useRouter();

function goBack(): void {
  router.go(-1);
}
</script>

<style scoped lang="scss">
.back-button {
  margin-inline: 0px;
  margin-top: 0px;
  margin-bottom: 0px;

  &::part(native) {
    padding: 0.5rem;
  }

  &__label {
    margin-left: 0.625rem;
  }
}

.back-button-container {
  display: flex;
  align-items: center;
}

.vertical-spacer {
  display: block;
  border-right: 1px solid var(--parsec-color-light-secondary-light);
  height: 1.5rem;
  margin: 0 1rem;
}
</style>
