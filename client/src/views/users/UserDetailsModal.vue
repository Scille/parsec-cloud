<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="$t('UsersPage.UserDetailsModal.title')"
      :close-button="{ visible: true }"
      :confirm-button="{
        label: $t('UsersPage.UserDetailsModal.actions.close'),
        disabled: false,
      }"
    >
      <div class="details">
        <div class="details-item">
          <ion-text class="details-item__title subtitles-sm">
            {{ $t('UsersPage.UserDetailsModal.subtitles.name') }}
          </ion-text>
          <ion-text class="details-item__text subtitles-normal">
            {{ user.humanHandle.label }}
          </ion-text>
          <ion-chip
            v-if="user.isRevoked()"
            color="danger"
            class="revoked"
          >
            <ion-icon
              class="revoked__icon"
              :icon="ellipse"
            />
            <ion-label class="caption-caption">
              {{ $t('UsersPage.UserDetailsModal.subtitles.revoked') }}
            </ion-label>
          </ion-chip>
        </div>
        <div class="details-item">
          <ion-text class="details-item__title subtitles-sm">
            {{ $t('UsersPage.UserDetailsModal.subtitles.joined') }}
          </ion-text>
          <ion-text class="details-item__text body-lg">
            {{ timeSince(user.createdOn, '--', 'short') }}
          </ion-text>
        </div>
      </div>

      <ion-list class="workspace">
        <ion-text class="subtitles-sm workspace-title">
          {{ $t('UsersPage.UserDetailsModal.subtitles.commonWorkspaces') }}
        </ion-text>
        <div class="workspace-list">
          <ion-card
            v-for="workspace in workspaces"
            :key="workspace.id"
            :disabled="user.isRevoked()"
            class="workspace-list-item"
          >
            <ion-card-content class="item-container">
              <ion-icon
                class="item-container__icon"
                :icon="business"
              />
              <ion-text class="item-container__name cell">
                {{ workspace.name }}
              </ion-text>
              <workspace-tag-role
                class="item-container__tag"
                :role="workspace.role"
              />
            </ion-card-content>
          </ion-card>
        </div>
      </ion-list>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { Formatters, FormattersKey } from '@/common/injectionKeys';
import { MsModal } from '@/components/core';
import WorkspaceTagRole from '@/components/workspaces/WorkspaceTagRole.vue';
import { UserInfo, WorkspaceRole } from '@/parsec';
import { IonCard, IonCardContent, IonChip, IonIcon, IonLabel, IonList, IonPage, IonText } from '@ionic/vue';
import { business, ellipse } from 'ionicons/icons';
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
</script>

<style lang="scss" scoped>
.details {
  display: flex;
  margin-bottom: 1rem;
  .details-item {
    display: flex;
    flex-direction: column;
    width: 50%;
    gap: 0.5rem;

    &__title {
      color: var(--parsec-color-light-secondary-text);
    }

    &__text {
      color: var(--parsec-color-light-primary-800);
    }

    .revoked {
      display: flex;
      gap: 0.5rem;
      align-items: center;
      justify-content: center;
      width: 5.5rem;
      padding: 0.125rem;
      min-height: 0;
      border-radius: var(--parsec-radius-6);
      margin: 0;

      &__icon {
        font-size: 0.375rem;
      }
    }
  }
}

.workspace {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  &-title {
    color: var(--parsec-color-light-secondary-text);
  }

  &-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    .workspace-list-item {
      background: var(--parsec-color-light-secondary-background);
      border: 1px solid var(--parsec-color-light-secondary-disabled);
      display: flex;
      align-items: center;
      padding: 1rem 1.5rem 1rem 1rem;
      margin: 0;
      box-shadow: none;
      &::after {
        display: none;
      }
    }
  }
}

.item-container {
  height: fit-content;
  display: flex;
  justify-content: left;
  align-items: center;
  color: var(--parsec-color-light-primary-800);
  width: 100%;
  padding: 0;

  &__icon {
    min-width: 1.5rem;
    height: 1.5rem;
  }

  &__name {
    margin-left: 0.75rem;
    width: 100%;
  }

  &__tag {
    margin-left: auto;
  }
}
</style>
