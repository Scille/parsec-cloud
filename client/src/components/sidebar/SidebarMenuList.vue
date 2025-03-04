<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="list-sidebar">
    <ion-header
      lines="none"
      class="list-sidebar-header title-h5"
      :class="isContentVisible ? 'open' : 'close'"
      @click="emits('update:isContentVisible', !isContentVisible)"
    >
      <div class="list-sidebar-header-title">
        <ion-icon
          v-if="icon"
          class="list-sidebar-header-title__icon"
          :icon="icon"
        />
        <ion-text class="list-sidebar-header-title__text">
          {{ $msTranslate(title) }}
        </ion-text>
      </div>
      <ion-icon
        class="list-sidebar-toggle-icon"
        :icon="isContentVisible ? chevronDown : chevronForward"
      />
    </ion-header>
    <div
      v-if="isContentVisible"
      class="list-sidebar-content"
    >
      <slot />
    </div>
  </ion-list>
</template>

<script setup lang="ts">
import { IonList, IonHeader, IonIcon, IonText } from '@ionic/vue';
import { Translatable } from 'megashark-lib';
import { chevronDown, chevronForward } from 'ionicons/icons';

defineProps<{
  title: Translatable;
  icon?: string;
  isContentVisible: boolean;
}>();

const emits = defineEmits<{
  (event: 'update:isContentVisible', visibility: boolean): void;
}>();
</script>

<style scoped lang="scss">
.list-sidebar {
  display: flex;
  flex-direction: column;
  flex: 1;
  margin-bottom: 1rem;
  border-radius: var(--parsec-radius-8);

  &.list-file {
    margin: 1rem 0;
  }

  &-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    transition: border 0.2s ease-in-out;
    border-radius: var(--parsec-radius-6);
    padding: 0.25rem 0.5rem;
    opacity: 0.8;
    cursor: pointer;
    width: 100%;

    &:hover {
      opacity: 1;
      background: var(--parsec-color-light-primary-30-opacity15);
    }

    &.open {
      margin-bottom: 0.5rem;
    }

    &-title {
      color: var(--parsec-color-light-secondary-inversed-contrast);
      display: flex;
      align-items: center;
      overflow: hidden;

      &__icon {
        margin-right: 0.5rem;
        flex-shrink: 0;
        padding: 0.125rem;
        font-size: 1rem;
      }

      &__text {
        user-select: none;
        text-overflow: ellipsis;
        overflow: hidden;
        white-space: nowrap;
      }
    }
  }

  &-toggle-icon {
    user-select: none;
    padding: 0.125rem;
    font-size: 1rem;
    border-radius: var(--parsec-radius-6);
    color: var(--parsec-color-light-secondary-inversed-contrast);
    flex-shrink: 0;
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
}
</style>
