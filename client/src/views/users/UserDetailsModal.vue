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
        <div
          class="details-item"
          :class="{ revoked: user.isRevoked() }"
        >
          <div class="details-item-name">
            <ion-text class="details-item-name__title subtitles-sm">
              {{ $msTranslate('UsersPage.UserDetailsModal.subtitles.name') }}
            </ion-text>
            <ion-text class="details-item-name__text subtitles-normal">
              {{ user.humanHandle.label }}
            </ion-text>
          </div>
          <ion-chip
            v-if="user.isRevoked()"
            color="danger"
            class="revoked-chip"
          >
            <ion-label class="subtitles-sm">
              {{ $msTranslate('UsersPage.UserDetailsModal.subtitles.revoked') }}
            </ion-label>
          </ion-chip>
        </div>

        <!-- join on -->
        <div class="details-item time-item">
          <ion-text class="details-item-name__title subtitles-sm">
            <ion-icon
              class="details-item__icon"
              :icon="personAdd"
            />
            {{ $msTranslate('UsersPage.UserDetailsModal.subtitles.joined') }}
          </ion-text>
          <ion-text class="details-item-name__text body-lg">
            {{ $msTranslate(formatTimeSince(user.createdOn, '--', 'short')) }}
          </ion-text>
        </div>

        <!-- revoked since -->
        <div
          class="details-item time-item"
          v-if="user.isRevoked() && user.revokedOn"
        >
          <ion-text class="details-item-name__title">
            <ion-icon
              class="details-item__icon body-lg"
              :icon="personRemove"
            />
            <span class="subtitles-sm">{{ $msTranslate('UsersPage.UserDetailsModal.subtitles.revokedSince') }}</span>
          </ion-text>
          <ion-text class="details-item-name__text body-lg">
            {{ $msTranslate(I18n.formatDate(user.revokedOn, 'short')) }}
          </ion-text>
        </div>
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
          {{ $msTranslate('UsersPage.UserDetailsModal.noSharedWorkspaces') }}
        </div>
      </ion-list>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { MsModal, formatTimeSince, I18n } from 'megashark-lib';
import WorkspaceTagRole from '@/components/workspaces/WorkspaceTagRole.vue';
import { SharedWithInfo, UserInfo, getWorkspacesSharedWith } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonCard, IonCardContent, IonChip, IonIcon, IonLabel, IonList, IonPage, IonText } from '@ionic/vue';
import { business, personAdd, personRemove } from 'ionicons/icons';
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
  margin-bottom: 1rem;
  flex-wrap: wrap;
  gap: 1rem;

  .details-item {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    width: calc(50% - 0.5rem);
    gap: 0.5rem;

    &.revoked {
      width: 100%;
      flex-direction: row;
      gap: 1rem;
      margin-bottom: 1rem;
    }

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

    .revoked-chip {
      display: flex;
      gap: 0.5rem;
      align-items: center;
      align-self: end;
      justify-content: center;
      width: 5.5rem;
      padding: 0.125rem;
      min-height: 0;
      border-radius: var(--parsec-radius-6);
      margin: 0;
      height: fit-content;
    }

    &.time-item {
      background: var(--parsec-color-light-secondary-background);
      padding: 0.75rem;
      border: 1px solid var(--parsec-color-light-secondary-premiere);
      border-radius: var(--parsec-radius-6);

      .details-item__icon {
        color: var(--parsec-color-light-secondary-hard-grey);
      }

      .details-item-name__title {
        color: var(--parsec-color-light-secondary-hard-grey);
        display: flex;
        align-items: center;
        gap: 0.5rem;
      }

      .details-item-name__text {
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
