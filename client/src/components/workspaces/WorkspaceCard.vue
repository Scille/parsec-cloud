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
    <div class="workspace-card-content">
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

      <ion-text
        class="workspace-card-content__title subtitles-sm"
        :title="workspace.currentName"
      >
        {{ workspace.currentName }}
      </ion-text>

      <div class="workspace-card-content-info">
        <ion-text class="workspace-card-content__update subtitles-sm">
          <span v-if="false">{{ $msTranslate(formatTimeSince(workspace.lastUpdated, '--', 'short')) }}</span>
          <ion-icon
            class="cloud-overlay"
            :class="workspace.availableOffline ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
            :icon="workspace.availableOffline ? cloudDone : cloudOffline"
          />
        </ion-text>

        <ion-text
          class="workspace-card-content__size body-sm"
          v-if="false"
        >
          {{ $msTranslate(formatFileSize(workspace.size)) }}
        </ion-text>
      </div>
    </div>
    <div class="workspace-card-bottom">
      <workspace-tag-role
        class="workspace-card-bottom__role"
        :role="workspace.currentSelfRole"
      />
      <div class="workspace-card-bottom__icons">
        <div
          v-show="clientProfile !== UserProfile.Outsider"
          class="icon-share-container"
          @click.stop="$emit('shareClick', workspace, $event)"
        >
          <ion-icon
            :icon="shareSocial"
            class="icon-share"
          />
        </div>
        <div
          class="icon-option-container"
          @click.stop="onOptionsClick($event)"
        >
          <ion-icon
            :icon="ellipsisHorizontal"
            class="icon-option"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatFileSize } from '@/common/file';
import { WorkspaceTagRole } from '@/components/workspaces';
import { UserProfile, WorkspaceInfo } from '@/parsec';
import { IonIcon, IonText } from '@ionic/vue';
import { cloudDone, cloudOffline, ellipsisHorizontal, shareSocial, star } from 'ionicons/icons';
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
.workspace-card-item {
  cursor: pointer;
  text-align: center;
  user-select: none;
  border-radius: var(--parsec-radius-12);
  border: 1px solid var(--parsec-color-light-secondary-disabled);
  max-width: 16rem;
  width: 100%;
  overflow: hidden;
  padding: 0.25rem;
  transition: all 0.15s ease-in-out;

  @include ms.responsive-breakpoint('sm') {
    max-width: 100%;
  }

  @include ms.responsive-breakpoint('xs') {
    max-width: 100%;
  }

  &:hover {
    border: 1px solid var(--parsec-color-light-secondary-medium) !important;
    box-shadow: var(--parsec-shadow-light);
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
  right: 1rem;
  top: 3.75rem;
  z-index: 10;

  &__on {
    color: var(--parsec-color-light-primary-600);

    &:hover {
      background: var(--parsec-color-light-primary-100);
      color: var(--parsec-color-light-primary-700);
    }
  }

  &__off {
    color: var(--parsec-color-light-secondary-light);

    &:hover {
      background: var(--parsec-color-light-primary-100);
      color: var(--parsec-color-light-primary-600);
      opacity: 0.9;
    }
  }
}

.workspace-card-content {
  transition: all 0.15s ease-in-out;
  padding: 1.5rem 1rem;
  width: 100%;
  background: var(--parsec-color-light-secondary-premiere);
  border-radius: var(--parsec-radius-8);
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  overflow: hidden;

  &__title {
    color: var(--parsec-color-light-primary-900);
    font-size: 18px;
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;

    ion-text {
      width: 100%;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }

  &__update {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    color: var(--parsec-color-light-secondary-grey);
    gap: 0.25rem;

    .cloud-overlay {
      font-size: 1rem;
      padding: 0.25rem;
      border-radius: 50%;
    }

    .cloud-overlay-ok {
      color: var(--parsec-color-light-primary-500);
    }

    .cloud-overlay-ko {
      color: var(--parsec-color-light-secondary-grey);
    }
  }

  &-info {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  &__size {
    color: var(--parsec-color-light-secondary-grey);
  }

  &:hover {
    background: var(--parsec-color-light-primary-50) !important;
  }
}

.workspace-card-bottom {
  background-color: var(--parsec-color-light-secondary-white);
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.375rem;

  &__icons {
    color: var(--parsec-color-light-secondary-hard-grey);
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.25rem;

    .icon-option-container,
    .icon-share-container {
      display: flex;
      align-items: center;
      padding: 0.25rem;
      border-radius: var(--parsec-radius-6);
      transition: background 0.15s ease-in-out;
      gap: 0.25rem;

      .icon-option,
      .icon-share {
        font-size: 1.375rem;

        @include ms.responsive-breakpoint('sm') {
          font-size: 1.5rem;
        }
      }

      &:hover {
        background: var(--parsec-color-light-primary-50);
        color: var(--parsec-color-light-primary-600);
      }
    }

    @include ms.responsive-breakpoint('sm') {
      padding: 0 0.25rem;
    }
  }
}
</style>
