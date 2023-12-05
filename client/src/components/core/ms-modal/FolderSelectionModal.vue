<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :subtitle="subtitle"
    :close-button="{ visible: true }"
    :cancel-button="{
      label: $t('TextInputModal.cancel'),
      disabled: false,
      onClick: cancel,
    }"
    :confirm-button="{
      label: $t('TextInputModal.ok'),
      disabled: selectedPath === startingPath,
      onClick: confirm,
    }"
  >
    <header-breadcrumbs
      :path-nodes="headerPath"
      @change="onPathChange"
    />
    <div>
      <ion-list>
        <ion-list-header
          class="folder-list-header"
          lines="full"
        >
          <ion-label class="folder-list-header__label label-name">
            {{ $t('FoldersPage.listDisplayTitles.name') }}
          </ion-label>
        </ion-list-header>
        <div class="folders-container">
          <ion-item
            class="file-list-item"
            v-for="entry in currentEntries"
            :key="entry.id"
            :disabled="entry.isFile()"
            @click="enterFolder(entry)"
          >
            <div class="file-name">
              <div class="file-name__icons">
                <ion-icon
                  class="main-icon"
                  :icon="entry.isFile() ? document : folder"
                  size="default"
                />
              </div>
              <ion-label class="file-name__label cell">
                {{ entry.name }}
              </ion-label>
            </div>
          </ion-item>
        </div>
      </ion-list>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { home, folder, document } from 'ionicons/icons';
import { EntryStat, EntryStatFolder } from '@/parsec';
import HeaderBreadcrumbs, { RouterPathNode } from '@/components/header/HeaderBreadcrumbs.vue';
import { modalController, IonList, IonItem, IonLabel, IonIcon, IonListHeader } from '@ionic/vue';
import { ref, Ref, onMounted } from 'vue';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { entryStat } from '@/parsec';
import { MsModalResult } from '@/components/core/ms-types';
import { Path } from '@/parsec';
import { FolderSelectionOptions } from '@/common/inputs';

const props = defineProps<FolderSelectionOptions>();
const selectedPath = ref(props.startingPath);
const headerPath: Ref<RouterPathNode[]> = ref([]);
const currentEntries: Ref<EntryStat[]> = ref([]);

onMounted(async () => {
  await update();
});

async function update(): Promise<void> {
  const components = await Path.parse(selectedPath.value);

  const result = await entryStat(selectedPath.value);
  if (result.ok) {
    currentEntries.value = [];
    for (const childName of (result.value as EntryStatFolder).children) {
      const childPath = await Path.join(selectedPath.value, childName);
      const entryResult = await entryStat(childPath);
      if (entryResult.ok) {
        currentEntries.value.push(entryResult.value);
      }
    }
  }
  currentEntries.value.sort((item1, item2) => Number(item1.isFile()) - Number(item2.isFile()));

  let path = '/';
  headerPath.value = [];
  headerPath.value.push({
    id: 0,
    display: '',
    icon: home,
    name: '',
    query: { path: path },
  });
  let id = 1;
  for (const comp of components) {
    path = await Path.join(path, comp);
    headerPath.value.push({
      id: id,
      display: comp === '/' ? '' : comp,
      icon: comp === '/' ? home : undefined,
      name: '',
      query: { path: path },
    });
    id += 1;
  }
}

async function onPathChange(node: RouterPathNode): Promise<void> {
  selectedPath.value = node.query.path;
  await update();
}

async function enterFolder(entry: EntryStat): Promise<void> {
  if (entry.isFile()) {
    return;
  }
  selectedPath.value = await Path.join(selectedPath.value, entry.name);
  await update();
}

async function confirm(): Promise<boolean> {
  console.log(selectedPath.value);
  return await modalController.dismiss(selectedPath.value, MsModalResult.Confirm);
}

async function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
.folder-list-header {
  color: var(--parsec-color-light-secondary-grey);
  font-weight: 600;
  padding-inline-start: 0;

  &__label {
    padding: 0.75rem 1rem;
  }

  .label-name {
    width: 100%;
    min-width: 11.25rem;
    white-space: nowrap;
    overflow: hidden;
  }
}

.folders-container {
  max-height: 15em;
  overflow-y: auto;
}

.file-list-item {
  border-radius: var(--parsec-radius-4);
  --show-full-highlight: 0;

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

.file-list-item > [class^='file-'] {
  padding: 0 1rem;
  display: flex;
  align-items: center;
  height: 4rem;
}
.file-name {
  padding: 0.75rem 1rem;
  width: 100%;
  white-space: nowrap;
  overflow: hidden;

  &__icons {
    position: relative;
    padding: 5px;

    .main-icon {
      color: var(--parsec-color-light-primary-600);
      font-size: 1.5rem;
    }
  }

  &__label {
    color: var(--parsec-color-light-secondary-text);
    margin-left: 1em;
  }
}
</style>
