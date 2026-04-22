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
import { eyeOff, star, time } from 'ionicons/icons';
import { computed } from 'vue';

defineProps<{
  activeMenu: WorkspaceMenu;
}>();

defineEmits<{
  (e: 'updateMenu', value: WorkspaceMenu): void;
}>();

const workspaceMenuList = computed(() => {
  const allMenus = [
    {
      icon: time,
      key: WorkspaceMenu.Recent,
      label: 'WorkspacesPage.categoriesMenu.recent',
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
  ];

  return allMenus.filter((menu) => [WorkspaceMenu.Recent, WorkspaceMenu.Favorites, WorkspaceMenu.Hidden].includes(menu.key));
});
</script>

<style scoped lang="scss">
.workspace-categories-menu {
  display: flex;
  padding: 0.25rem;
  width: fit-content;
  position: relative;
  z-index: 5;
  gap: 0.75rem;

  @include ms.responsive-breakpoint('lg') {
    width: 100%;
    margin: 0 1rem;
  }

  &-item {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    cursor: pointer;
    flex: 1 0 8rem;
    min-width: 8rem;
    padding: 0.5rem 1rem;
    min-width: fit-content;
    color: var(--parsec-color-light-secondary-grey);
    position: relative;
    border-radius: var(--parsec-radius-18);
    overflow: visible;
    z-index: 3;
    transition: background-color 0.2s ease-in-out;

    &:not(.active) {
      outline: 1px solid var(--parsec-color-light-secondary-disabled);
    }

    &__text {
      font-size: 0.9375rem !important;
      pointer-events: none;
      user-select: none;
    }

    &:not(.active):hover {
      background-color: var(--parsec-color-light-secondary-background);
    }

    &__icon {
      font-size: 1rem;
      flex-shrink: 0;
    }

    &.active {
      background-color: var(--parsec-color-light-secondary-text);
      box-shadow: var(--parsec-shadow-soft);

      .workspace-categories-menu-item__text {
        color: var(--parsec-color-light-secondary-white);
      }

      .workspace-categories-menu-item__icon {
        color: var(--parsec-color-light-secondary-white);
      }

      &:hover {
        opacity: 0.9;
      }
    }
  }
}
</style>
