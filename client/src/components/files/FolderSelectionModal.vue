<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :close-button="{ visible: true }"
    :cancel-button="{
      label: 'TextInputModal.cancel',
      disabled: false,
      onClick: cancel,
    }"
    :confirm-button="{
      label: okButtonLabel || 'TextInputModal.moveHere',
      disabled: allowStartingPath ? false : selectedPath === startingPath,
      onClick: confirm,
    }"
  >
    <!-- :disabled="backStack.length === 0" -->
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
        :items-after-collapse="2"
      />
    </div>
    <ion-list class="folder-list">
      <ion-text
        class="folder-list__empty body"
        v-if="currentEntries.length === 0"
      >
        {{ $msTranslate('FoldersPage.copyMoveFolderNoElement') }}
      </ion-text>
      <div
        class="folder-container"
        v-if="currentEntries.length > 0"
      >
        <ion-item
          class="file-list-item"
          v-for="entry in currentEntries"
          :key="entry[0].id"
          :disabled="entry[1]"
          @click="enterFolder(entry[0])"
        >
          <div class="file-name">
            <div class="file-name__icons">
              <ms-image
                :image="entry[0].isFile() ? getFileIcon(entry[0].name) : Folder"
                class="file-icon"
              />
            </div>
            <ion-label class="file-name__label cell">
              {{ entry[0].name }}
            </ion-label>
          </div>
        </ion-item>
      </div>
    </ion-list>
  </ms-modal>
</template>

<script setup lang="ts">
import { getFileIcon } from '@/common/file';
import { FolderSelectionOptions } from '@/components/files';
import { Folder, MsImage, MsModalResult, MsModal } from 'megashark-lib';
import HeaderBreadcrumbs, { RouterPathNode } from '@/components/header/HeaderBreadcrumbs.vue';
import { EntryStat, FsPath, Path, StartedWorkspaceInfo, getWorkspaceInfo, statFolderChildren } from '@/parsec';
import { IonButton, IonButtons, IonText, IonIcon, IonItem, IonLabel, IonList, modalController } from '@ionic/vue';
import { chevronBack, chevronForward } from 'ionicons/icons';
import { Ref, onMounted, ref } from 'vue';

const props = defineProps<FolderSelectionOptions>();
const selectedPath: Ref<FsPath> = ref(props.startingPath);
const headerPath: Ref<RouterPathNode[]> = ref([]);
const currentEntries: Ref<[EntryStat, boolean][]> = ref([]);
const workspaceInfo: Ref<StartedWorkspaceInfo | null> = ref(null);
const backStack: FsPath[] = [];
const forwardStack: FsPath[] = [];

onMounted(async () => {
  const result = await getWorkspaceInfo(props.workspaceHandle);
  if (result.ok) {
    workspaceInfo.value = result.value;
  }
  await update();
});

async function update(): Promise<void> {
  if (!workspaceInfo.value) {
    return;
  }
  const workspaceHandle = workspaceInfo.value.handle;
  const components = await Path.parse(selectedPath.value);

  const result = await statFolderChildren(workspaceHandle, selectedPath.value);
  if (result.ok) {
    const newEntries: [EntryStat, boolean][] = [];
    for (const entry of result.value
      .filter((entry) => !entry.isConfined())
      .sort((item1, item2) => Number(item1.isFile()) - Number(item2.isFile()))) {
      const isDisabled = await isEntryDisabled(entry);
      newEntries.push([entry, isDisabled]);
    }
    currentEntries.value = newEntries;
  }

  let path = '/';
  headerPath.value = [];
  headerPath.value.push({
    id: 0,
    display: workspaceInfo.value ? workspaceInfo.value.currentName : '',
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

async function isEntryDisabled(entry: EntryStat): Promise<boolean> {
  if (entry.isFile()) {
    return true;
  }
  for (const excludePath of props.excludePaths || []) {
    if (entry.path.startsWith(excludePath)) {
      return true;
    }
  }
  return false;
}

async function forward(): Promise<void> {
  const forwardPath = forwardStack.pop();

  if (!forwardPath) {
    return;
  }
  backStack.push(selectedPath.value);
  selectedPath.value = forwardPath;
  await update();
}

async function back(): Promise<void> {
  const backPath = backStack.pop();

  if (!backPath) {
    return;
  }
  forwardStack.push(selectedPath.value);
  selectedPath.value = backPath;
  await update();
}

async function onPathChange(node: RouterPathNode): Promise<void> {
  forwardStack.splice(0, forwardStack.length);
  if (node.query && node.query.documentPath) {
    selectedPath.value = node.query.documentPath;
    await update();
  }
}

async function enterFolder(entry: EntryStat): Promise<void> {
  if (entry.isFile()) {
    return;
  }
  backStack.push(selectedPath.value);
  selectedPath.value = await Path.join(selectedPath.value, entry.name);
  await update();
}

async function confirm(): Promise<boolean> {
  return await modalController.dismiss(selectedPath.value, MsModalResult.Confirm);
}

async function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
.navigation {
  display: flex;
  margin-bottom: 1.5rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);

  .disabled {
    pointer-events: none;
    color: var(--parsec-color-light-secondary-light);
    opacity: 1;
  }
}

.folder-list {
  overflow-y: auto;
  display: flex;
  justify-content: center;
  background: var(--parsec-color-light-secondary-background);
  padding: 0.5rem;
  height: -webkit-fill-available;
  border-radius: var(--parsec-radius-6);

  &__empty {
    max-width: 10rem;
    text-align: center;
    color: var(--parsec-color-light-secondary-soft-text);
  }
}

.folder-container {
  overflow-y: auto;
  width: 100%;
}

.file-list-item {
  border-radius: var(--parsec-radius-4);
  --show-full-highlight: 0;
  cursor: pointer;

  &::part(native) {
    --padding-start: 0px;
  }

  &:hover {
    color: var(--parsec-color-light-secondary-text);
  }

  &:focus,
  &:active {
    --background-focused: var(--parsec-color-light-primary-100);
    --background: var(--parsec-color-light-primary-100);
    --background-focused-opacity: 1;
    --border-width: 0;
  }
}

.file-name {
  padding: 0.75rem 1rem;
  width: 100%;
  white-space: nowrap;
  overflow: hidden;

  .file-icon {
    width: 2rem;
    height: 2rem;
  }

  &__label {
    color: var(--parsec-color-light-secondary-text);
    margin-left: 1em;
  }
}
</style>
