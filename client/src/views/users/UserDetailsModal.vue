<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="'UsersPage.UserDetailsModal.title'"
      :close-button="{ visible: true }"
      :confirm-button="{
        label: 'UsersPage.UserDetailsModal.actions.close',
        disabled: false,
      }"
    >
      <div class="details">
        <div class="details-item">
          <div class="details-item-name">
            <ion-text class="details-item-name__title subtitles-sm">
              {{ $msTranslate('UsersPage.UserDetailsModal.subtitles.name') }}
            </ion-text>
            <ion-text class="details-item-name__text subtitles-normal">
              {{ user.humanHandle.label }}
            </ion-text>
          </div>
          <user-status-tag
            :active="user.isActive()"
            :revoked="user.isRevoked()"
            :frozen="user.isFrozen()"
            :show-tooltip="true"
          />
        </div>

        <div class="time-list">
          <!-- join on -->
          <div class="time-list-item">
            <ion-text class="time-list-item__title">
              <ion-icon
                class="time-list-item__icon body-lg"
                :icon="personAdd"
              />
              <span class="subtitles-sm">{{ $msTranslate('UsersPage.UserDetailsModal.subtitles.joined') }}</span>
            </ion-text>
            <ion-text class="time-list-item__text body-lg">
              {{ $msTranslate(formatTimeSince(user.createdOn, '--', 'short')) }}
            </ion-text>
          </div>

          <!-- revoked since -->
          <div
            class="time-list-item"
            v-if="user.isRevoked() && user.revokedOn"
          >
            <ion-text class="time-list-item__title">
              <ion-icon
                class="time-list-item__icon body-lg"
                :icon="personRemove"
              />
              <span class="subtitles-sm">{{ $msTranslate('UsersPage.UserDetailsModal.subtitles.revokedSince') }}</span>
            </ion-text>
            <ion-text class="time-list-item__text body-lg">
              {{ $msTranslate(I18n.formatDate(user.revokedOn, 'short')) }}
            </ion-text>
          </div>
        </div>
        <technical-id :id="user.id" />
      </div>
      <ion-list class="workspace">
        <ion-text class="subtitles-sm workspace-title">
          {{ $msTranslate('UsersPage.UserDetailsModal.subtitles.commonWorkspaces') }}
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
                {{ sharedWorkspace.workspace.currentName }}
              </ion-text>
              <workspace-role-tag
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
          {{ $msTranslate('UsersPage.UserDetailsModal.noSharedWorkspaces') }}
        </div>
      </ion-list>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { TechnicalId } from '@/components/misc';
import UserStatusTag from '@/components/users/UserStatusTag.vue';
import WorkspaceRoleTag from '@/components/workspaces/WorkspaceRoleTag.vue';
import { SharedWithInfo, UserInfo, getWorkspacesSharedWith } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonCard, IonCardContent, IonIcon, IonList, IonPage, IonText } from '@ionic/vue';
import { business, personAdd, personRemove } from 'ionicons/icons';
import { I18n, MsModal, formatTimeSince } from 'megashark-lib';
import { Ref, onMounted, ref } from 'vue';

const sharedWorkspaces: Ref<Array<SharedWithInfo>> = ref([]);

const props = defineProps<{
  user: UserInfo;
  informationManager: InformationManager;
}>();

onMounted(async () => {
  const result = await getWorkspacesSharedWith(props.user.id);

  if (result.ok) {
    sharedWorkspaces.value = result.value;
  } else {
    props.informationManager.present(
      new Information({
        message: 'UsersPage.UserDetailsModal.failedToListWorkspaces',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
});
</script>

<style lang="scss" scoped>
.details {
  display: flex;
  flex-direction: column;
  margin-bottom: 1rem;
  gap: 1rem;

  .details-item {
    display: flex;
    width: 100%;
    flex-direction: row;
    gap: 1rem;
    margin-bottom: 1rem;
    align-items: end;

    &-name {
      margin: auto 0;
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &__title {
        color: var(--parsec-color-light-secondary-text);
      }

      &__text {
        color: var(--parsec-color-light-primary-800);
      }
    }
  }

  .time-list {
    display: flex;
    gap: 1rem;

    &-item {
      background: var(--parsec-color-light-secondary-background);
      padding: 0.75rem;
      border: 1px solid var(--parsec-color-light-secondary-premiere);
      border-radius: var(--parsec-radius-6);
      width: 100%;
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &__title {
        color: var(--parsec-color-light-secondary-hard-grey);
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
      }

      &__icon {
        color: var(--parsec-color-light-secondary-hard-grey);
      }

      &__text {
        color: var(--parsec-color-light-primary-800);
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
      border: 1px solid var(--parsec-color-light-secondary-premiere);
      flex-shrink: 0;
      display: flex;
      align-items: center;
      padding: 1rem 1.5rem 1rem 1rem;
      margin: 0;
      border-radius: var(--parsec-radius-6);
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
