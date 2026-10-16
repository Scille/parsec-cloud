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
      <ion-text class="button__text">
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
      <ion-text class="button__text">
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
  gap: ms.spacing('gap-3xl');
  padding: ms.spacing('padding-7xl') ms.spacing('padding-4xl') ms.spacing('padding-4xl');
  justify-content: center;
  border-bottom: ms.border('thin') solid ms.color('border-base-default');

  .button {
    width: 100%;
    display: flex;
    flex-direction: column;
    padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl');
    gap: ms.spacing('gap-md');
    border-radius: ms.radius('lg');
    color: ms.color('text-base-description');
    background-color: ms.color('surface-base-default-secondary');

    &__text {
      @include ms.font('label-lg-medium');
    }

    &-icon {
      font-size: 1.125rem;
    }

    &:hover {
      cursor: pointer;
      color: ms.color('text-brand-default-hover');
      background-color: ms.color('surface-brand-default-subtle-hover');
    }
  }
}
</style>
