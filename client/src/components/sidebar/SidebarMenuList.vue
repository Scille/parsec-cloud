<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="list-sidebar">
    <ion-header
      lines="none"
      class="list-sidebar-header title-h5"
      :class="isContentVisible ? 'open' : 'close'"
      @click="setContentVisible(!isContentVisible)"
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
import { ref } from 'vue';
import { chevronDown, chevronForward } from 'ionicons/icons';

const isContentVisible = ref(true);

defineProps<{
  title: Translatable;
  icon?: string;
}>();

defineExpose({
  isContentVisible,
  setContentVisible,
});

const emits = defineEmits<{
  (event: 'visibilityChanged', visibility: boolean): void;
}>();

function setContentVisible(visible: boolean, blockEvent = false): void {
  isContentVisible.value = visible;
  if (!blockEvent) {
    emits('visibilityChanged', isContentVisible.value);
  }
}
</script>

<style scoped lang="scss">
.list-sidebar {
  display: flex;
  flex-direction: column;
  flex: 1;
  gap: 0.5rem;
  margin-bottom: 1rem;
  padding: 0.5rem;
  border-radius: var(--parsec-radius-8);

  &.list-file {
    margin: 1rem 0;
  }

  &-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    transition: border 0.2s ease-in-out;
    opacity: 0.8;
    cursor: pointer;

    &:hover {
      opacity: 1;
    }

    &.open {
      margin-bottom: 0.5rem;
    }

    &-title {
      color: var(--parsec-color-light-secondary-inversed-contrast);
      display: flex;
      align-items: center;

      &__icon {
        font-size: 1rem;
        margin-right: 0.5rem;
      }

      &__text {
        margin-right: 0.5rem;
        line-height: auto;
        user-select: none;
      }
    }

    &:active {
      .list-sidebar-toggle-icon {
        scale: 0.85;
      }
    }
  }

  &-toggle-icon {
    user-select: none;
    padding: 0.125rem;
    font-size: 1.25rem;
    border-radius: var(--parsec-radius-6);
    color: var(--parsec-color-light-secondary-inversed-contrast);
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
}
</style>
