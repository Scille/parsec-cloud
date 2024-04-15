<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="card"
    @click="$emit('click', workspace, $event)"
  >
    <div
      class="card-favorite"
      :class="{
        'card-favorite-on': isFavorite,
        'card-favorite-off': !isFavorite,
      }"
      @click.stop="$emit('favoriteClick', workspace, $event)"
    >
      <ion-icon :icon="star" />
    </div>
    <div
      class="card-option"
      @click.stop="$emit('menuClick', workspace, $event)"
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

      <ion-text class="card-content-last-update caption-caption">
        <span v-show="true">{{ $msTranslate('WorkspacesPage.Workspace.lastUpdate') }}</span>
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
import { formatTimeSince } from '@/common/date';
import { formatFileSize } from '@/common/file';
import AvatarGroup from '@/components/workspaces/AvatarGroup.vue';
import { UserProfile, WorkspaceInfo } from '@/parsec';
import { IonAvatar, IonIcon, IonLabel, IonText, IonTitle } from '@ionic/vue';
import { business, cloudDone, cloudOffline, ellipsisHorizontal, star } from 'ionicons/icons';

defineProps<{
  workspace: WorkspaceInfo;
  clientProfile: UserProfile;
  isFavorite: boolean;
}>();

defineEmits<{
  (e: 'click', workspace: WorkspaceInfo, event?: Event): void;
  (e: 'favoriteClick', workspace: WorkspaceInfo, event?: Event): void;
  (e: 'menuClick', workspace: WorkspaceInfo, event: Event): void;
  (e: 'shareClick', workspace: WorkspaceInfo, event?: Event): void;
}>();
</script>

<style lang="scss" scoped>
.card {
  padding: 2rem 1em 1em;
  cursor: pointer;
  text-align: center;
  background-color: var(--parsec-color-light-secondary-background);
  user-select: none;
  border-radius: 8px;
  width: 16rem;

  &:hover {
    background-color: var(--parsec-color-light-primary-30);
    // box-shadow: var(--parsec-shadow-light);
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

.card-favorite {
  text-align: left;
  position: absolute;
  display: flex;
  align-items: center;
  top: 0;
  left: 0.5rem;
  font-size: 1.5rem;
  padding: 0.75rem;
  &-on {
    color: var(--parsec-color-light-primary-500);
  }
  &-off {
    color: var(--parsec-color-light-secondary-disabled);
  }
  &:hover {
    color: var(--parsec-color-light-primary-300);
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
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

.workspace-info {
  display: flex;
  justify-content: space-between;
  padding: 0.625rem 0;
  align-items: center;
  color: var(--parsec-color-light-secondary-grey);
  border-top: 1px solid var(--parsec-color-light-secondary-disabled);

  .not-shared-label {
    text-align: right;
    padding: 0.375rem 0;
  }
}

/* No idea how to change the color of the ion-item */
.card-content__title::part(native),
.workspace-info::part(native),
.card-content-last-update::part(native) {
  background-color: var(--parsec-color-light-secondary-background);
}
</style>
