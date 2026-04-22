<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="workspace-trash-page">
    <ion-content class="content-scroll">
      <div
        class="workspace-trash-info"
        v-if="filteredWorkspaces.length > 0 && !querying"
      >
        <ion-text class="workspace-trash-info__title subtitles-normal">
          {{ $msTranslate('WorkspacesPage.trashWorkspace.info') }}
        </ion-text>
        <ion-text class="workspace-trash-info__count body">
          {{
            $msTranslate({
              key: 'WorkspacesPage.trashWorkspace.count',
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
          :title="'WorkspacesPage.trashWorkspace.loading'"
        />

        <div
          class="no-trashed-workspaces"
          v-if="!querying && filteredWorkspaces.length === 0"
        >
          <ms-image
            :image="NoTrashedWorkspaces"
            class="no-trashed-workspaces__image"
          />
          <ion-text class="body-lg">
            {{ $msTranslate('WorkspacesPage.categoriesMenu.noTrashedWorkspaces') }}
          </ion-text>
          <ion-button
            class="no-trashed-workspaces__button button-medium button-default"
            size="default"
            fill="solid"
            @click="backToAllWorkspaces"
          >
            <ion-icon
              :icon="arrowBack"
              class="button-icon"
            />
            {{ $msTranslate('WorkspacesPage.trashWorkspace.backToWorkspace') }}
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
import NoTrashedWorkspaces from '@/assets/images/no-trashed-workspaces.svg?raw';
import { openArchivedWorkspaceContextMenu } from '@/components/workspaces/utils';
import WorkspaceCard from '@/components/workspaces/WorkspaceCard.vue';
import { listTrashedWorkspaces, UserProfile, WorkspaceInfo } from '@/parsec';
import { currentRouteIs, navigateTo, navigateToWorkspace, Routes } from '@/router';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { IonButton, IonContent, IonIcon, IonPage, IonText } from '@ionic/vue';
import { arrowBack } from 'ionicons/icons';
import { MsImage, MsSpinner, useWindowSize } from 'megashark-lib';
import { computed, inject, onMounted, Ref, ref } from 'vue';

const querying = ref(true);
const workspaceList: Ref<Array<WorkspaceInfo>> = ref([]);
const { isLargeDisplay } = useWindowSize();
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;

const filteredWorkspaces = computed(() => {
  return Array.from(workspaceList.value).sort((a: WorkspaceInfo, b: WorkspaceInfo) => a.name.localeCompare(b.name));
});

onMounted(async (): Promise<void> => {
  await refreshTrashedWorkspacesList();
});

async function refreshTrashedWorkspacesList(): Promise<void> {
  if (!currentRouteIs(Routes.Trash)) {
    return;
  }
  querying.value = true;
  window.electronAPI.log('debug', 'Starting Parsec list workspaces');
  const result = await listTrashedWorkspaces();
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
  await openArchivedWorkspaceContextMenu(event, workspace, informationManager.value, false, isLargeDisplay.value);
  await refreshTrashedWorkspacesList();

  if (onFinished) {
    onFinished();
  }
}

async function backToAllWorkspaces(): Promise<void> {
  await navigateTo(Routes.Workspaces);
}
</script>

<style lang="scss" scoped>
.workspace-trash-page {
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

.workspace-trash-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1.5rem;
  min-height: 3rem;
  background: var(--parsec-color-light-secondary-white);
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  gap: 1rem;

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem 1.5rem 1rem 1.5rem;
    flex-direction: column;
    align-items: flex-start;
    min-height: fit-content;
  }

  &__title,
  &__count {
    color: var(--parsec-color-light-secondary-soft-text);
  }

  &__count {
    flex-shrink: 0;
  }

  .workspace-trash-count {
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

.no-trashed-workspaces {
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
