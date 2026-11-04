<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="'UsersPage.UserDetailsModal.title'"
      :close-button="{ visible: true }"
    >
      <div class="user-details">
        <div class="user-header">
          <div class="user-header__icon">
            <ion-icon :icon="person" />
          </div>
          <div class="user-header-text">
            <ion-text class="user-header-text__name">
              {{ user.humanHandle.label }}
            </ion-text>
            <user-status-tag
              class="user-header-text__status"
              :active="user.isActive()"
              :revoked="user.isRevoked()"
              :frozen="user.isFrozen()"
              :show-tooltip="true"
            />
          </div>
        </div>

        <div class="user-section">
          <ion-text class="user-section__title">{{ $msTranslate('UsersPage.UserDetailsModal.subtitles.generalInfo') }}</ion-text>
          <div class="user-info">
            <!-- Joined on -->
            <div class="user-info-item">
              <ion-label class="user-info-item__title">
                {{ $msTranslate('UsersPage.UserDetailsModal.subtitles.joined') }}
              </ion-label>
              <ion-text class="user-info-item__value">
                {{ $msTranslate(I18n.formatDate(user.createdOn, 'full')) }}
              </ion-text>
            </div>
            <!-- Revoked since -->
            <div
              class="user-info-item"
              v-if="user.isRevoked() && user.revokedOn"
            >
              <ion-label class="user-info-item__title">
                {{ $msTranslate('UsersPage.UserDetailsModal.subtitles.revokedSince') }}
              </ion-label>
              <ion-text class="user-info-item__value">
                {{ $msTranslate(I18n.formatDate(user.revokedOn, 'full')) }}
              </ion-text>
            </div>
          </div>
          <technical-id :id="user.id" />
        </div>

        <div class="user-section">
          <ion-text class="user-section__title">{{ $msTranslate('UsersPage.UserDetailsModal.subtitles.commonWorkspaces') }}</ion-text>
          <ul
            class="workspace-list"
            v-show="sharedWorkspaces.length > 0"
          >
            <li
              v-for="sharedWorkspace in sharedWorkspaces"
              :key="sharedWorkspace.workspace.id"
              class="workspace-list-item"
              :class="{ 'workspace-list-item--disabled': user.isRevoked() }"
            >
              <ion-icon
                class="workspace-list-item__icon"
                :icon="business"
              />
              <ion-text class="workspace-list-item__name">
                {{ sharedWorkspace.workspace.name }}
              </ion-text>
              <workspace-role-tag
                class="workspace-list-item__role"
                :role="sharedWorkspace.userRole"
              />
            </li>
          </ul>
          <ion-text
            class="workspace-empty"
            v-show="sharedWorkspaces.length === 0"
          >
            {{ $msTranslate('UsersPage.UserDetailsModal.noSharedWorkspaces') }}
          </ion-text>
        </div>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { TechnicalId } from '@/components/misc';
import UserStatusTag from '@/components/users/UserStatusTag.vue';
import WorkspaceRoleTag from '@/components/workspaces/WorkspaceRoleTag.vue';
import { SharedWithInfo, UserInfo, getWorkspacesSharedWith } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonIcon, IonLabel, IonPage, IonText } from '@ionic/vue';
import { business, person } from 'ionicons/icons';
import { I18n, MsModal } from 'megashark-lib';
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
.user-details {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');

  @include ms.responsive-breakpoint('sm') {
    margin-bottom: 2rem;
  }
}

.user-header {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-2xl');

  &__icon {
    display: flex;
    padding: ms.spacing('padding-lg');
    border-radius: ms.radius('lg');
    background: ms.color('surface-brand-default-subtle');
    flex-shrink: 0;

    ion-icon {
      font-size: 1.5rem;
      color: ms.color('icon-brand-default');
    }
  }

  &-text {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: ms.spacing('gap-xs');
    min-width: 0;

    &__name {
      @include ms.font('heading-h5');

      max-width: 100%;
      color: ms.color('text-base-heading');
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }
}

.user-section {
  display: flex;
  flex-direction: column;

  &__title {
    @include ms.font('body-md-medium');

    color: ms.color('text-base-body');
    margin-bottom: 0.5rem;
  }
}

.user-info {
  display: flex;
  overflow: hidden;
  border: ms.border('thin') solid ms.color('border-base-default');
  border-radius: ms.radius('lg');
  margin-bottom: 0.75rem;

  &-item {
    display: flex;
    flex-direction: column;
    flex: 1 0 0;
    gap: ms.spacing('gap-lg');
    padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
    background: ms.color('surface-base-default-secondary');

    &:not(:last-child) {
      border-right: ms.border('thin') solid ms.color('border-base-default');
    }

    &__title {
      @include ms.font('body-md-medium');

      color: ms.color('text-base-description');
    }

    &__value {
      @include ms.font('body-md-regular');

      color: ms.color('text-base-body');
    }
  }
}

.workspace-list {
  display: flex;
  flex-direction: column;
  margin: 0;
  padding: ms.spacing('padding-none');
  list-style: none;
  overflow: hidden;
  border: ms.border('thin') solid ms.color('border-base-default');
  border-radius: ms.radius('lg');

  &-item {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
    background: ms.color('surface-base-default-secondary');

    &:not(:last-child) {
      border-bottom: ms.border('thin') solid ms.color('border-base-default');
    }

    &--disabled {
      opacity: ms.opacity('disabled');
    }

    &__icon {
      flex-shrink: 0;
      font-size: 1.25rem;
      color: ms.color('icon-brand-default');
    }

    &__name {
      @include ms.font('body-md-medium');

      flex: 1;
      min-width: 0;
      color: ms.color('text-base-body');
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &__role {
      flex-shrink: 0;
      background: ms.color('surface-base-page');
    }
  }
}

.workspace-empty {
  @include ms.font('body-md-regular');

  padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
  border-radius: ms.radius('lg');
  background: ms.color('surface-base-default-secondary');
  color: ms.color('text-base-description');
}
</style>
