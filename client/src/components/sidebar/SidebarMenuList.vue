<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-list class="list-sidebar">
    <ion-header
      lines="none"
      class="list-sidebar-header"
      :class="isContentVisible ? 'open' : 'close'"
      @click="emits('update:isContentVisible', !isContentVisible)"
    >
      <ion-text class="list-sidebar-header__text">
        {{ $msTranslate(title) }}
      </ion-text>
      <ion-icon
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
import { Translatable } from 'megashark-lib';

defineProps<{
  title: Translatable;
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
  margin-bottom: 1.5rem;
  background: transparent;

  &.list-file {
    margin: 1rem 0;
  }

  &-header {
    @include ms.font('heading-h6');
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: ms.spacing('gap-lg');
    transition: border 0.2s ease-in-out;
    width: 100%;
    padding: ms.spacing('padding-lg') ms.spacing('padding-2xl') ms.spacing('padding-2xl') ms.spacing('padding-3xl');
    overflow: hidden;
    border-bottom: ms.border('thin') solid ms.color('opacity-inverse-500');
    cursor: pointer;

    &.open {
      margin-bottom: 1rem;
    }

    &__text {
      color: ms.color('text-on-color-heading');
      display: flex;
      align-items: center;
      user-select: none;
      text-overflow: ellipsis;
      overflow: hidden;
      white-space: nowrap;

      &.clickable {
        cursor: pointer;

        &:hover {
          text-decoration: underline;
        }
      }
    }

    &__toggle {
      user-select: none;
      padding: ms.spacing('padding-xs');
      font-size: 1rem;
      border-radius: ms.radius('md');
      color: ms.color('text-on-color-heading');
      flex-shrink: 0;
      cursor: pointer;
      opacity: ms.opacity('5');
    }

    &:hover {
      .list-sidebar-header__toggle {
        opacity: ms.opacity('10');
      }
    }
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-lg');
    padding-inline: ms.spacing('padding-3xl');
  }
}
</style>
