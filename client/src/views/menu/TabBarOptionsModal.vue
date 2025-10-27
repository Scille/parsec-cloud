<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="tab-bar-menu"
    id="tab-bar-options-modal"
  >
    <div
      v-for="(action, index) of actions"
      :key="index"
      @click="onActionClicked(action.action)"
    >
      <div
        class="tab-bar-menu-button"
        :class="action.danger ? 'tab-bar-menu-button-danger' : ''"
      >
        <ion-icon
          v-if="action.icon && !action.image"
          class="tab-bar-menu-button__icon"
          :icon="action.icon"
        />
        <ms-image
          class="tab-bar-menu-button__icon"
          v-if="!action.icon && action.image"
          :image="action.image"
        />
        <ion-text class="tab-bar-menu-button__text button-medium">
          {{ $msTranslate(action.label) }}
        </ion-text>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { MenuAction } from '@/views/menu';
import { IonIcon, IonText, modalController } from '@ionic/vue';
import { MsImage, MsModalResult } from 'megashark-lib';

defineProps<{
  actions: MenuAction[];
}>();

async function onActionClicked(action: MenuAction): Promise<void> {
  await modalController.dismiss({ action: action }, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
.tab-bar-menu {
  background: var(--parsec-color-light-secondary-inversed-contrast);
  padding: 3rem 1rem 2rem;
  gap: 2.5rem 0;
  justify-content: space-between;
  border: none;
  border-top: 1px solid var(--parsec-color-light-secondary-premiere);
  position: relative;
  z-index: 19;
  width: 100%;
  display: grid;
  grid-template-columns: repeat(4, 1fr);

  &-button {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.375rem;
    cursor: pointer;
    color: var(--parsec-color-light-secondary-text);

    &-danger {
      color: var(--parsec-color-light-danger-500);
    }

    &__icon {
      font-size: 1.5rem;
      width: 1.5rem;
      height: 1.5rem;
      --fill-color: var(--parsec-color-light-secondary-text);
    }

    &__text {
      font-size: 0.75rem;
      text-align: center;
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      max-width: 4.75rem;
      width: 100%;
    }
  }
}
</style>
