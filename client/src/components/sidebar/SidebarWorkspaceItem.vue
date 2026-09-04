<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-item
    lines="none"
    button
    @click="$emit('workspaceClicked', workspace)"
    :class="currentRouteIsWorkspaceRoute(workspace.handle) ? 'item-selected' : 'item-not-selected'"
    class="sidebar-item ion-no-padding"
    @contextmenu="onContextMenu"
  >
    <div class="sidebar-item-workspace">
      <ion-icon
        v-if="workspace.isArchived"
        :icon="archive"
        class="sidebar-item-workspace__icon"
      />
      <ion-icon
        v-else-if="workspace.isTrashed"
        :icon="trash"
        class="sidebar-item-workspace__icon"
      />
      <ion-icon
        v-else-if="isHidden"
        :icon="eyeOff"
        class="sidebar-item-workspace__icon"
      />
      <ion-text
        class="sidebar-item-workspace__label"
        :title="workspace.name"
      >
        {{ workspace.name }}
      </ion-text>
      <div
        class="sidebar-item-workspace__option"
        @click.stop="$emit('contextMenuRequested', $event)"
      >
        <ion-icon :icon="ellipsisHorizontal" />
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { WorkspaceInfo } from '@/parsec';
import { currentRouteIsWorkspaceRoute } from '@/router';
import { IonIcon, IonItem, IonText } from '@ionic/vue';
import { archive, ellipsisHorizontal, eyeOff, trash } from 'ionicons/icons';

defineProps<{
  workspace: WorkspaceInfo;
  isHidden: boolean;
}>();

const emits = defineEmits<{
  (e: 'workspaceClicked', workspace: WorkspaceInfo): void;
  (e: 'contextMenuRequested', event: Event): void;
}>();

async function onContextMenu(event: Event): Promise<void> {
  event.preventDefault();
  emits('contextMenuRequested', event);
}
</script>

<style scoped lang="scss">
.sidebar-item {
  @include ms.font('label-md-medium');
  --background: none;
  border-radius: ms.radius('lg');
  border: ms.border('thin') solid transparent;
  --min-height: 0;
  --padding-start: #{ms.spacing('padding-2xl')};
  --padding-end: #{ms.spacing('padding-2xl')};
  --padding-bottom: #{ms.spacing('padding-lg')};
  --padding-top: #{ms.spacing('padding-lg')};

  &:active,
  &.item-selected {
    --background: #{ms.color('surface-neutral-on-color')};
  }

  .sidebar-item-workspace {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: ms.spacing('gap-lg');
    width: 100%;

    &__icon {
      font-size: 1rem;
      color: ms.color('icon-on-color-label');
      flex-shrink: 0;
    }

    &__label {
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      color: ms.color('text-on-color-label');
      width: 100%;
    }

    &__option {
      color: ms.color('icon-on-color-label');
      display: none;
      margin-left: auto;
      font-size: 1rem;

      &:hover {
        color: ms.color('icon-on-color-heading');
      }
    }
  }

  &:hover {
    border-color: ms.color('border-neutral-on-color');
    cursor: pointer;

    .sidebar-item-workspace__label {
      color: ms.color('text-on-color-heading');
    }

    .sidebar-item-workspace__option {
      display: flex;
    }
  }
}
</style>
