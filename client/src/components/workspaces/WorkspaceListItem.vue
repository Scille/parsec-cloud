<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    class="workspace-list-item no-padding-end"
    :detail="false"
    :class="{ 'workspace-hovered': isHovered || menuOpened }"
    @click="$emit('click', workspace, $event)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
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
      <ion-label
        class="workspace-name__label cell"
        :title="workspace.currentName"
      >
        {{ workspace.currentName }}
      </ion-label>

      <!-- favorites -->
      <div
        class="workspace-favorite-icon"
        :class="{
          'workspace-favorite-icon__on': isFavorite,
          'workspace-favorite-icon__off': !isFavorite,
        }"
        @click.stop="$emit('favoriteClick', workspace, $event)"
      >
        <ion-icon :icon="star" />
      </div>
    </div>

    <!-- role user -->
    <div class="workspace-role">
      <workspace-role-tag :role="workspace.currentSelfRole" />
    </div>

    <!-- user avatars -->
    <div
      class="workspace-users"
      v-show="clientProfile !== UserProfile.Outsider"
    >
      <avatar-group
        v-show="workspace.sharing.length > 0"
        class="shared-group"
        :people="workspace.sharing.map((item) => item[0].humanHandle.label)"
        :max-display="2"
        @click.stop="$emit('shareClick', workspace, $event)"
      />
      <ion-label
        class="not-shared-label cell"
        v-show="workspace.sharing.length === 0"
        @click.stop="$emit('shareClick', workspace, $event)"
      >
        {{ $msTranslate('WorkspacesPage.Workspace.notShared') }}
      </ion-label>
    </div>

    <!-- last update -->
    <div
      class="workspace-last-update"
      v-show="false"
    >
      <ion-label class="label-last-update cell">
        {{ $msTranslate(formatTimeSince(workspace.lastUpdated, '--', 'short')) }}
      </ion-label>
    </div>

    <!-- workspace size -->
    <div
      class="workspace-size"
      v-show="false"
    >
      <ion-label class="label-size cell">
        {{ $msTranslate(formatFileSize(workspace.size)) }}
      </ion-label>
    </div>

    <!-- options -->
    <div class="workspace-options">
      <ion-button
        fill="clear"
        class="options-button"
        @click.stop="onOptionsClick($event)"
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
import { formatFileSize } from '@/common/file';
import AvatarGroup from '@/components/workspaces/AvatarGroup.vue';
import WorkspaceRoleTag from '@/components/workspaces/WorkspaceRoleTag.vue';
import { UserProfile, WorkspaceInfo } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonLabel } from '@ionic/vue';
import { business, cloudDone, cloudOffline, ellipsisHorizontal, star } from 'ionicons/icons';
import { formatTimeSince } from 'megashark-lib';
import { ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);

const props = defineProps<{
  workspace: WorkspaceInfo;
  clientProfile: UserProfile;
  isFavorite: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', workspace: WorkspaceInfo, event?: Event): void;
  (e: 'favoriteClick', workspace: WorkspaceInfo, event?: Event): void;
  (e: 'menuClick', workspace: WorkspaceInfo, event: Event, onFinished: () => void): void;
  (e: 'shareClick', workspace: WorkspaceInfo, event?: Event): void;
}>();

async function onOptionsClick(event: Event): Promise<void> {
  event.preventDefault();
  menuOpened.value = true;
  emits('menuClick', props.workspace, event, () => {
    menuOpened.value = false;
  });
}
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

  .not-shared-label {
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

.workspace-last-update {
  min-width: 11.25rem;
  flex-grow: 0;
}

.workspace-size {
  max-width: 7.5rem;
}

.workspace-options {
  flex-grow: 0;
  margin-left: auto;
  align-items: center;

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

.workspace-favorite-icon {
  display: flex;
  align-items: center;
  font-size: 1.25rem;
  margin-left: auto;
  padding: 0.25rem;
  border-radius: var(--parsec-radius-6);
  transition: color 150ms ease-out;

  &__on {
    color: var(--parsec-color-light-primary-600);

    &:hover {
      background: var(--parsec-color-light-primary-50);
      color: var(--parsec-color-light-primary-700);
    }
  }

  &__off {
    color: var(--parsec-color-light-secondary-disabled);

    &:hover {
      background: var(--parsec-color-light-primary-50);
      color: var(--parsec-color-light-primary-600);
    }
  }
}
</style>
