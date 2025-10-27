<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="list-sidebar">
    <ion-header
      lines="none"
      class="list-sidebar-header title-h5"
      :class="isContentVisible ? 'open' : 'close'"
    >
      <ion-text
        class="list-sidebar-header-text"
        :class="{ clickable: isHeaderClickable }"
        @click="isHeaderClickable && emits('headerClicked')"
      >
        {{ $msTranslate(title) }}
        <ms-image
          class="list-sidebar-header-text__open-icon"
          v-if="isHeaderClickable"
          :image="OpenIcon"
        />
      </ion-text>
      <ion-icon
        @click="emits('update:isContentVisible', !isContentVisible)"
        class="list-sidebar-header__toggle"
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
import { IonHeader, IonIcon, IonList, IonText } from '@ionic/vue';
import { chevronDown, chevronForward } from 'ionicons/icons';
import { MsImage, OpenIcon, Translatable } from 'megashark-lib';

defineProps<{
  title: Translatable;
  isContentVisible: boolean;
  isHeaderClickable?: boolean;
}>();

const emits = defineEmits<{
  (event: 'update:isContentVisible', visibility: boolean): void;
  (event: 'headerClicked'): void;
}>();
</script>

<style scoped lang="scss">
.list-sidebar {
  display: flex;
  flex-direction: column;
  margin-bottom: 1.5rem;
  background: transparent;

  &.list-file {
    margin: 1rem 0;
  }

  &-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    transition: border 0.2s ease-in-out;
    width: 100%;
    padding: 0 1rem 0.75rem 1.25rem;
    overflow: hidden;
    border-bottom: 1px solid var(--parsec-color-light-primary-30-opacity15);

    &.open {
      margin-bottom: 1rem;
    }

    &-text {
      color: var(--parsec-color-light-secondary-inversed-contrast);
      display: flex;
      align-items: center;
      user-select: none;
      text-overflow: ellipsis;
      overflow: hidden;
      white-space: nowrap;

      &__open-icon {
        --fill-color: var(--parsec-color-light-secondary-inversed-contrast);
        margin-left: 0.5rem;
        width: 1rem;
        height: 1rem;
      }

      &.clickable {
        cursor: pointer;

        &:hover {
          text-decoration: underline;
        }
      }
    }

    &__toggle {
      user-select: none;
      padding: 0.125rem;
      font-size: 1rem;
      border-radius: var(--parsec-radius-6);
      color: var(--parsec-color-light-secondary-inversed-contrast);
      flex-shrink: 0;
      cursor: pointer;
      opacity: 0.4;

      &:hover {
        background: var(--parsec-color-light-primary-30-opacity15);
      }
    }

    &:hover {
      .list-sidebar-header__toggle {
        opacity: 1;
      }
    }
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding-inline: 1rem;
  }
}
</style>
