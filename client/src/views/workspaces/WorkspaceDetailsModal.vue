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
            <ion-text class="workspace-header-text__name">
              {{ workspace.name }}
            </ion-text>
            <ion-text
              v-if="false"
              class="workspace-header-text__status"
            >
              {{ $msTranslate(workspace.availableOffline ? 'WorkspaceDetails.availableOffline' : 'WorkspaceDetails.notAvailableOffline') }}
            </ion-text>
          </div>
        </div>

        <div class="workspace-section">
          <ion-text class="workspace-section__title">{{ $msTranslate('WorkspaceDetails.generalInfo') }}</ion-text>
          <div class="workspace-info">
            <!-- Role -->
            <div class="workspace-info-item">
              <ion-label class="workspace-info-item__title">
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
              <ion-label class="workspace-info-item__title">
                {{ $msTranslate('WorkspaceDetails.created') }}
              </ion-label>
              <ion-text class="workspace-info-item__value">
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
              <ion-text class="workspace-rights-item__text">
                {{ $msTranslate(right) }}
              </ion-text>
            </li>
          </ul>
          <technical-id :id="workspace.id" />
        </div>
        <div class="workspace-section">
          <ion-text class="workspace-section__title">{{ $msTranslate('export.exportWorkspaceTitle') }}</ion-text>
          <div class="workspace-export">
            <ion-text class="workspace-export__description">
              {{ $msTranslate('export.exportWorkspaceDescription') }}
            </ion-text>
            <ion-button
              class="workspace-export__button"
              @click="downloadWorkspaceExport"
              v-show="workspace.selfRole === RealmRole.Owner"
              fill="clear"
            >
              {{ $msTranslate('export.exportWorkspace') }}
            </ion-button>
          </div>
        </div>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { TechnicalId } from '@/components/misc';
import { WorkspaceRoleTag } from '@/components/workspaces';
import { WorkspaceInfo, WorkspaceRole } from '@/parsec';
import { RealmRole } from '@/plugins/libparsec';
import { exportWorkspace } from '@/services/export';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonButton, IonIcon, IonLabel, IonPage, IonText } from '@ionic/vue';
import { business, checkmark } from 'ionicons/icons';
import { I18n, MsModal, openSpinnerModal, Translatable } from 'megashark-lib';
import { computed, Ref } from 'vue';

const props = defineProps<{
  workspace: WorkspaceInfo;
  informationManager: InformationManager;
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

async function downloadWorkspaceExport(): Promise<void> {
  const modal = await openSpinnerModal('export.exporting');
  const result = await exportWorkspace(props.workspace.handle);
  if (!result) {
    props.informationManager.present(
      new Information({
        message: 'export.exportWorkspaceFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  modal.dismiss();
}
</script>

<style scoped lang="scss">
.workspace-details {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');

  @include ms.responsive-breakpoint('sm') {
    margin-bottom: 2rem;
  }
}

.workspace-header {
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
    gap: ms.spacing('gap-xs');
    min-width: 0;

    &__name {
      @include ms.font('heading-h5');

      color: ms.color('text-base-heading');
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &__status {
      @include ms.font('body-md-regular');

      color: ms.color('text-base-description');
    }
  }
}

.workspace-section {
  display: flex;
  flex-direction: column;

  &__title {
    @include ms.font('body-md-medium');

    color: ms.color('text-base-body');
    margin-bottom: 0.5rem;
  }
}

.workspace-rights {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');
  margin: 0;
  padding: ms.spacing('padding-xl');
  list-style: none;
  border-right: ms.border('thin') solid ms.color('border-base-default');
  border-bottom: ms.border('thin') solid ms.color('border-base-default');
  border-left: ms.border('thin') solid ms.color('border-base-default');
  border-radius: 0 0 ms.radius('lg') ms.radius('lg');
  margin-bottom: 0.75rem;

  &-item {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-sm');

    &__icon {
      flex-shrink: 0;
      font-size: 0.875rem;
      color: ms.color('icon-brand-default');
    }

    &__text {
      @include ms.font('body-sm-regular');

      color: ms.color('text-base-body');
    }
  }
}

.workspace-info {
  display: flex;
  overflow: hidden;
  border: ms.border('thin') solid ms.color('border-base-default');
  border-radius: ms.radius('lg') ms.radius('lg') 0 0;

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

    &__role {
      align-self: flex-start;
      background: ms.color('surface-base-page');
    }

    &__value {
      @include ms.font('body-md-regular');

      color: ms.color('text-base-body');
    }
  }
}

.workspace-export {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: ms.spacing('gap-3xl');
  padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl');
  border-radius: ms.radius('lg');
  background: ms.color('surface-base-default-secondary');

  &__description {
    @include ms.font('body-md-regular');

    color: ms.color('text-base-body');
  }

  &__button {
    @include ms.font('label-md-medium');

    --background: #{ms.color('surface-base-default')};
    --background-hover: #{ms.color('surface-base-default-secondary')};
    --color: #{ms.color('text-neutral-default')};
    --border-radius: #{ms.radius('lg')};
    --padding-top: #{ms.spacing('padding-xl')};
    --padding-bottom: #{ms.spacing('padding-xl')};
    --padding-start: #{ms.spacing('padding-2xl')};
    --padding-end: #{ms.spacing('padding-2xl')};
    min-width: 5rem;
    min-height: 0;
    margin: 0;
    border: ms.border('thin') solid ms.color('border-base-default');
    border-radius: ms.radius('lg');
  }
}
</style>
