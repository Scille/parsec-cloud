<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="content-scroll"
    >
      <div class="workspaces-container scroll">
        <div
          v-show="querying && filteredWorkspaces.length === 0"
          class="no-workspaces-loading"
        >
          <ms-spinner :title="'WorkspacesPage.loading'" />
        </div>

        <div
          v-if="!querying && filteredWorkspaces.length === 0"
          class="no-workspaces body-lg"
        >
          <div
            class="no-archived-workspaces"
            v-if="filteredWorkspaces.length === 0"
          >
            <ms-image
              :image="NoHiddenWorkspaces"
              class="no-workspaces__image"
            />
            <ion-text>
              {{ $msTranslate('WorkspacesPage.categoriesMenu.noArchivedWorkspaces') }}
            </ion-text>
          </div>
        </div>

        <div
          v-if="filteredWorkspaces.length > 0"
          class="workspaces-container-grid list"
        >
          <workspace-card
            v-for="workspace in filteredWorkspaces"
            :key="workspace.id"
            :workspace="workspace"
            :client-profile="clientProfile"
            :is-favorite="workspaceAttributes.isFavorite(workspace.id)"
            :is-hidden="workspaceAttributes.isHidden(workspace.id)"
            :is-archived="workspace.archivingConfiguration.tag === RealmArchivingConfigurationTag.Archived"
            @click="onWorkspaceClick"
            @menu-click="onOpenWorkspaceContextMenu"
          />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import NoHiddenWorkspaces from '@/assets/images/no-hidden-workspaces.svg?raw';
import { openWorkspaceContextMenu } from '@/components/workspaces';
import WorkspaceCard from '@/components/workspaces/WorkspaceCard.vue';
import { listWorkspaces, UserProfile, WorkspaceInfo } from '@/parsec';
import { RealmArchivingConfigurationTag } from '@/plugins/libparsec';
import { currentRouteIs, navigateToWorkspace, Routes } from '@/router';
import { EventDistributor, EventDistributorKey } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { useWorkspaceAttributes } from '@/services/workspaceAttributes';
import { IonContent, IonPage, IonText } from '@ionic/vue';
import { MsImage, MsSpinner, useWindowSize } from 'megashark-lib';
import { computed, inject, onMounted, Ref, ref } from 'vue';

const querying = ref(true);
const workspaceList: Ref<Array<WorkspaceInfo>> = ref([]);
const workspaceAttributes = useWorkspaceAttributes();
const { isLargeDisplay } = useWindowSize();
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;

// clientProfile is required as a WorkspaceCard prop but only to show or hide
// the share button, we can leave it as Outsider
const clientProfile: Ref<UserProfile> = ref(UserProfile.Outsider);

const filteredWorkspaces = computed(() => {
  return Array.from(workspaceList.value)
    .filter((workspace) => {
      return workspace.archivingConfiguration.tag === RealmArchivingConfigurationTag.Archived;
    })
    .sort((a: WorkspaceInfo, b: WorkspaceInfo) => a.currentName.localeCompare(b.currentName));
});

onMounted(async (): Promise<void> => {
  await workspaceAttributes.load();
  await refreshArchivedWorkspacesList();
});

async function refreshArchivedWorkspacesList(): Promise<void> {
  if (!currentRouteIs(Routes.Archived)) {
    return;
  }
  querying.value = true;
  window.electronAPI.log('debug', 'Starting Parsec list workspaces');
  const result = await listWorkspaces();
  if (result.ok) {
    workspaceList.value = result.value;
  } else {
    informationManager.value.present(
      new Information({
        message: 'WorkspacesPage.listError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  querying.value = false;
}

async function onWorkspaceClick(workspace: WorkspaceInfo): Promise<void> {
  await navigateToWorkspace(workspace.handle);
}

async function onOpenWorkspaceContextMenu(workspace: WorkspaceInfo, event: Event, onFinished?: () => void): Promise<void> {
  await openWorkspaceContextMenu(
    event,
    workspace,
    workspaceAttributes,
    eventDistributor.value,
    informationManager.value,
    storageManager,
    false,
    isLargeDisplay.value,
  );
  await refreshArchivedWorkspacesList();

  if (onFinished) {
    onFinished();
  }
}
</script>

<style lang="scss" scoped>
.content-scroll {
  &::part(background) {
    @include ms.responsive-breakpoint('sm') {
      background: var(--parsec-color-light-secondary-background);
    }
  }

  &::part(scroll) {
    @include ms.responsive-breakpoint('sm') {
      padding-top: 1rem;
    }
  }
}

.workspaces-container {
  @include ms.responsive-breakpoint('sm') {
    position: sticky;
    z-index: 10;
    background: var(--parsec-color-light-secondary-white);
    box-shadow: var(--parsec-shadow-strong);
    border-radius: var(--parsec-radius-18) var(--parsec-radius-18) 0 0;
  }
}

.no-workspaces {
  max-width: 30rem;
  color: var(--parsec-color-light-secondary-soft-text);
  display: flex;
  justify-content: center;
  margin: auto;
  height: 100%;
  align-items: center;

  @include ms.responsive-breakpoint('xs') {
    align-items: start;
    height: fit-content;
  }

  &-loading,
  .no-archived-workspaces {
    border-radius: var(--parsec-radius-8);
    display: flex;
    height: fit-content;
    text-align: center;
    flex-direction: column;
    justify-content: center;
    gap: 1.5rem;
    align-items: center;
    padding: 3rem 1rem;

    #new-workspace {
      display: flex;
      align-items: center;

      ion-icon {
        margin-inline: 0em;
        margin-right: 0.375rem;
      }
    }
  }

  &__image {
    width: 8rem;
    height: 8rem;

    @include ms.responsive-breakpoint('xs') {
      width: 6rem;
      height: 6rem;
    }
  }

  .no-all-workspaces .no-workspaces__image {
    width: 12.5rem;
    height: 12.5rem;
  }
}
</style>
