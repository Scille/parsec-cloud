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
        {{ workspace.currentName }}
      </ion-label>
    </div>

    <!-- role user -->
    <div class="workspace-role">
      <workspace-tag-role :role="workspace.currentSelfRole" />
    </div>

    <!-- user avatars -->
    <div
      class="workspace-users"
      v-show="clientProfile !== UserProfile.Outsider"
    >
      <avatar-group
        v-show="workspace.sharing.length === 0"
        class="shared-group"
        :people="workspace.sharing.map((item) => item[0].humanHandle.label)"
        :max-display="2"
        @click.stop="$emit('shareClick', $event, workspace)"
      />
      <ion-label
        class="label-not-shared cell"
        v-show="workspace.sharing.length !== 0"
        @click.stop="$emit('shareClick', $event, workspace)"
      >
        {{ $t('WorkspacesPage.Workspace.notShared') }}
      </ion-label>
    </div>

    <!-- last update -->
    <div
      class="workspace-lastUpdate"
      v-show="false"
    >
      <ion-label class="label-last-update cell">
        {{ formatTimeSince(workspace.lastUpdated, '--', 'short') }}
      </ion-label>
    </div>

    <!-- workspace size -->
    <div
      class="workspace-size"
      v-show="false"
    >
      <ion-label class="label-size cell">
        {{ formatFileSize(workspace.size) }}
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
import { formatTimeSince } from '@/common/date';
import { formatFileSize } from '@/common/file';
import AvatarGroup from '@/components/workspaces/AvatarGroup.vue';
import WorkspaceTagRole from '@/components/workspaces/WorkspaceTagRole.vue';
import { UserProfile, WorkspaceInfo } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonLabel } from '@ionic/vue';
import { business, cloudDone, cloudOffline, ellipsisHorizontal } from 'ionicons/icons';
import { ref } from 'vue';

const isSelected = ref(false);

defineProps<{
  workspace: WorkspaceInfo;
  clientProfile: UserProfile;
}>();

defineEmits<{
  (e: 'click', event: Event, workspace: WorkspaceInfo): void;
  (e: 'menuClick', event: Event, workspace: WorkspaceInfo): void;
  (e: 'shareClick', event: Event, workspace: WorkspaceInfo): void;
}>();
</script>

<style lang="scss" scoped>
.workspace-name {
  padding: 0.75rem 1rem;
  width: 100%;
  max-width: 40vw;
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
  overflow: visible;

  .shared-group {
    padding: 0.25rem;
    transition: background 0.2s ease-in-out;

    &:hover {
      background: var(--parsec-color-light-primary-100);
    }
  }

  .label-not-shared {
    color: var(--parsec-color-light-secondary-grey);
    padding: 0.375rem 0.5rem;
    border-radius: var(--parsec-radius-6);
    transition: background 0.15s ease-in-out;
    flex-grow: 0 !important;
    position: relative;
    z-index: 2;

    &:hover {
      background: var(--parsec-color-light-primary-100);
      color: var(--parsec-color-light-primary-700) !important;
    }
  }
}

.workspace-lastUpdate {
  min-width: 11.25rem;
  flex-grow: 0;
}

.workspace-size {
  max-width: 7.5rem;
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

.label-size,
.label-last-update {
  color: var(--parsec-color-light-secondary-grey);
}
</style>
