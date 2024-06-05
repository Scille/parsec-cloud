<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="card"
    :class="{ 'workspace-hovered': isHovered || menuOpened }"
    @click="$emit('click', workspace, $event)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
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
    <div class="card-content">
      <ion-avatar class="card-content-icons">
        <ion-icon
          class="card-content-icons__item"
          :icon="business"
        />
        <ion-icon
          class="cloud-overlay"
          :class="workspace.availableOffline ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
          :icon="workspace.availableOffline ? cloudDone : cloudOffline"
        />
      </ion-avatar>

      <ion-title class="card-content__title body-lg">
        {{ workspace.currentName }}
      </ion-title>

      <ion-text class="card-content-last-update subtitles-sm">
        <ion-icon
          :icon="time"
          class="time"
        />
        <span v-show="true">{{ $msTranslate(formatTimeSince(workspace.lastUpdated, '--', 'short')) }}</span>
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
  </div>
</template>

<script setup lang="ts">
import { formatTimeSince } from 'megashark-lib';
import { formatFileSize } from '@/common/file';
import AvatarGroup from '@/components/workspaces/AvatarGroup.vue';
import { UserProfile, WorkspaceInfo } from '@/parsec';
import { IonAvatar, IonIcon, IonLabel, IonText, IonTitle } from '@ionic/vue';
import { business, cloudDone, cloudOffline, ellipsisHorizontal, star, time } from 'ionicons/icons';
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
  menuOpened.value = true;
  emits('menuClick', props.workspace, event, () => {
    menuOpened.value = false;
  });
}
</script>

<style lang="scss" scoped>
.card {
  padding: 2rem 1em 1em;
  cursor: pointer;
  text-align: center;
  background-color: var(--parsec-color-light-secondary-background);
  user-select: none;
  border-radius: var(--parsec-radius-12);
  width: 16rem;

  &:hover {
    background-color: var(--parsec-color-light-primary-30);
  }
}

.card-option {
  color: var(--parsec-color-light-secondary-grey);
  text-align: right;
  position: absolute;
  display: flex;
  align-items: center;
  top: 0;
  right: 1rem;
  font-size: 1.5rem;
  padding: 0.75rem;

  &:hover {
    color: var(--parsec-color-light-primary-500);
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

.card-content-icons {
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
    bottom: -10px;
    left: 54%;
    padding: 2px;
    background: white;
    border-radius: 50%;
  }

  .cloud-overlay-ok {
    color: var(--parsec-color-light-primary-500);
  }

  .cloud-overlay-ko {
    color: var(--parsec-color-light-secondary-text);
  }
}

.card-content__title {
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

.card-content-last-update {
  color: var(--parsec-color-light-secondary-grey);
  text-align: center;
  margin: 0.5rem 0 2rem;
  display: flex;
  gap: 0.25rem;
  justify-content: center;
  align-items: center;
}

.workspace-info {
  display: flex;
  justify-content: space-between;
  padding: 0.625rem 0;
  align-items: center;
  color: var(--parsec-color-light-secondary-grey);
  border-top: 1px solid var(--parsec-color-light-secondary-disabled);

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
.card-content__title::part(native),
.workspace-info::part(native),
.card-content-last-update::part(native) {
  background-color: var(--parsec-color-light-secondary-background);
}
</style>
