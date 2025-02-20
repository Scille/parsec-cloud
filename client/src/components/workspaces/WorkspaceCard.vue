<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="workspace-card-item ion-no-padding"
    :class="{ 'workspace-hovered': isHovered || menuOpened }"
    @click="$emit('click', workspace, $event)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @contextmenu="onOptionsClick"
  >
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
    <div
      class="card-option"
      @click.stop="onOptionsClick($event)"
    >
      <ion-icon :icon="ellipsisHorizontal" />
    </div>
    <div class="workspace-card">
      <ion-avatar class="workspace-card-icons">
        <ion-icon
          class="workspace-card-icons__item"
          :icon="business"
        />
        <ion-icon
          class="cloud-overlay"
          :class="workspace.availableOffline ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
          :icon="workspace.availableOffline ? cloudDone : cloudOffline"
        />
      </ion-avatar>

      <ion-title class="workspace-card__title body-lg">
        {{ workspace.currentName }}
      </ion-title>

      <ion-text
        class="card-content-last-update subtitles-sm"
        v-show="false"
      >
        <ion-icon
          :icon="time"
          class="time"
        />
        <span>{{ $msTranslate(formatTimeSince(workspace.lastUpdated, '--', 'short')) }}</span>
      </ion-text>

      <div
        class="workspace-info"
        v-show="clientProfile !== UserProfile.Outsider"
      >
        <ion-text class="label-file-size body-sm">
          <span v-show="false">
            {{ $msTranslate(formatFileSize(workspace.size)) }}
          </span>
        </ion-text>
        <avatar-group
          v-show="workspace.sharing.length > 0"
          class="shared-group"
          :people="workspace.sharing.map((item) => item[0].humanHandle.label)"
          :max-display="2"
          @click.stop="$emit('shareClick', workspace, $event)"
        />
        <ion-label
          v-show="workspace.sharing.length === 0"
          @click.stop="$emit('shareClick', workspace, $event)"
          class="body-sm not-shared-label"
        >
          {{ $msTranslate('WorkspacesPage.Workspace.notShared') }}
        </ion-label>
      </div>
    </div>
    <div class="card-bottom">
      <workspace-tag-role
        class="card-bottom__role"
        :role="workspace.currentSelfRole"
      />
      <div class="card-bottom__icons">
        <ion-icon
          :icon="personAdd"
          :class="{
            'icon-disabled': workspace.currentSelfRole === WorkspaceRole.Reader || workspace.currentSelfRole === WorkspaceRole.Contributor,
          }"
        />
        <ion-icon
          :icon="cloudUpload"
          :class="{
            'icon-disabled': workspace.currentSelfRole === WorkspaceRole.Reader,
          }"
        />
        <ion-icon :icon="eye" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatTimeSince } from 'megashark-lib';
import { formatFileSize } from '@/common/file';
import AvatarGroup from '@/components/workspaces/AvatarGroup.vue';
import { UserProfile, WorkspaceInfo, WorkspaceRole } from '@/parsec';
import { IonAvatar, IonIcon, IonLabel, IonText, IonTitle } from '@ionic/vue';
import { business, cloudDone, cloudOffline, ellipsisHorizontal, star, time, cloudUpload, personAdd, eye } from 'ionicons/icons';
import { ref } from 'vue';
import { WorkspaceTagRole } from '@/components/workspaces';

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
.workspace-card-item {
  --background: var(--parsec-color-light-secondary-background);
  background: var(--parsec-color-light-secondary-background);
  cursor: pointer;
  text-align: center;
  user-select: none;
  border-radius: var(--parsec-radius-12);
  width: 16rem;
  overflow: hidden;

  &:hover {
    background-color: var(--parsec-color-light-primary-30);
    border-color: var(--parsec-color-light-primary-100);
  }
}

.card-option {
  top: 0;
  right: 0;
  padding: 0.75rem;
}

.workspace-favorite-icon {
  display: flex;
  align-items: center;
  font-size: 1.25rem;
  margin-left: auto;
  padding: 0.25rem;
  border-radius: var(--parsec-radius-6);
  transition: color 150ms ease-out;
  position: absolute;
  text-align: left;
  left: 0.75rem;
  top: 0.75rem;

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

.workspace-card {
  padding: 2rem 1em 1em;
  width: 100%;

  &-icons {
    margin: 0 auto 0.5rem;
    position: relative;
    height: fit-content;
    display: flex;
    justify-content: center;
    align-items: center;
    color: var(--parsec-color-light-primary-900);
    width: 100%;

    &__item {
      font-size: 2.5rem;
    }

    .cloud-overlay {
      position: absolute;
      font-size: 1.25rem;
      bottom: -11px;
      left: 53%;
      padding: 0.25rem;
      background: var(--parsec-color-light-secondary-background);
      border-radius: 50%;
    }

    .cloud-overlay-ok {
      color: var(--parsec-color-light-primary-500);
    }

    .cloud-overlay-ko {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  &__title {
    color: var(--parsec-color-light-primary-900);
    font-size: 18px;
    text-align: center;

    ion-text {
      width: 100%;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }
}

.workspace-info {
  display: flex;
  justify-content: space-between;
  padding: 0.625rem 0 0;
  align-items: center;
  color: var(--parsec-color-light-secondary-grey);
  height: 2rem;

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

  .shared-group {
    padding: 0.25rem;
    transition: background 0.2s ease-in-out;

    &:hover {
      background: var(--parsec-color-light-primary-100);
    }
  }
}

/* No idea how to change the color of the ion-item */
.workspace-card__title::part(native),
.workspace-info::part(native) {
  background-color: var(--parsec-color-light-secondary-background);
}

.card-bottom {
  background-color: var(--parsec-color-light-secondary-white);
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0.75rem 0.5rem 0.5rem;

  &__icons {
    color: var(--parsec-color-light-secondary-hard-grey);
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.25rem;

    .icon-disabled {
      opacity: 0.3;
    }
  }
}
</style>
