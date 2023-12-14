<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="$t('UsersPage.UserDetailsModal.title')"
      :close-button="{ visible: true }"
    >
      <div class="details">
        <div class="details-name">
          <ion-text class="button-small">
            {{ $t('UsersPage.UserDetailsModal.subtitles.name') }}
          </ion-text>
          <ion-text class="details-name__fullname subtitles-normal">
            {{ user.humanHandle.label }}
          </ion-text>
        </div>
        <div class="details-joined">
          <ion-text class="button-small">
            {{ $t('UsersPage.UserDetailsModal.subtitles.joined') }}
          </ion-text>
          <ion-text class="details-joined__date body-lg">
            {{ timeSince(user.createdOn, '--', 'short') }}
          </ion-text>
        </div>
      </div>
      <div>
        <ion-chip
          v-if="user.isRevoked()"
          color="danger"
        >
          <ion-icon
            :icon="ellipse"
            class="revoked"
          />
          <ion-label>
            {{ $t('UsersPage.UserDetailsModal.subtitles.revoked') }}
          </ion-label>
        </ion-chip>
      </div>
      <ion-list>
        <ion-text class="button-small">
          {{ $t('UsersPage.UserDetailsModal.subtitles.commonWorkspaces') }}
        </ion-text>
        <ion-card
          v-for="workspace in workspaces"
          :key="workspace.id"
          :disabled="user.isRevoked()"
          class="workspace-card"
        >
          <ion-card-content>
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
              <ion-text class="workspace-name cell">
                {{ workspace.name }}
              </ion-text>
              <workspace-tag-role :role="workspace.role" />
            </ion-avatar>
          </ion-card-content>
        </ion-card>
      </ion-list>
      <ion-button
        class="close-button"
        @click="cancel"
      >
        {{ $t('UsersPage.UserDetailsModal.actions.close') }}
      </ion-button>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { Formatters, FormattersKey } from '@/common/injectionKeys';
import { MsModal } from '@/components/core';
import { MsModalResult } from '@/components/core/ms-modal/types';
import WorkspaceTagRole from '@/components/workspaces/WorkspaceTagRole.vue';
import { UserInfo, WorkspaceRole } from '@/parsec';
import {
  IonAvatar,
  IonButton,
  IonCard,
  IonCardContent,
  IonChip,
  IonIcon,
  IonLabel,
  IonList,
  IonPage,
  IonText,
  modalController,
} from '@ionic/vue';
import { business, cloudDone, cloudOffline, ellipse } from 'ionicons/icons';
import { defineProps, inject } from 'vue';

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const { timeSince } = inject(FormattersKey)! as Formatters;
const workspaces = [
  {
    id: 'fake1',
    name: 'Workspace1',
    role: WorkspaceRole.Owner,
    availableOffline: true,
  },
  {
    id: 'fake2',
    name: 'Workspace2',
    role: WorkspaceRole.Contributor,
    availableOffline: false,
  },
];

defineProps<{
  user: UserInfo;
}>();

async function cancel(): Promise<boolean> {
  return await modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style lang="scss" scoped>
.details {
  display: flex;
  margin-bottom: 1rem;
  .details-name {
    display: flex;
    flex-direction: column;
    width: 50%;
    gap: 0.5rem;
  }
  .details-joined {
    display: flex;
    flex-direction: column;
    width: 50%;
    gap: 0.5rem;
  }
}
.workspace-name {
  margin-left: 1rem;
  margin-right: auto;
}
.close-button {
  display: flex;
  margin-left: auto;
  width: fit-content;
}
.card-content-icons {
  margin: 0 auto 0.5rem;
  position: relative;
  height: fit-content;
  display: flex;
  justify-content: left;
  align-items: center;
  color: var(--parsec-color-light-primary-900);
  width: 100%;

  &__item {
    font-size: 2rem;
  }

  .cloud-overlay {
    position: absolute;
    font-size: 1rem;
    bottom: -10px;
    left: 4%;
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
.revoked {
  font-size: 0.5rem;
  margin-right: 0.5rem;
}
</style>
