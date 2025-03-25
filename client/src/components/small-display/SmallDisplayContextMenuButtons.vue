<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="buttons">
    <div
      v-if="leftButton"
      class="button button-left"
      @click="leftButton.callback"
    >
      <ion-icon
        class="button-icon"
        :icon="leftButton.icon"
      />
      <ion-text class="button-large">
        {{ $msTranslate(leftButton.text) }}
      </ion-text>
    </div>
    <div
      class="button button-right"
      @click="rightButton.callback"
    >
      <ion-icon
        class="button-icon"
        :icon="rightButton.icon"
      />
      <ion-text class="button-large">
        {{ $msTranslate(rightButton.text) }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonIcon, IonText } from '@ionic/vue';

interface buttonData {
  icon: string;
  text: string;
  callback: () => Promise<boolean>;
}

defineProps<{
  leftButton?: buttonData;
  rightButton: buttonData;
}>();
</script>

<style lang="scss" scoped>
.buttons {
  display: flex;
  flex-direction: row;
  gap: 1rem;
  padding: 3rem 1.5rem 1.5rem;
  justify-content: center;
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);

  .button {
    width: 100%;
    display: flex;
    flex-direction: column;
    padding: 0.75rem 1rem;
    gap: 0.375rem;
    border-radius: var(--parsec-radius-8);

    &-icon {
      font-size: 1.125rem;
    }

    &:hover {
      cursor: pointer;
      color: var(--parsec-color-light-primary-600);
      background-color: var(--parsec-color-light-primary-100);
    }

    &-left {
      color: var(--parsec-color-light-primary-600);
      background-color: var(--parsec-color-light-primary-50);
    }

    &-right {
      color: var(--parsec-color-light-secondary-hard-grey);
      background-color: var(--parsec-color-light-secondary-background);
    }
  }
}
</style>
