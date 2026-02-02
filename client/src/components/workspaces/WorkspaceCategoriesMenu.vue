<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="workspace-categories-menu">
    <div
      class="workspace-categories-menu-item"
      v-for="item in workspaceMenuList"
      :key="item.key"
      :class="{ active: item.key === activeMenu }"
      @click="$emit('updateMenu', item.key)"
    >
      <ion-icon
        v-if="isLargeDisplay"
        class="workspace-categories-menu-item__icon"
        :icon="item.icon"
      />
      <ion-text class="workspace-categories-menu-item__text title-h4">
        {{ $msTranslate(item.label) }}
      </ion-text>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { WorkspaceMenu } from '@/views/workspaces/types';
import { IonIcon, IonText } from '@ionic/vue';
import { eyeOff, rocket, star, time } from 'ionicons/icons';
import { useWindowSize, WindowSizeBreakpoints } from 'megashark-lib';
import { computed } from 'vue';

const { isLargeDisplay, windowWidth } = useWindowSize();

defineProps<{
  activeMenu: WorkspaceMenu;
}>();

defineEmits<{
  (e: 'updateMenu', value: WorkspaceMenu): void;
}>();

const workspaceMenuList = computed(() => [
  {
    icon: rocket,
    key: WorkspaceMenu.All,
    label: windowWidth.value > WindowSizeBreakpoints.MD ? 'WorkspacesPage.categoriesMenu.all' : 'WorkspacesPage.categoriesMenu.allShort',
  },
  {
    icon: time,
    key: WorkspaceMenu.Recent,
    label:
      windowWidth.value > WindowSizeBreakpoints.MD ? 'WorkspacesPage.categoriesMenu.recent' : 'WorkspacesPage.categoriesMenu.recentShort',
  },
  {
    icon: star,
    key: WorkspaceMenu.Favorites,
    label: 'WorkspacesPage.categoriesMenu.favorites',
  },
  {
    icon: eyeOff,
    key: WorkspaceMenu.Hidden,
    label: 'WorkspacesPage.categoriesMenu.hidden',
  },
]);
</script>

<style scoped lang="scss">
.workspace-categories-menu {
  display: flex;
  padding: 0.25rem;
  gap: 0.125rem;
  width: fit-content;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-premiere);
  position: relative;
  z-index: 5;

  @include ms.responsive-breakpoint('lg') {
    width: 100%;
    margin: 0 1rem;
    gap: 0;
  }

  &::after {
    content: '';
    position: absolute;
    height: calc(100% - 0.5rem);
    width: 12rem;
    top: 0;
    bottom: 0;
    transform: translateY(0.25rem);
    border-radius: var(--parsec-radius-8);
    background-color: var(--parsec-color-light-secondary-white);
    z-index: 2;
    transition:
      right 0.25s ease-in-out,
      left 0.25s ease-in-out;

    @include ms.responsive-breakpoint('lg') {
      width: calc(100% / 4 - 0.25rem);
    }
  }

  &-item {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    cursor: pointer;
    flex: 1 0 12rem;
    min-width: 12rem;
    padding: 0.5rem 1rem;
    color: var(--parsec-color-light-secondary-soft-text);
    box-shadow: var(--parsec-shadow-filter);
    position: relative;
    border-radius: var(--parsec-radius-8);
    z-index: 3;
    transition: background-color 0.2s ease-in-out;

    &:not(:last-child)::after {
      content: '';
      position: absolute;
      top: 0;
      bottom: 0;
      right: 0;
      width: 1px;
      height: 100%;
      background-color: var(--parsec-color-light-secondary-disabled);
    }

    @include ms.responsive-breakpoint('lg') {
      flex-basis: auto;
      min-width: auto;
      width: calc(100% / 4 - 1rem);
      text-align: center;
      padding: 0.375rem 0.5rem;
    }

    &__text {
      @include ms.responsive-breakpoint('sm') {
        font-size: 0.9375rem !important;
      }
    }

    &:not(.active):hover {
      background-color: var(--parsec-color-light-secondary-background);
    }

    &__icon {
      font-size: 1rem;
      flex-shrink: 0;
    }

    &.active {
      .workspace-categories-menu-item__text {
        background: var(--parsec-color-light-gradient-background);
        background-clip: text;
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
      }

      .workspace-categories-menu-item__icon {
        color: var(--parsec-color-light-primary-500);
      }
    }
  }

  &:has(.workspace-categories-menu-item:nth-child(1).active) {
    &::after {
      left: 0.25rem;
    }

    .workspace-categories-menu-item:nth-child(1)::after {
      content: none;
    }
  }

  &:has(.workspace-categories-menu-item:nth-child(2).active) {
    &::after {
      left: 25%;
    }

    .workspace-categories-menu-item:nth-child(1)::after,
    .workspace-categories-menu-item:nth-child(2)::after {
      content: none;
    }
  }

  &:has(.workspace-categories-menu-item:nth-child(3).active) {
    &::after {
      left: 50%;
    }

    .workspace-categories-menu-item:nth-child(2)::after,
    .workspace-categories-menu-item:nth-child(3)::after {
      content: none;
    }
  }

  &:has(.workspace-categories-menu-item:nth-child(4).active) {
    &::after {
      left: 75%;
    }

    .workspace-categories-menu-item:nth-child(3)::after {
      content: none;
    }
  }
}
</style>
