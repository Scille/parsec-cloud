<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    lines="none"
    button
    @click="$emit('workspaceClicked', workspace.handle)"
    :class="currentRouteIsWorkspaceRoute(workspace.handle) ? 'item-selected' : 'item-not-selected'"
    class="sidebar-item menu-default"
    ref="itemRef"
  >
    <ion-label class="sidebar-item-workspace-label">{{ workspace.currentName }}</ion-label>
    <div
      class="workspace-option"
      @click.stop="$emit('contextMenuRequested', $event)"
    >
      <ion-icon :icon="ellipsisHorizontal" />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { IonItem, IonLabel, IonIcon } from '@ionic/vue';
import { WorkspaceInfo, WorkspaceHandle } from '@/parsec';
import { ellipsisHorizontal } from 'ionicons/icons';
import { currentRouteIsWorkspaceRoute } from '@/router';
import { onMounted, onBeforeUnmount, ref } from 'vue';

defineProps<{
  workspace: WorkspaceInfo;
}>();

const emits = defineEmits<{
  (e: 'workspaceClicked', handle: WorkspaceHandle): void;
  (e: 'contextMenuRequested', event: Event): void;
}>();

const itemRef = ref();

onMounted(async () => {
  itemRef.value.$el.addEventListener('contextmenu', (event: Event) => {
    event.preventDefault();
    emits('contextMenuRequested', event);
  });
});

onBeforeUnmount(async () => {
  itemRef.value.removeEventListener('contextmenu');
});
</script>

<style scoped lang="scss">
.sidebar-item {
  --background: none;
  border-radius: var(--parsec-radius-8);
  border: solid 1px var(--parsec-color-light-primary-800);
  --min-height: 0;

  .workspace-option {
    color: var(--parsec-color-light-secondary-grey);
    text-align: right;
    position: absolute;
    display: flex;
    align-items: center;
    top: 0;
    right: 1rem;
    font-size: 1.2rem;
    padding-top: 0.5rem;
    opacity: 0;

    &:hover {
      color: var(--parsec-color-light-primary-30);
    }
  }

  &:hover {
    border: solid 1px var(--parsec-color-light-primary-30-opacity15);
    cursor: pointer;

    .workspace-option {
      opacity: 1;
    }
  }

  &:active,
  &.item-selected {
    --background: var(--parsec-color-light-primary-30-opacity15);
  }

  & > ion-label {
    --color: var(--parsec-color-light-primary-100);
  }

  &-workspace-label {
    position: relative;
    margin-right: 1.1rem;
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
    --color: var(--parsec-color-light-primary-100);
  }
}
</style>
