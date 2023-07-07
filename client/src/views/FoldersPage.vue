<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <ion-item-divider class="folder-toolbar ion-margin-bottom secondary">
        <button-option
          id="button-new-folder"
          :button-label="$t('FoldersPage.createFolder')"
          :icon="folderOpen"
          @click="createFolder()"
        />
        <button-option
          id="button-import"
          :button-label="$t('FoldersPage.import')"
          :icon="document"
          @click="importFiles()"
        />
        <div class="right-side">
          <list-grid-toggle
            :list-view="true"
            @toggle-view="onToggleView($event)"
          />
        </div>
      </ion-item-divider>
      <div class="folder-container">
        <div v-if="listView">
          <ion-list>
            <ion-list-header
              class="folder-list-header"
              lines="full"
            >
              <ion-label class="folder-list-header__label label-name">
                {{ $t('FoldersPage.listDisplayTitles.name') }}
              </ion-label>
              <ion-label class="folder-list-header__label label-updatedBy">
                {{ $t('FoldersPage.listDisplayTitles.updatedBy') }}
              </ion-label>
              <ion-label class="folder-list-header__label label-lastUpdate">
                {{ $t('FoldersPage.listDisplayTitles.lastUpdate') }}
              </ion-label>
              <ion-label class="folder-list-header__label label-size">
                {{ $t('FoldersPage.listDisplayTitles.size') }}
              </ion-label>
              <ion-label class="folder-list-header__label label-space" />
            </ion-list-header>
            <file-list-item
              v-for="child in folderInfo.children"
              :key="child.name"
              :file="child"
              @click="onFileClick"
              @menu-click="openFileContextMenu"
            />
          </ion-list>
        </div>
        <div
          v-else
          class="folders-container-grid"
        >
          <ion-item
            class="folder-grid-item"
            v-for="child in folderInfo.children"
            :key="child.name"
          >
            <file-card
              :file="child"
              @click="onFileClick"
              @menu-click="openFileContextMenu"
            />
          </ion-item>
        </div>
        <div class="folder-footer title-h5">
          {{ $t('FoldersPage.itemCount', { count: folderInfo.children.length }, folderInfo.children.length) }}
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonItem,
  IonItemDivider,
  IonPage,
  IonContent,
  IonList,
  IonListHeader,
  IonLabel,
  popoverController
} from '@ionic/vue';
import { folderOpen, document } from 'ionicons/icons';
import { useRoute } from 'vue-router';
import { computed, ref } from 'vue';
import { MockFile, pathInfo } from '@/common/mocks';
import ButtonOption from '@/components/ButtonOption.vue';
import listGridToggle from '@/components/listGridToggle.vue';
import FileListItem from '@/components/FileListItem.vue';
import FileCard from '@/components/FileCard.vue';
import FileContextMenu from '@/components/FileContextMenu.vue';
import router from '@/router';

const currentRoute = useRoute();

// Use computed so the variables will automatically update when the route changes
const path = computed(() => {
  return currentRoute.query.path ?? '/';
});
const workspaceId = computed(() => {
  return currentRoute.params.workspaceId;
});
const folderInfo = computed((): MockFile => {
  return pathInfo(path.value.toString());
});
const listView = ref(true);

// eslint-disable-next-line @typescript-eslint/no-empty-function
function createFolder(): void {
}

// eslint-disable-next-line @typescript-eslint/no-empty-function
function importFiles(): void {
}

function onFileClick(_event: Event, file: MockFile): void {
  if (file.type === 'folder') {
    router.push({
      name: 'folder',
      params: { deviceId: currentRoute.params.deviceId, workspaceId: workspaceId.value },
      query: { path: `${path.value}/${file.name}` }
    });
  }
}

async function openFileContextMenu(event: Event, _file: MockFile): Promise<void> {
  const popover = await popoverController
    .create({
      component: FileContextMenu,
      event: event,
      translucent: true,
      showBackdrop: false,
      dismissOnSelect: true,
      reference: 'event'
    });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  if (data !== undefined) {
    console.log(data.action);
    /*
    Keeping the comment here just to show how to check
    what action was selected.

    if (data.action === FileAction.Rename) {
      console.log('Rename!');
    }
    */
  }
}

function onToggleView(value: boolean): void {
  listView.value = value;
}
</script>

<style scoped>
.folder-container {
  margin: 2em;
  background-color: white;
}

.folder-list-header {
  color: var(--parsec-color-light-secondary-grey);
  font-weight: 600;
  padding-inline-start: 0;

}

.label-name {
  width: 100%;
  min-width: 11.25rem;
  white-space: nowrap;
  overflow: hidden;
}

.label-updatedBy {
  min-width: 11.25rem;
  max-width: 10vw;
  flex-grow: 2;
}

.label-lastUpdate {
  min-width: 11.25rem;
  flex-grow: 0;
}

.label-size {
  min-width: 11.25rem;
}

.label-space {
  min-width: 4rem;
  flex-grow: 0;
  margin-left: auto;
  margin-right: 1rem;
}

.folder-footer {
  width: 100%;
  left: 0;
  position: fixed;
  bottom: 0;
  text-align: center;
  color: var(--parsec-color-light-secondary-text);
  margin-bottom: 2em;
}

.folders-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow-y: auto;
}

ion-item::part(native) {
  --padding-start: 0px;
}

.folder-toolbar {
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
