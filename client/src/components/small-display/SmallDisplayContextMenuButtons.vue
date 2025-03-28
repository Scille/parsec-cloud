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
      <ion-text class="button-text">
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
      <ion-text class="button-text">
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
  padding-top: 2.5rem;
  justify-content: center;

  .button {
    min-width: 5rem;
    width: 45%;
    height: 4.4rem;
    border-radius: var(--parsec-radius-6);
    font-weight: 500;

    &:hover {
      cursor: pointer;
      background-color: var(--parsec-color-light-primary-100);
    }

    &-icon {
      display: flex;
      padding-top: 0.7rem;
      padding-left: 0.8rem;
    }

    &-text {
      display: flex;
      padding-top: 0.5rem;
      padding-left: 0.8rem;
    }

    &-left {
      margin-left: 1rem;
      color: var(--parsec-color-light-primary-600);
      background-color: var(--parsec-color-light-primary-50);
    }

    &-right {
      margin-right: 1rem;
      color: var(--parsec-color-light-secondary-hard-grey);
      background-color: var(--parsec-color-light-secondary-background);
    }
  }

  &::after {
    content: '';
    position: absolute;
    width: 100%;
    outline: 1px solid var(--parsec-color-light-secondary-medium);
    top: 7.8rem;
  }
}
</style>
