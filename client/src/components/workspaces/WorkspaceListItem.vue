<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="workspace-list-item"
    :detail="false"
    :class="{ selected: isSelected, 'no-padding-end': !isSelected }"
    @click="$emit('click', $event, workspace)"
  >
    <!-- workspace name -->
    <div class="workspace-name">
      <div class="workspace-name__icons">
        <ion-icon
          class="main-icon"
          :icon="business"
          size="default"
        />
        <ion-icon
          class="cloud-overlay"
          :class="workspace.availableOffline ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
          :icon="workspace.availableOffline ? cloudDone : cloudOffline"
        />
      </div>
      <ion-label class="workspace-name__label cell">
        {{ workspace.name }}
      </ion-label>
    </div>

    <!-- role user -->
    <div class="workspace-role">
      <workspace-tag-role
        :role="workspace.role"
      />
    </div>

    <!-- user avatars -->
    <div class="workspace-users">
      <avatar-group
        v-show="getSharedWith(workspace).length > 0"
        class="shared-group"
        :people="getSharedWith(workspace)"
        :max-display="2"
        @click.stop="$emit('shareClick', $event, workspace)"
      />
      <ion-label
        v-show="getSharedWith(workspace).length === 0"
        @click.stop="$emit('shareClick', $event, workspace)"
      >
        {{ $t('WorkspacesPage.Workspace.notShared') }}
      </ion-label>
    </div>

    <!-- last update -->
    <div class="workspace-lastUpdate">
      <ion-label class="label-last-update cell">
        {{ timeSince(workspace.lastUpdate, '--', 'short') }}
      </ion-label>
    </div>

    <!-- workspace size -->
    <div class="workspace-size">
      <ion-label class="label-size cell">
        {{ fileSize(workspace.size) }}
      </ion-label>
    </div>

    <!-- options -->
    <div class="workspace-options">
      <ion-button
        v-if="!isSelected"
        fill="clear"
        class="options-button"
        @click.stop="$emit('menuClick', $event, workspace)"
      >
        <ion-icon
          :icon="ellipsisHorizontal"
          slot="icon-only"
          class="options-button__icon"
        />
      </ion-button>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import {
  business,
  ellipsisHorizontal,
  cloudDone,
  cloudOffline,
} from 'ionicons/icons';
import { ref, inject } from 'vue';
import { IonIcon, IonButton, IonItem, IonLabel } from '@ionic/vue';
import { FormattersKey, Formatters } from '@/common/injectionKeys';
import { MockWorkspace, getSharedWith } from '@/common/mocks';
import AvatarGroup from '@/components/workspaces/AvatarGroup.vue';
import WorkspaceTagRole from '@/components/workspaces/WorkspaceTagRole.vue';

const isSelected = ref(false);

defineProps<{
  workspace: MockWorkspace
}>();

defineEmits<{
  (e: 'click', event: Event, workspace: MockWorkspace): void
  (e: 'menuClick', event: Event, workspace: MockWorkspace): void
  (e: 'shareClick', event: Event, workspace: MockWorkspace): void
}>();

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const { timeSince, fileSize } = inject(FormattersKey)! as Formatters;
</script>

<style lang="scss" scoped>
.workspace-list-item {
  --background-activated: var(--parsec-color-light-primary-100);
  border-radius: 4px;

  &:hover {
    --background-hover: var(--parsec-color-light-primary-30);
    --background-hover-opacity: 1;
  }

  &:focus {
    --background-focused: var(--parsec-color-light-primary-100);
    --background-focused-opacity: 1;
  }
}

.workspace-list-item>[class^="workspace-"] {
  padding: 0 1rem;
  display: flex;
  align-items: center;
  height: 4rem;
}

.workspace-name {
  padding: .75rem 1rem;
  width: 100%;
  max-width: 20vw;
  white-space: nowrap;
  overflow: hidden;

  &__icons {
  position: relative;
  padding: 5px;

    .main-icon {
      color: var(--parsec-color-light-secondary-text);
      font-size: 1.5rem;
    }

    .cloud-overlay {
      height: 40%;
      width: 40%;
      position: absolute;
      font-size: 1.5rem;
      bottom: 1px;
      left: 53%;
      padding: 2px;
      background: var(--parsec-color-light-secondary-inversed-contrast);
      border-radius: 50%;
    }

    .cloud-overlay-ok {
      color: var(--parsec-color-light-primary-500);
    }

    .cloud-overlay-ko {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  &__label {
    color: var(--parsec-color-light-secondary-text);
    margin-left: 1em;
  }
}

.workspace-role {
  min-width: 11.25rem;
  max-width: 10vw;
  flex-grow: 2;
}

.workspace-users {
  min-width: 14.5rem;
  flex-grow: 0;
}

.workspace-lastUpdate {
  min-width: 11.25rem;
  flex-grow: 0;
}

.workspace-size {
  min-width: 7.5rem;
}

.workspace-options {
  flex-grow: 0;
  margin-left: auto;

  ion-button::part(native) {
    padding: 0;
  }

  .options-button {
    --background-hover: none;

    &__icon {
      color: var(--parsec-color-light-secondary-grey);
    }

    &:hover {
      .options-button__icon {
        color: var(--parsec-color-light-primary-500);
      }
    }
  }
}

.label-size, .label-shared-with, .label-last-update {
  color: var(--parsec-color-light-secondary-grey);
}
</style>
