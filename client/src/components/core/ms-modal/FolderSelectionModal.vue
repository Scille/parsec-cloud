<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :close-button="{ visible: true }"
    :cancel-button="{
      label: $t('TextInputModal.cancel'),
      disabled: false,
      onClick: cancel,
    }"
    :confirm-button="{
      label: $t('TextInputModal.moveHere'),
      disabled: selectedPath === startingPath,
      onClick: confirm,
    }"
  >
    <div class="navigation">
      <ion-buttons>
        <ion-button
          fill="clear"
          @click="goBack()"
          class="navigation-back-button"
          :class="{ disabled: selectedPath === startingPath }"
          ref="backButtonDisabled"
        >
          <ion-icon :icon="chevronBack" />
        </ion-button>
        <ion-button
          fill="clear"
          @click="goForward()"
          class="navigation-back-button"
          :class="{ disabled: backButtonDisabled }"
        >
          <ion-icon :icon="chevronForward" />
        </ion-button>
      </ion-buttons>
      <header-breadcrumbs
        :path-nodes="headerPath"
        @change="onPathChange"
        class="navigation-breadcrumb"
      />
    </div>
    <ion-list class="file-list">
      <div class="file-container">
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
  </ms-modal>
</template>

<script setup lang="ts">
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { FolderSelectionOptions, MsModalResult } from '@/components/core/ms-modal/types';
import HeaderBreadcrumbs, { RouterPathNode } from '@/components/header/HeaderBreadcrumbs.vue';
import { EntryStat, EntryStatFolder, Path, entryStat, getWorkspaceName } from '@/parsec';
import { IonButton, IonButtons, IonIcon, IonItem, IonLabel, IonList, modalController } from '@ionic/vue';
import { chevronBack, chevronForward, document, folder } from 'ionicons/icons';
import { Ref, onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const props = defineProps<FolderSelectionOptions>();
const selectedPath = ref(props.startingPath);
const headerPath: Ref<RouterPathNode[]> = ref([]);
const currentEntries: Ref<EntryStat[]> = ref([]);
const backButtonDisabled = ref(true);

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

  let workspaceName = '';
  const workspaceResult = await getWorkspaceName(props.workspaceId);
  if (workspaceResult.ok) {
    workspaceName = workspaceResult.value;
  }

  let path = '/';
  headerPath.value = [];
  headerPath.value.push({
    id: 0,
    display: workspaceName,
    name: '',
    query: { path: path },
  });
  let id = 1;
  for (const comp of components) {
    path = await Path.join(path, comp);
    headerPath.value.push({
      id: id,
      display: comp === '/' ? '' : comp,
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

function goBack(): void {
  backButtonDisabled.value = false;
  router.back();
}

function goForward(): void {
  router.forward();
}
</script>

<style scoped lang="scss">
.navigation {
  display: flex;
  margin-bottom: 1.5rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--parsec-color-light-secondary-light);

  .disabled {
    pointer-events: none;
    color: var(--parsec-color-light-secondary-disabled);
  }
}

.file-list {
  overflow-y: auto;
}

.file-container {
  overflow-y: auto;
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
