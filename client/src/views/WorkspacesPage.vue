<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <!-- contextual menu -->
      <ion-item-divider class="workspace-toolbar ion-margin-bottom secondary">
        <button-option
          id="button-new-workspace"
          :button-label="$t('WorkspacesPage.createWorkspace')"
          :icon="addCircle"
          @click="openCreateWorkspaceModal()"
        />
        <div class="right-side">
          <ms-select
            id="workspace-filter-select"
            :options="msSelectOptions"
            default-option="name"
            @change="onMsSelectChange($event)"
          />
          <list-grid-toggle
            v-model="displayView"
          />
        </div>
      </ion-item-divider>
      <!-- workspaces -->
      <div class="workspaces-container">
        <div v-if="displayView === DisplayState.List">
          <ion-list>
            <ion-list-header
              class="workspace-list-header"
              lines="full"
            >
              <ion-label class="workspace-list-header__label label-name">
                {{ $t('WorkspacesPage.listDisplayTitles.name') }}
              </ion-label>
              <ion-label class="workspace-list-header__label label-role">
                {{ $t('WorkspacesPage.listDisplayTitles.role') }}
              </ion-label>
              <ion-label class="workspace-list-header__label label-users">
                {{ $t('WorkspacesPage.listDisplayTitles.sharedWith') }}
              </ion-label>
              <ion-label class="workspace-list-header__label label-update">
                {{ $t('WorkspacesPage.listDisplayTitles.lastUpdate') }}
              </ion-label>
              <ion-label class="workspace-list-header__label label-size">
                {{ $t('WorkspacesPage.listDisplayTitles.size') }}
              </ion-label>
              <ion-label class="workspace-list-header__label label-space" />
            </ion-list-header>
            <workspace-list-item
              v-for="workspace in filteredWorkspaces"
              :key="workspace.id"
              :workspace="workspace"
              @click="onWorkspaceClick"
              @menu-click="openWorkspaceContextMenu"
              @share-click="onWorkspaceShareClick"
            />
          </ion-list>
        </div>
        <div
          v-else
          class="workspaces-container-grid"
        >
          <ion-item
            class="workspaces-grid-item"
            v-for="workspace in filteredWorkspaces"
            :key="workspace.id"
          >
            <workspace-card
              :workspace="workspace"
              @click="onWorkspaceClick"
              @menu-click="openWorkspaceContextMenu"
              @share-click="onWorkspaceShareClick"
            />
          </ion-item>
        </div>
      </div>
      <div class="workspaces-footer title-h5">
        {{ $t('WorkspacesPage.itemCount', { count: workspaceList.length }, workspaceList.length) }}
      </div>
      <ion-fab
        v-if="isPlatform('mobile')"
        vertical="bottom"
        horizontal="end"
        slot="fixed"
      >
        <ion-fab-button @click="openCreateWorkspaceModal()">
          <ion-icon :icon="addCircle" />
        </ion-fab-button>
      </ion-fab>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonLabel,
  IonIcon,
  IonPage,
  IonItemDivider,
  IonContent,
  popoverController,
  isPlatform,
  IonFab,
  IonFabButton,
  modalController,
  IonList,
  IonListHeader,
  IonItem
} from '@ionic/vue';

