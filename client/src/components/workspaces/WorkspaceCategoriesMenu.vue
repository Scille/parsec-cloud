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
      <ion-text class="workspace-categories-menu-item__text">
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

const props = defineProps<{
  activeMenu: WorkspaceMenu;
  showHidden: boolean;
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
  ];

  if (props.showHidden) {
    allMenus.push({
      icon: eyeOff,
      key: WorkspaceMenu.Hidden,
      label: 'WorkspacesPage.categoriesMenu.hidden',
    });
  }

  return allMenus;
});
</script>

<style scoped lang="scss">
.workspace-categories-menu {
  display: flex;
  padding: ms.spacing('padding-sm');
  width: fit-content;
  position: relative;
  z-index: 5;
  gap: ms.spacing('gap-2xl');

  @include ms.responsive-breakpoint('lg') {
    width: 100%;
    margin: 0 ms.spacing('padding-3xl');
  }

  &-item {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: ms.spacing('gap-lg');
    cursor: pointer;
    flex: 1 0 8rem;
    min-width: 8rem;
    padding: ms.spacing('padding-lg') ms.spacing('padding-3xl');
    color: ms.color('text-neutral-default');
    position: relative;
    border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    border-radius: ms.radius('3xl');
    box-shadow: ms.shadow('elevation-sm');
    overflow: visible;
    z-index: 3;
    transition:
      background-color 0.2s ease-in-out,
      border-color 0.2s ease-in-out;

    &__text {
      @include ms.font('heading-h6');
      min-width: 0;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      pointer-events: none;
      user-select: none;
    }

    &__icon {
      font-size: 1rem;
      flex-shrink: 0;
      color: ms.color('icon-neutral-default');
    }

    &:not(.active):hover {
      background-color: ms.color('surface-base-default-secondary');
      border-color: ms.color('border-neutral-default-subtle-hover');
      box-shadow: none;

      .workspace-categories-menu-item__text {
        color: ms.color('text-neutral-default-hover');
      }

      .workspace-categories-menu-item__icon {
        color: ms.color('icon-neutral-default-hover');
      }
    }

    &.active {
      background-color: ms.color('surface-neutral-default');
      border-color: ms.color('border-neutral-default-subtle');
      box-shadow: none;

      .workspace-categories-menu-item__text {
        color: ms.color('text-on-color-label');
      }

      .workspace-categories-menu-item__icon {
        color: ms.color('icon-on-color-label');
      }

      &:hover {
        background-color: ms.color('surface-neutral-default-hover');
        border-color: ms.color('border-neutral-default-subtle-hover');
        box-shadow: ms.shadow('elevation-xs');
      }
    }
  }
}
</style>
