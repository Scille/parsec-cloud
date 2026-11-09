<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-list
    class="list-sidebar"
    :class="{ 'list-sidebar--collapsed': collapsed }"
  >
    <ion-header
      v-if="!collapsed"
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
      v-if="isContentVisible || collapsed"
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
  collapsed?: boolean;
}>();

const emits = defineEmits<{
  (event: 'update:isContentVisible', visibility: boolean): void;
}>();
</script>

<style scoped lang="scss">
.list-sidebar {
  display: flex;
  flex-direction: column;
  padding: ms.spacing('padding-none');
  margin-bottom: 1.5rem;
  background: transparent;

  & + & {
    padding-top: ms.spacing('padding-4xl');
    border-top: ms.border('thin') solid ms.color('border-neutral-on-color');
  }

  &.list-file {
    margin: 1rem 0;
  }

  &--collapsed {
    margin-bottom: 0;
    padding-bottom: ms.spacing('padding-2xl');
    border-bottom: ms.border('thin') solid ms.color('border-neutral-on-color');

    .list-sidebar-content {
      padding-inline: ms.spacing('padding-lg');
    }
  }

  &-header {
    @include ms.font('label-sm-emphasis');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-sm');
    margin-inline: ms.spacing('padding-3xl');
    padding: ms.spacing('padding-sm') ms.spacing('padding-2xl') ms.spacing('padding-2xl');
    border-inline: ms.border('thin') solid transparent;
    overflow: hidden;
    cursor: pointer;

    &__text {
      color: ms.color('text-on-color-label');
      opacity: ms.opacity('6');
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
      font-size: 0.75rem;
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
    gap: ms.spacing('gap-sm');
    padding-inline: ms.spacing('padding-3xl');
  }
}
</style>
