<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
        <ion-text class="tab-bar-menu-button__text">
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
  background: ms.color('surface-base-default');
  padding: ms.spacing('padding-7xl') ms.spacing('padding-3xl') ms.spacing('padding-5xl');
  gap: ms.spacing('gap-6xl') ms.spacing('gap-none');
  justify-content: space-between;
  border: none;
  border-top: ms.border('thin') solid ms.color('border-neutral-default-subtle');
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
    gap: ms.spacing('gap-md');
    cursor: pointer;
    color: ms.color('text-neutral-default');

    &-danger {
      color: ms.color('text-error-default');
    }

    &__icon {
      font-size: 1.5rem;
      width: 1.5rem;
      height: 1.5rem;
      --fill-color: #{ms.color('icon-neutral-default')};
    }

    &__text {
      @include ms.font('label-xs-medium');
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
