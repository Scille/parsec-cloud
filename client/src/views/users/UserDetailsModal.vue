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
        <div
          class="workspace-list"
          v-show="sharedWorkspaces.length > 0"
        >
          <ion-card
            v-for="sharedWorkspace in sharedWorkspaces"
            :key="sharedWorkspace.workspace.id"
            :disabled="user.isRevoked()"
            class="workspace-list-item"
          >
            <ion-card-content class="item-container">
              <ion-icon
                class="item-container__icon"
                :icon="business"
              />
              <ion-text class="item-container__name cell">
                {{ sharedWorkspace.workspace.name }}
              </ion-text>
              <workspace-tag-role
                class="item-container__tag"
                :role="sharedWorkspace.userRole"
              />
            </ion-card-content>
          </ion-card>
        </div>
        <div
          class="workspace-empty body"
          v-show="sharedWorkspaces.length === 0"
        >
          {{ $t('UsersPage.UserDetailsModal.noSharedWorkspaces') }}
        </div>
      </ion-list>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { Formatters, FormattersKey } from '@/common/injectionKeys';
import { MsModal } from '@/components/core';
import WorkspaceTagRole from '@/components/workspaces/WorkspaceTagRole.vue';
import { SharedWithInfo, UserInfo, getWorkspacesSharedWith } from '@/parsec';
import { Notification, NotificationKey, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import { translate } from '@/services/translation';
import { IonCard, IonCardContent, IonChip, IonIcon, IonLabel, IonList, IonPage, IonText } from '@ionic/vue';
import { business, ellipse } from 'ionicons/icons';
import { Ref, defineProps, inject, onMounted, ref } from 'vue';

const { timeSince } = inject(FormattersKey)! as Formatters;
const sharedWorkspaces: Ref<Array<SharedWithInfo>> = ref([]);
const notificationManager: NotificationManager = inject(NotificationKey)!;

const props = defineProps<{
  user: UserInfo;
}>();

onMounted(async () => {
  const result = await getWorkspacesSharedWith(props.user.id);

  if (result.ok) {
    sharedWorkspaces.value = result.value;
  } else {
    notificationManager.showToast(
      new Notification({
        title: translate('UsersPage.UserDetailsModal.failedToListWorkspaces.title'),
        message: translate('UsersPage.UserDetailsModal.failedToListWorkspaces.message'),
        level: NotificationLevel.Error,
      }),
    );
  }
});
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
  overflow-y: auto;

  &-title {
    color: var(--parsec-color-light-secondary-text);
  }

  &-empty {
    display: flex;
    flex-direction: column;
    color: var(--parsec-color-light-secondary-grey);
  }

  &-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    overflow: auto;
    height: 100%;
    min-height: min-content;

    .workspace-list-item {
      background: var(--parsec-color-light-secondary-background);
      border: 1px solid var(--parsec-color-light-secondary-disabled);
      flex-shrink: 0;
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
