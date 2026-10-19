<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="WorkspaceDetails.title"
      :close-button="{ visible: true }"
    >
      <div class="workspace-details">
        <div class="workspace-header">
          <div class="workspace-header__icon">
            <ion-icon :icon="business" />
          </div>
          <div class="workspace-header-text">
            <ion-text class="workspace-header-text__name title-h4">
              {{ workspace.name }}
            </ion-text>
            <ion-text
              v-if="false"
              class="workspace-header-text__status body"
            >
              {{ $msTranslate(workspace.availableOffline ? 'WorkspaceDetails.availableOffline' : 'WorkspaceDetails.notAvailableOffline') }}
            </ion-text>
          </div>
        </div>

        <div class="workspace-section">
          <ion-text class="workspace-section__title subtitles-sm">{{ $msTranslate('WorkspaceDetails.generalInfo') }}</ion-text>
          <div class="workspace-info">
            <!-- Role -->
            <div class="workspace-info-item">
              <ion-label class="workspace-info-item__title subtitles-sm">
                {{ $msTranslate('WorkspaceDetails.role') }}
              </ion-label>
              <workspace-role-tag
                class="workspace-info-item__role"
                :role="workspace.selfRole"
              />
            </div>
            <!-- Creation date -->
            <div
              class="workspace-info-item"
              v-if="workspace.created"
            >
              <ion-label class="workspace-info-item__title subtitles-sm">
                {{ $msTranslate('WorkspaceDetails.created') }}
              </ion-label>
              <ion-text class="workspace-info-item__value body">
                {{ $msTranslate(I18n.formatDate(workspace.created, 'full')) }}
              </ion-text>
            </div>
          </div>
          <ul class="workspace-rights">
            <li
              v-for="right in rights"
              :key="JSON.stringify(right)"
              class="workspace-rights-item"
            >
              <ion-icon
                class="workspace-rights-item__icon"
                :icon="checkmark"
              />
              <ion-text class="workspace-rights-item__text body-sm">
                {{ $msTranslate(right) }}
              </ion-text>
            </li>
          </ul>
          <technical-id :id="workspace.id" />
        </div>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { TechnicalId } from '@/components/misc';
import { WorkspaceRoleTag } from '@/components/workspaces';
import { WorkspaceInfo, WorkspaceRole } from '@/parsec';
import { IonIcon, IonLabel, IonPage, IonText } from '@ionic/vue';
import { business, checkmark } from 'ionicons/icons';
import { I18n, MsModal, Translatable } from 'megashark-lib';
import { computed, Ref } from 'vue';

const props = defineProps<{
  workspace: WorkspaceInfo;
}>();

const rights: Ref<Array<Translatable>> = computed(() => {
  switch (props.workspace.selfRole) {
    case WorkspaceRole.Owner:
      return [
        'WorkspaceDetails.rights.manageWorkspace',
        'WorkspaceDetails.rights.share',
        'WorkspaceDetails.rights.editDocuments',
        'WorkspaceDetails.rights.viewDocuments',
      ];
    case WorkspaceRole.Manager:
      return ['WorkspaceDetails.rights.share', 'WorkspaceDetails.rights.editDocuments', 'WorkspaceDetails.rights.viewDocuments'];
    case WorkspaceRole.Contributor:
      return ['WorkspaceDetails.rights.editDocuments', 'WorkspaceDetails.rights.viewDocuments'];
    case WorkspaceRole.Reader:
    default:
      return ['WorkspaceDetails.rights.viewDocuments'];
  }
});
</script>

<style scoped lang="scss">
.workspace-details {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;

  @include ms.responsive-breakpoint('sm') {
    margin-bottom: 2rem;
  }
}

.workspace-header {
  display: flex;
  align-items: center;
  gap: 0.75rem;

  &__icon {
    display: flex;
    padding: 0.5rem;
    border-radius: var(--parsec-radius-8);
    background: var(--parsec-color-light-primary-50);
    flex-shrink: 0;

    ion-icon {
      font-size: 1.5rem;
      color: var(--parsec-color-light-primary-500);
    }
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    min-width: 0;

    &__name {
      color: var(--parsec-color-light-secondary-black);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &__status {
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}

.workspace-section {
  display: flex;
  flex-direction: column;

  &__title {
    color: var(--parsec-color-light-secondary-text);
    margin-bottom: 0.5rem;
  }
}

.workspace-rights {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin: 0;
  padding: 0.625rem;
  list-style: none;
  border-right: 1px solid var(--parsec-color-light-secondary-medium);
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  border-left: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: 0 0 var(--parsec-radius-8) var(--parsec-radius-8);
  margin-bottom: 0.75rem;

  &-item {
    display: flex;
    align-items: center;
    gap: 0.25rem;

    &__icon {
      flex-shrink: 0;
      font-size: 0.875rem;
      color: var(--parsec-color-light-primary-500);
    }

    &__text {
      color: var(--parsec-color-light-secondary-text);
    }
  }
}

.workspace-info {
  display: flex;
  overflow: hidden;
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-8) var(--parsec-radius-8) 0 0;

  &-item {
    display: flex;
    flex-direction: column;
    flex: 1 0 0;
    gap: 0.5rem;
    padding: 0.625rem 1rem;
    background: var(--parsec-color-light-secondary-premiere);

    &:not(:last-child) {
      border-right: 1px solid var(--parsec-color-light-secondary-disabled);
    }

    &__title {
      color: var(--parsec-color-light-secondary-grey);
    }

    &__role {
      align-self: flex-start;
      background: var(--parsec-color-light-secondary-background);
    }

    &__value {
      color: var(--parsec-color-light-secondary-text);
    }
  }
}
</style>
