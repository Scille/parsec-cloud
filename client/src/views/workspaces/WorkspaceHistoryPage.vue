<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="content-scroll"
    >
      <div class="head-content">
        <div
          v-if="workspaceInfo"
        >
          {{ workspaceInfo.currentName }}
        </div>

        <div>
          <ion-datetime-button datetime="datetime"></ion-datetime-button>
          <ion-modal :keep-contents-mounted="true">
            <ion-datetime
              id="datetime"
              :min="_MOCK_WORKSPACE_CREATION_DATE.toISO()"
              :max="DateTime.now().toISO()"
              @ion-change="onDateTimeChange"
              :value="selectedDateTime.toISO()"
            />
          </ion-modal>
        </div>
      </div>

      <div class="folder-container scroll">
        <div class="navigation">
          <ion-buttons>
            <ion-button
              fill="clear"
              @click="back()"
              class="navigation-back-button"
              :disabled="backStack.length === 0"
              :class="{ disabled: backStack.length === 0 }"
              ref="backButtonDisabled"
            >
              <ion-icon :icon="chevronBack" />
            </ion-button>
            <ion-button
              fill="clear"
              @click="forward()"
              :disabled="forwardStack.length === 0"
              :class="{ disabled: forwardStack.length === 0 }"
              class="navigation-forward-button"
            >
              <ion-icon :icon="chevronForward" />
            </ion-button>
          </ion-buttons>
          <header-breadcrumbs
            :path-nodes="headerPath"
            @change="onPathChange"
            class="navigation-breadcrumb"
            :items-before-collapse="1"
            :items-after-collapse="1"
            :max-shown="2"
          />
        </div>
        <div>
          <ms-search-input
            @change="onSearchChanged"
            :debounce="1000"
          />
          <ion-button
            :disabled="!someSelected"
          >
            Restaurer
          </ion-button>
        </div>
        <div
          class="scroll"
        >
          <div v-show="querying">
            <ms-spinner />
          </div>
          <div v-show="!querying && entries.entriesCount() === 0">
            EMPTY
          </div>
          <div v-show="!querying && entries.entriesCount() > 0">
            <ion-list class="list">
              <ion-list-header
                class="folder-list-header"
                lines="full"
              >
                <ion-label class="folder-list-header__label ion-text-nowrap label-selected">
                  <ms-checkbox
                    @change="selectAll"
                    :checked="allSelected"
                    :indeterminate="someSelected && !allSelected"
                  />
                </ion-label>
                <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-name">
                  {{ $msTranslate('FoldersPage.listDisplayTitles.name') }}
                </ion-label>
                <ion-label
                  class="folder-list-header__label cell-title ion-text-nowrap label-updatedBy"
                  v-show="false"
                >
                  {{ $msTranslate('FoldersPage.listDisplayTitles.updatedBy') }}
                </ion-label>
                <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-lastUpdate">
                  {{ $msTranslate('FoldersPage.listDisplayTitles.lastUpdate') }}
                </ion-label>
                <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-size">
                  {{ $msTranslate('FoldersPage.listDisplayTitles.size') }}
                </ion-label>
                <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-space" />
              </ion-list-header>
              <div>
                <file-list-item
                  v-for="entry in entries.getEntries()"
                  :key="entry.id"
                  :entry="entry"
                  :show-checkbox="true"
                  :disable-drop="true"
                  @click="onEntryClicked(entry)"
                  :hide-menu="true"
                  :hide-sync-status="true"
                  :hide-last-update="true"
                />
              </div>
            </ion-list>
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonPage,
  IonDatetime,
  IonDatetimeButton,
  IonModal,
  DatetimeCustomEvent,
  IonList,
  IonItem,
  IonLabel,
  IonButtons,
  IonIcon,
  IonButton,
  IonListHeader,
  IonContent,
} from '@ionic/vue';
import { computed, onMounted, ref, Ref } from 'vue';
import { FsPath, Path, statFolderChildren, getWorkspaceInfo, StartedWorkspaceInfo } from '@/parsec';
import { MsModal, MsCheckbox } from 'megashark-lib';
import { DateTime } from 'luxon';
import { RouterPathNode } from '@/components/header/HeaderBreadcrumbs.vue';
import HeaderBreadcrumbs from '@/components/header/HeaderBreadcrumbs.vue';
import { EntryCollection, EntryModel } from '@/components/files';
import { chevronBack, chevronForward } from 'ionicons/icons';
import { getFileIcon } from '@/common/file';
import { Folder, MsImage, MsSearchInput } from 'megashark-lib';
import { FileListItem } from '@/components/files';
import { currentRouteIs, getDocumentPath, getWorkspaceHandle, Routes } from '@/router';

