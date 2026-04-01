<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="workspace-archive-page">
    <ion-content class="content-scroll">
      <div
        class="workspace-archive-info"
        v-if="filteredWorkspaces.length > 0 && !querying"
      >
        <ion-text class="workspace-archive-info__title subtitles-normal">
          {{ $msTranslate('WorkspacesPage.archiveWorkspace.info') }}
        </ion-text>
        <ion-text class="workspace-archive-info__count body">
          {{
            $msTranslate({
              key: 'WorkspacesPage.archiveWorkspace.count',
              data: { count: filteredWorkspaces.length },
              count: filteredWorkspaces.length,
            })
          }}
        </ion-text>
      </div>

      <div class="workspaces-container scroll">
        <ms-spinner
          class="workspaces-loading"
          v-if="querying && filteredWorkspaces.length === 0"
          :title="'WorkspacesPage.archiveWorkspace.loading'"
        />

        <div
          class="no-archived-workspaces"
          v-if="!querying && filteredWorkspaces.length === 0"
        >
          <ms-image
            :image="NoHiddenWorkspaces"
            class="no-archived-workspaces__image"
          />
          <ion-text class="body-lg">
            {{ $msTranslate('WorkspacesPage.categoriesMenu.noArchivedWorkspaces') }}
          </ion-text>
          <ion-button
            class="no-archived-workspaces__button button-medium button-default"
            size="default"
            fill="solid"
            @click="backToAllWorkspaces"
          >
            <ion-icon
              :icon="arrowBack"
              class="button-icon"
            />
            {{ $msTranslate('WorkspacesPage.archiveWorkspace.backToWorkspace') }}
          </ion-button>
        </div>

        <div
          v-if="filteredWorkspaces.length > 0"
          class="workspaces-container-grid list"
        >
          <workspace-card
            v-for="workspace in filteredWorkspaces"
            :key="workspace.id"
            :workspace="workspace"
            :client-profile="UserProfile.Outsider"
            :is-favorite="false"
            :is-hidden="false"
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
import { openArchivedWorkspaceContextMenu } from '@/components/workspaces/utils';
import WorkspaceCard from '@/components/workspaces/WorkspaceCard.vue';
import { listArchivedWorkspaces, UserProfile, WorkspaceInfo } from '@/parsec';
import { currentRouteIs, navigateTo, navigateToWorkspace, Routes } from '@/router';
import { EventDistributor, EventDistributorKey } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { IonButton, IonContent, IonIcon, IonPage, IonText } from '@ionic/vue';
import { arrowBack } from 'ionicons/icons';
import { MsImage, MsSpinner, useWindowSize } from 'megashark-lib';
import { computed, inject, onMounted, Ref, ref } from 'vue';

const querying = ref(true);
const workspaceList: Ref<Array<WorkspaceInfo>> = ref([]);
const { isLargeDisplay } = useWindowSize();
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;

const filteredWorkspaces = computed(() => {
  return Array.from(workspaceList.value).sort((a: WorkspaceInfo, b: WorkspaceInfo) => a.currentName.localeCompare(b.currentName));
});

onMounted(async (): Promise<void> => {
  await refreshArchivedWorkspacesList();
});

async function refreshArchivedWorkspacesList(): Promise<void> {
  if (!currentRouteIs(Routes.Archived)) {
    return;
  }
  querying.value = true;
  window.electronAPI.log('debug', 'Starting Parsec list workspaces');
  const result = await listArchivedWorkspaces();
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
  await openArchivedWorkspaceContextMenu(event, workspace, eventDistributor.value, informationManager.value, false, isLargeDisplay.value);
  await refreshArchivedWorkspacesList();

  if (onFinished) {
    onFinished();
  }
}

async function backToAllWorkspaces(): Promise<void> {
  await navigateTo(Routes.Workspaces);
}
</script>

<style lang="scss" scoped>
.workspace-archive-page {
  padding: 1.5rem;
  background: var(--parsec-color-light-secondary-background);

  @include ms.responsive-breakpoint('sm') {
    padding: 1rem 0 0 0;
  }

  .content-scroll {
    display: flex;
    border-radius: var(--parsec-radius-12);
    overflow: hidden;
    box-shadow: var(--parsec-shadow-soft);

    @include ms.responsive-breakpoint('sm') {
      box-shadow: var(--parsec-shadow-strong);
      border-radius: var(--parsec-radius-12) var(--parsec-radius-12) 0 0;
    }
  }
}

.workspace-archive-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1.5rem;
  min-height: 3rem;
  background: var(--parsec-color-light-secondary-white);
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  gap: 1rem;

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem;
  }

  &__title,
  &__count {
    color: var(--parsec-color-light-secondary-soft-text);
  }

  &__count {
    flex-shrink: 0;
  }

  .workspace-archive-count {
    color: var(--parsec-color-light-secondary-grey);
  }
}

.workspaces-container {
  display: flex;
  flex-direction: column;

  @include ms.responsive-breakpoint('sm') {
    position: sticky;
    z-index: 10;
    padding: 1rem;
  }

  &-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }
}

.workspaces-loading {
  border-radius: var(--parsec-radius-8);
  display: flex;
  height: 100%;
  text-align: center;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1.5rem;

  @include ms.responsive-breakpoint('xs') {
    height: auto;
  }
}

.no-archived-workspaces {
  display: flex;
  height: fit-content;
  text-align: center;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 1.5rem;
  max-width: 30rem;
  height: 100%;
  color: var(--parsec-color-light-secondary-soft-text);
  margin: auto;
  padding: 3rem 1rem;

  &__image {
    width: 8rem;
    height: 8rem;

    @include ms.responsive-breakpoint('xs') {
      width: 6rem;
      height: 6rem;
    }
  }

  &__button {
    &::part(native) {
      background: var(--parsec-color-light-secondary-premiere);
      --background-hover: var(--parsec-color-light-secondary-medium);
      color: var(--parsec-color-light-secondary-text);
      border: 1px solid var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-8);
      --box-shadow: var(--parsec-shadow-input);
    }

    .button-icon {
      font-size: 1rem;
      margin-right: 0.5rem;
    }
  }
}
</style>
