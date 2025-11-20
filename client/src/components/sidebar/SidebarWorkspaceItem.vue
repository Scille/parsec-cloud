<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    lines="none"
    button
    @click="$emit('workspaceClicked', workspace)"
    :class="currentRouteIsWorkspaceRoute(workspace.handle) ? 'item-selected' : 'item-not-selected'"
    class="sidebar-item button-medium ion-no-padding"
    @contextmenu="onContextMenu"
  >
    <div class="sidebar-item-workspace">
      <ion-icon
        v-if="isHidden"
        :icon="eyeOff"
        class="sidebar-item-workspace__hide"
      />
      <ion-text
        class="sidebar-item-workspace__label"
        :title="workspace.currentName"
      >
        {{ workspace.currentName }}
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
import { ellipsisHorizontal, eyeOff } from 'ionicons/icons';

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
  --background: none;
  border-radius: var(--parsec-radius-8);
  border: solid 1px transparent;
  --min-height: 0;
  --padding-start: 0.75rem;
  --padding-end: 0.75rem;
  --padding-bottom: 0.5rem;
  --padding-top: 0.5rem;

  &:active,
  &.item-selected {
    --background: var(--parsec-color-light-primary-30-opacity15);
  }

  .sidebar-item-workspace {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;

    &__hide {
      margin-right: 0.5rem;
      font-size: 1rem;
      color: var(--parsec-color-light-primary-30);
      flex-shrink: 0;
    }

    &__label {
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      color: var(--parsec-color-light-secondary-premiere);
      width: 100%;
    }

    &__option {
      color: var(--parsec-color-light-secondary-light);
      display: none;
      margin-left: auto;
      font-size: 1rem;

      &:hover {
        color: var(--parsec-color-light-primary-30);
      }
    }
  }

  &:hover {
    border-color: var(--parsec-color-light-primary-30-opacity15);
    cursor: pointer;

    .sidebar-item-workspace__option {
      display: flex;
    }
  }
}
</style>