// Replace this with the real workspace creation date when available
const _MOCK_WORKSPACE_CREATION_DATE = DateTime.fromISO('2024-04-07T12:00:00');

const workspaceInfo: Ref<StartedWorkspaceInfo | null> = ref(null);
// Default it to 5 seconds ago to not interfere with the `max` value
const selectedDateTime: Ref<DateTime> = ref(DateTime.now().minus({ seconds: 5 }));
const backStack: FsPath[] = [];
const forwardStack: FsPath[] = [];
const currentPath: Ref<FsPath> = ref('/');
const headerPath: Ref<RouterPathNode[]> = ref([]);
const entries: Ref<EntryCollection<EntryModel>> = ref(new EntryCollection<EntryModel>());
const querying = ref(false);

const allSelected = computed(() => {
  return entries.value.selectedCount() === entries.value.entriesCount();
});

const someSelected = computed(() => {
  return entries.value.selectedCount() > 0;
});

onMounted(async () => {
  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    const infoResult = await getWorkspaceInfo(workspaceHandle);
    if (infoResult.ok) {
      workspaceInfo.value = infoResult.value;
    } else {
      console.error('Failed to retrieve workspace info');
    }
  }
  currentPath.value = getDocumentPath() ?? '/';

  await listFolder();
});

async function onDateTimeChange(event: DatetimeCustomEvent): Promise<void> {
  if (!event.detail.value) {
    return;
  }
  selectedDateTime.value = DateTime.fromISO(event.detail.value as string);
}

async function listFolder(): Promise<void> {
  if (!currentRouteIs(Routes.History)) {
    return;
  }
  if (!workspaceInfo.value) {
    return;
  }

  const workspaceHandle = workspaceInfo.value.handle;
  const components = await Path.parse(currentPath.value);

  const result = await statFolderChildren(workspaceHandle, currentPath.value, selectedDateTime.value);
  if (result.ok) {
    const newEntries: EntryModel[] = [];
    const entriesList = result.value
      .filter((entry) => !entry.isConfined())
      .sort((item1, item2) => Number(item1.isFile()) - Number(item2.isFile()));
    for (const entry of entriesList) {
      (entry as EntryModel).isSelected = false;
      newEntries.push(entry as EntryModel);
    }
    entries.value.replace(newEntries);
  }

  let path = '/';
  headerPath.value = [];
  headerPath.value.push({
    id: 0,
    display: workspaceInfo.value.currentName,
    name: '',
    query: { documentPath: path },
  });
  let id = 1;
  for (const comp of components) {
    path = await Path.join(path, comp);
    headerPath.value.push({
      id: id,
      display: comp === '/' ? '' : comp,
      name: '',
      query: { documentPath: path },
    });
    id += 1;
  }
}

async function forward(): Promise<void> {
  const forwardPath = forwardStack.pop();

  if (!forwardPath) {
    return;
  }
  backStack.push(currentPath.value);
  currentPath.value = forwardPath;
  await listFolder();
}

async function back(): Promise<void> {
  const backPath = backStack.pop();

  if (!backPath) {
    return;
  }
  forwardStack.push(currentPath.value);
  currentPath.value = backPath;
  await listFolder();
}

async function onPathChange(node: RouterPathNode): Promise<void> {
  forwardStack.splice(0, forwardStack.length);
  if (node.query && node.query.documentPath) {
    currentPath.value = node.query.documentPath;
    await listFolder();
  }
}

async function selectAll(selected: boolean): Promise<void> {
  entries.value.selectAll(selected);
}

async function onEntryClicked(entry: EntryModel): Promise<void> {
  if (entry.isFile()) {
    console.log('click on file');
  } else {
    backStack.push(currentPath.value);
    currentPath.value = await Path.join(currentPath.value, entry.name);
    await listFolder();
  }
}

async function onSearchChanged(value: string): Promise<void> {
  console.log(value);
  if (!value) {
    await listFolder();
  } else {

  }
}

</script>

<style scoped lang="scss">
.folder-container {
  height: 100%;
}

.scroll {
  padding: 0;
  margin-bottom: 0;
}

.folder-list-header {
  &__label {
    padding: 0.75rem 1rem;
  }
  .label-selected {
    display: flex;
    align-items: center;
  }

  .label-space {
    min-width: 4rem;
    flex-grow: 0;
    margin-left: auto;
  }
}
</style>