import {
  addCircle
} from 'ionicons/icons';
import WorkspaceCard from '@/components/WorkspaceCard.vue';
import WorkspaceListItem from '@/components/WorkspaceListItem.vue';
import { MockWorkspace, getMockWorkspaces } from '@/common/mocks';
import WorkspaceContextMenu from '@/components/WorkspaceContextMenu.vue';
import { WorkspaceAction } from '@/components/WorkspaceContextMenu.vue';
import WorkspaceSharingModal from './WorkspaceSharingModal.vue';
import CreateWorkspaceModal from '@/components/CreateWorkspaceModal.vue';
import MsSelect from '@/components/MsSelect.vue';
import ButtonOption from '@/components/ButtonOption.vue';
import { MsSelectChangeEvent, MsSelectOption } from '@/components/MsSelectOption';
import ListGridToggle from '@/components/ListGridToggle.vue';
import { DisplayState } from '@/components/ListGridToggle.vue';
import { useI18n } from 'vue-i18n';
import { ref, Ref, onMounted, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const currentRoute = useRoute();
const router = useRouter();
const { t } = useI18n();
const sortBy = ref('name');
const workspaceList: Ref<MockWorkspace[]> = ref([]);
const displayView = ref(DisplayState.Grid);

onMounted(async (): Promise<void> => {
  workspaceList.value = await getMockWorkspaces();
});

const filteredWorkspaces = computed(() => {
  // Copy to avoid updating the workspaceList itself
  return Array.from(workspaceList.value).sort((a: MockWorkspace, b: MockWorkspace) => {
    if (sortBy.value === 'name') {
      return a.name.localeCompare(b.name);
    } else if (sortBy.value === 'size') {
      return a.size - b.size;
    } else if (sortBy.value === 'lastUpdated') {
      return b.lastUpdate.diff(a.lastUpdate).milliseconds;
    }
    return 0;
  });
});

const msSelectOptions: MsSelectOption[] = [
  { label: t('WorkspacesPage.sort.sortByName'), key: 'name' },
  { label: t('WorkspacesPage.sort.sortBySize'), key: 'size' },
  { label: t('WorkspacesPage.sort.sortByLastUpdated'), key: 'lastUpdated' }
];

function onMsSelectChange(event: MsSelectChangeEvent): void {
  sortBy.value = event.option.key;
}

async function openCreateWorkspaceModal(): Promise<void> {
  const modal = await modalController.create({
    component: CreateWorkspaceModal,
    cssClass: 'one-line-modal'
  });
  modal.present();

  const { data, role } = await modal.onWillDismiss();

  if (role === 'confirm') {
    console.log(data);
  }
}

function onWorkspaceClick(_event: Event, workspace: MockWorkspace): void {
  router.push({
    name: 'folder',
    params: { deviceId: currentRoute.params.deviceId, workspaceId: workspace.id },
    query: { path: '/' }
  });
}

async function onWorkspaceShareClick(_: Event, workspace: MockWorkspace): Promise<void> {
  const modal = await modalController.create({
    component: WorkspaceSharingModal,
    componentProps: {
      workspaceId: workspace.id
    },
    cssClass: 'workspace-sharing-modal'
  });
  await modal.present();
  await modal.onWillDismiss();
}

async function openWorkspaceContextMenu(event: Event, workspace: MockWorkspace): Promise<void> {
  const popover = await popoverController
    .create({
      component: WorkspaceContextMenu,
      event: event,
      translucent: true,
      showBackdrop: false,
      dismissOnSelect: true,
      reference: 'event'
    });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  if (data !== undefined) {
    if (data.action === WorkspaceAction.Manage) {
      onWorkspaceShareClick(new Event('ignored'), workspace);
    }
  }
}
</script>

<style lang="scss" scoped>
.workspaces-container {
  margin: 2em;
  background-color: white;
}

.workspace-list-header {
  color: var(--parsec-color-light-secondary-grey);
  font-weight: 600;
  padding-inline-start:0;

  &__label {
    padding: 0 1rem;
    height: 100%;
    display: flex;
    align-items: center;
  }

  .label-name {
    width: 100%;
    max-width: 20vw;
    min-width: 11.25rem;
    white-space: nowrap;
    overflow: hidden;
  }

  .label-role {
    min-width: 11.25rem;
    max-width: 10vw;
    flex-grow: 2;
  }

  .label-users {
    min-width: 14.5rem;
    flex-grow: 0;
  }

  .label-update {
    min-width: 11.25rem;
    flex-grow: 0;
  }

  .label-size {
    min-width: 7.5rem;
  }

  .label-space {
    min-width: 4rem;
    flex-grow: 0;
    margin-left: auto;
    margin-right: 1rem;
  }
}

.workspaces-footer {
  width: 100%;
  left: 0;
  position: fixed;
  bottom: 0;
  text-align: center;
  color: var(--parsec-color-light-secondary-text);
  margin-bottom: 2em;
}

.workspaces-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow-y: auto;
}

ion-item::part(native) {
  --padding-start: 0px;
}

.workspace-toolbar {
  padding: 1em 2em;
  height: 6em;
  background-color: var(--parsec-color-light-secondary-background);
  border-top: 1px solid var(--parsec-color-light-secondary-light);
}

.right-side {
  margin-left: auto;
  display: flex;
}
</style>
