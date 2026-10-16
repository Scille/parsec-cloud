<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="workspace-archive-page">
    <ion-content class="content-scroll">
      <div
        class="workspace-archive-info"
        v-if="filteredWorkspaces.length > 0 && !querying"
      >
        <ion-text class="workspace-archive-info__title">
          {{ $msTranslate('WorkspacesPage.archiveWorkspace.info') }}
        </ion-text>
        <ion-text class="workspace-archive-info__count">
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
            :image="isDarkTheme ? NoArchivedWorkspaceDark : NoArchivedWorkspace"
            class="no-archived-workspaces__image"
          />
          <ion-text class="no-archived-workspaces__text">
            {{ $msTranslate('WorkspacesPage.categoriesMenu.noArchivedWorkspaces') }}
          </ion-text>
          <ion-button
            class="no-archived-workspaces__button color-secondary"
            fill="outline"
            @click="backToAllWorkspaces"
          >
            <ion-icon
              :icon="arrowBack"
              class="button-icon-left"
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
import WorkspaceCard from '@/components/workspaces/WorkspaceCard.vue';
import { listArchivedWorkspaces, UserProfile, WorkspaceInfo } from '@/parsec';
import { currentRouteIs, navigateTo, navigateToWorkspace, Routes } from '@/router';
import { useWorkspaceContextMenu } from '@/services/contextMenu';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { IonButton, IonContent, IonIcon, IonPage, IonText } from '@ionic/vue';
import { arrowBack } from 'ionicons/icons';
import { MsImage, MsSpinner, NoArchivedWorkspace, NoArchivedWorkspaceDark, useTheme } from 'megashark-lib';
import { computed, inject, onMounted, onUnmounted, Ref, ref } from 'vue';

const contextMenu = useWorkspaceContextMenu(false);
const { isDarkTheme } = useTheme();
const querying = ref(true);
const workspaceList: Ref<Array<WorkspaceInfo>> = ref([]);
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
let eventCbId: string | null = null;

const filteredWorkspaces = computed(() => {
  return Array.from(workspaceList.value).sort((a: WorkspaceInfo, b: WorkspaceInfo) => a.name.localeCompare(b.name));
});

onMounted(async (): Promise<void> => {
  await refreshArchivedWorkspacesList();
  eventCbId = await eventDistributor.value.registerCallback([Events.WorkspaceUpdated], refreshArchivedWorkspacesList);
});

onUnmounted(async () => {
  if (eventCbId) {
    eventDistributor.value.removeCallback(eventCbId);
  }
});

async function refreshArchivedWorkspacesList(): Promise<void> {
  if (!currentRouteIs(Routes.Archived)) {
    return;
  }
  querying.value = true;
  window.nativeAPI.log('debug', 'Starting Parsec list archived workspaces');
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
  await contextMenu.openArchivedContextMenu(event, workspace);
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
  padding: ms.spacing('padding-none') ms.spacing('padding-4xl') ms.spacing('padding-4xl');
  background: ms.color('surface-base-page');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-3xl') ms.spacing('padding-none') ms.spacing('padding-none') ms.spacing('padding-none');
  }

  .content-scroll {
    display: flex;
    border-radius: ms.radius('2xl');
    overflow: hidden;
    box-shadow: ms.shadow('light');

    @include ms.responsive-breakpoint('sm') {
      box-shadow: ms.shadow('strong');
      border-radius: ms.radius('2xl') ms.radius('2xl') 0 0;
    }
  }
}

.workspace-archive-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: ms.spacing('padding-2xl') ms.spacing('padding-4xl');
  min-height: 3rem;
  background: ms.color('surface-base-default');
  border-bottom: ms.border('thin') solid ms.color('border-base-default');
  gap: ms.spacing('gap-3xl');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-4xl');
  }

  &__title {
    @include ms.font('body-lg-medium');

    color: ms.color('text-neutral-default');
  }

  &__count {
    @include ms.font('body-md-regular');

    color: ms.color('text-neutral-default');
    flex-shrink: 0;
  }

  .workspace-archive-count {
    color: ms.color('text-base-description');
  }
}

.workspaces-container {
  display: flex;
  flex-direction: column;

  @include ms.responsive-breakpoint('sm') {
    position: sticky;
    z-index: 10;
    padding: ms.spacing('padding-3xl');
  }

  &-grid {
    display: flex;
    flex-wrap: wrap;
    gap: ms.spacing('gap-3xl');
  }
}

.workspaces-loading {
  border-radius: ms.radius('lg');
  display: flex;
  height: 100%;
  text-align: center;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: ms.spacing('gap-4xl');

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
  gap: ms.spacing('gap-4xl');
  max-width: 30rem;
  height: 100%;
  color: ms.color('text-neutral-default');
  margin: auto;
  padding: ms.spacing('padding-7xl') ms.spacing('padding-3xl');

  &__image {
    width: 8rem;
    height: 8rem;

    @include ms.responsive-breakpoint('xs') {
      width: 6rem;
      height: 6rem;
    }
  }

  &__text {
    @include ms.font('body-lg-regular');
  }
}
</style>
