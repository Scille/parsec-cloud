<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div class="container">
        <ms-spinner
          class="file-handler--loading"
          title="fileViewers.retrievingFileContent"
          v-show="!loaded"
        />
        <div
          v-if="loaded && contentInfo"
          @click.prevent="onClick"
          class="file-handler"
        >
          <!-- file-handler topbar -->
          <div class="file-handler-topbar">
            <!-- icon visible when menu is hidden -->
            <ms-image
              v-if="!isMobile() && isLargeDisplay && !isHeaderVisible()"
              slot="start"
              id="trigger-toggle-menu-button"
              :image="SidebarToggle"
              @click="isSidebarMenuVisible ? hideSidebarMenu() : showSidebarMenu()"
            />
            <div
              class="topbar-left-content"
              ref="backBlock"
              v-if="!isHeaderVisible()"
            >
              <header-back-button
                :short="true"
                class="file-handler-topbar__back-button"
              />
            </div>
            <ms-image
              :image="getFileIcon(contentInfo.fileName)"
              class="file-icon"
            />
            <div class="file-handler-topbar__title">
              <ion-text class="subtitles-normal">
                {{ contentInfo.fileName }}
              </ion-text>
              <ion-text
                class="subtitles-sm"
                v-if="atDateTime"
              >
                {{ $msTranslate(I18n.formatDate(atDateTime)) }}
              </ion-text>
            </div>
            <div
              v-if="isComponentEditor() && readOnly"
              class="save-info"
            >
              {{ $msTranslate('fileEditors.saving.readOnly') }}
            </div>
            <div
              v-if="isComponentEditor() && !readOnly"
              class="save-info"
            >
              <ion-icon
                v-show="saveState === SaveState.Error"
                class="save-info-icon save-info-icon-error"
                ref="errorIcon"
                :icon="alert"
              />
              <ion-icon
                v-show="saveState === SaveState.Saved"
                class="save-info-icon save-info-icon-ok"
                id="saved-changes"
                ref="savedIcon"
                :icon="save"
              />
              <ms-spinner
                v-show="saveState === SaveState.Saving"
                class="save-info-spinner"
              />
              <ms-image
                v-show="saveState === SaveState.Unsaved"
                class="save-info-icon save-info-icon-ko"
                id="unsaved-changes"
                ref="unsavedIcon"
                :image="UnsavedIcon"
              />
              <ion-icon
                v-show="saveState === SaveState.Offline"
                class="save-info-icon save-info-icon-ko"
                ref="offlineIcon"
                :icon="warning"
              />
              <ion-text
                v-show="saveState !== SaveState.None && showSaveStateText && isLargeDisplay"
                class="save-info-text button-small"
                :class="{
                  'save-info-text-fade-out': saveState === SaveState.Saved || saveState === SaveState.Error,
                  'save-info-text-fade-in': saveState !== SaveState.Saved && saveState !== SaveState.Error,
                  'save-info-text-ok': saveState !== SaveState.Error,
                  'save-info-text-ko': saveState === SaveState.Error,
                }"
              >
                {{ $msTranslate(saveStateText) }}
              </ion-text>
            </div>
            <!-- Here we could put the file action buttons -->
            <div
              class="file-handler-topbar-buttons"
              v-if="isLargeDisplay"
            >
              <ion-button
                class="file-handler-topbar-buttons__item"
                id="file-handler-details"
                @click="showDetails"
                v-if="isDesktop()"
                :disabled="!handlerReadyRef"
              >
                <ion-icon
                  :icon="informationCircle"
                  class="item-icon"
                />
                {{ $msTranslate('fileViewers.details') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item"
                id="file-handler-copy-link"
                @click="copyPath(contentInfo.path)"
                v-if="isWeb()"
                :disabled="!handlerReadyRef"
              >
                <ion-icon
                  :icon="link"
                  class="item-icon"
                />
                {{ $msTranslate('fileViewers.copyLink') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item"
                id="file-handler-open-editor"
                @click="openEditor(contentInfo.path)"
                v-if="!atDateTime && handlerMode === FileHandlerMode.View && isFileEditable(contentInfo.fileName) && !isReader"
              >
                <ion-icon
                  :icon="create"
                  class="item-icon"
                />
                {{ $msTranslate('fileViewers.openInEditor') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item"
                @click="downloadFile"
                v-if="isWeb()"
                :disabled="!handlerReadyRef"
              >
                <ms-image
                  :image="DownloadIcon"
                  class="item-icon"
                />
                {{ $msTranslate('fileViewers.download') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item"
                id="file-handler-open-with-system"
                @click="openWithSystem(contentInfo.path)"
                v-show="isDesktop() && !atDateTime"
                :disabled="!handlerReadyRef"
              >
                <ion-icon
                  :icon="open"
                  class="item-icon"
                />
                {{ $msTranslate('fileViewers.openWithDefault') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item toggle-menu"
                @click="toggleMainHeader"
                :class="{ 'header-visible': isHeaderVisible() }"
              >
                <ion-icon :icon="isHeaderVisible() ? chevronUp : chevronDown" />
                <span v-if="windowWidth > WindowSizeBreakpoints.MD">
                  {{ $msTranslate(isHeaderVisible() ? 'fileViewers.hideMenu' : 'fileViewers.showMenu') }}
                </span>
              </ion-button>
            </div>
            <ion-button
              class="file-handler-topbar-buttons__item action-menu"
              @click="openSmallDisplayActionMenu"
              v-else
            >
              <ion-icon :icon="ellipsisHorizontal" />
            </ion-button>
          </div>

          <!-- file-handler sub-component -->
          <component
            v-if="detectedFileType"
            :is="handlerComponent"
            :content-info="contentInfo"
            :file-info="detectedFileType"
            @file-loaded="handlerReadyRef = true"
            v-on="isComponentEditor() ? { onSaveStateChange: onSaveStateChange } : {}"
            v-bind="isComponentEditor() ? { userInfo: userInfo, readOnly: readOnly } : {}"
          />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { getFileIcon } from '@/common/file';
import { DetectedFileType } from '@/common/fileTypes';
import { copyPathLinkToClipboard } from '@/components/files';
import HeaderBackButton from '@/components/header/HeaderBackButton.vue';
import {
  ClientInfo,
  closeFile,
  DEFAULT_READ_SIZE,
  EntryName,
  entryStat,
  EntryStatFile,
  FsPath,
  getClientInfo,
  getSystemPath,
  getWorkspaceInfo,
  isDesktop,
  isMobile,
  isWeb,
  openFile,
  Path,
  readFile,
  WorkspaceHandle,
  WorkspaceHistory,
  WorkspaceHistoryEntryStatFile,
  WorkspaceRole,
} from '@/parsec';
import {
  currentRouteIs,
  getCurrentRouteQuery,
  getDocumentPath,
  getFileHandlerMode,
  getWorkspaceHandle,
  navigateTo,
  routerGoBack,
  Routes,
  watchRoute,
} from '@/router';
import { isFileEditable } from '@/services/cryptpad';
import { Env } from '@/services/environment';
import { openPath } from '@/services/fileOpener';
import { FileOperationManager, FileOperationManagerKey } from '@/services/fileOperationManager';
import useHeaderControl from '@/services/headerControl';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import useSidebarMenu from '@/services/sidebarMenu';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { downloadEntry, FileDetailsModal, FileHandlerAction, openDownloadConfirmationModal } from '@/views/files';
import { FileHandlerMode } from '@/views/files/handler';
import { FileEditor, SaveState } from '@/views/files/handler/editor';
import SmallDisplayViewerActionMenu from '@/views/files/handler/SmallDisplayViewerActionMenu.vue';
import { FileViewer } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { IonButton, IonContent, IonIcon, IonPage, IonText, modalController } from '@ionic/vue';
import { alert, chevronDown, chevronUp, create, ellipsisHorizontal, informationCircle, link, open, save, warning } from 'ionicons/icons';
import { DateTime } from 'luxon';
import {
  Answer,
  askQuestion,
  attachMouseOverTooltip,
  Base64,
  DownloadIcon,
  I18n,
  MsImage,
  MsModalResult,
  MsSpinner,
  SidebarToggle,
  UnsavedIcon,
  useWindowSize,
  WindowSizeBreakpoints,
} from 'megashark-lib';
import { computed, inject, onMounted, onUnmounted, ref, Ref, shallowRef, useTemplateRef } from 'vue';
import { onBeforeRouteLeave, onBeforeRouteUpdate } from 'vue-router';

const { isLargeDisplay, windowWidth } = useWindowSize();
const storageManager: StorageManager = inject(StorageManagerKey)!;
const fileOperationManager: FileOperationManager = inject(FileOperationManagerKey)!;
const informationManager: InformationManager = inject(InformationManagerKey)!;
const contentInfo: Ref<FileContentInfo | undefined> = ref(undefined);
const detectedFileType = ref<DetectedFileType | null>(null);
const loaded = ref(false);
const atDateTime: Ref<DateTime | undefined> = ref(undefined);
const { isHeaderVisible, toggleHeader: toggleMainHeader, showHeader, hideHeader } = useHeaderControl();
const { isVisible: isSidebarMenuVisible, hide: hideSidebarMenu, show: showSidebarMenu } = useSidebarMenu();
const handlerReadyRef = ref(false);
const handlerComponent: Ref<typeof FileEditor | typeof FileViewer | null> = shallowRef(null);
const handlerMode = ref<FileHandlerMode | undefined>(undefined);
const sidebarMenuVisibleOnMounted = ref(false);
const userInfo: Ref<ClientInfo | undefined> = ref(undefined);
const isReader: Ref<boolean> = ref(true);
const saveState = ref<SaveState>(SaveState.None);
const savedIconRef = useTemplateRef<InstanceType<typeof IonIcon>>('savedIcon');
const unsavedIconRef = useTemplateRef<InstanceType<typeof MsImage>>('unsavedIcon');
const errorIconRef = useTemplateRef<InstanceType<typeof IonIcon>>('errorIcon');
const offlineIconRef = useTemplateRef<InstanceType<typeof IonIcon>>('offlineIcon');
const showSaveStateText = ref(true);
const readOnly = ref(false);

const cancelRouteWatch = watchRoute(async () => {
  if (!currentRouteIs(Routes.FileHandler)) {
    return;
  }

  const query = getCurrentRouteQuery();

  const timestamp = Number.isNaN(Number(query.timestamp)) ? undefined : Number(query.timestamp);
  const fileHandlerMode = getFileHandlerMode();

  // Same file, no need to reload
  if (
    contentInfo.value &&
    contentInfo.value.path === getDocumentPath() &&
    atDateTime.value?.toMillis() === timestamp &&
    fileHandlerMode === handlerMode.value &&
    Boolean(query.readOnly) === readOnly.value
  ) {
    return;
  }
  handlerMode.value = fileHandlerMode;
  readOnly.value = Boolean(query.readOnly);

  await loadFile();
});

const saveStateText = computed(() => {
  switch (saveState.value) {
    case SaveState.Saved:
      return 'fileEditors.saving.saved';
    case SaveState.Saving:
      return 'fileEditors.saving.saving';
    case SaveState.None:
      return '';
    case SaveState.Error:
      return 'fileEditors.saving.error';
    case SaveState.Offline:
      return 'fileEditors.saving.offline';
    default:
      return 'fileEditors.saving.unsaved';
  }
});

function isComponentEditor(): boolean {
  return handlerComponent.value === FileEditor;
}

async function _getFileInfoAt(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  at: DateTime,
  fileInfo: DetectedFileType,
  fileName: EntryName,
): Promise<FileContentInfo | undefined> {
  const infoResult = await getWorkspaceInfo(workspaceHandle);
  if (!infoResult.ok) {
    return;
  }
  const history = new WorkspaceHistory(infoResult.value.id);
  try {
    await history.start(at);
    const statsResult = await history.entryStat(path);

    if (!statsResult.ok || !statsResult.value.isFile()) {
      return;
    }
    const openResult = await history.openFile(path);
    if (!openResult.ok) {
      return;
    }
    const info: FileContentInfo = {
      data: new Uint8Array(Number((statsResult.value as WorkspaceHistoryEntryStatFile).size)),
      extension: fileInfo.extension,
      contentType: fileInfo.type,
      fileName: fileName,
      path: path,
      fileId: statsResult.value.id,
    };
    const fd = openResult.value;
    try {
      let loop = true;
      let offset = 0;
      while (loop) {
        const readResult = await history.readFile(openResult.value, offset, DEFAULT_READ_SIZE);
        if (!readResult.ok) {
          throw Error(JSON.stringify(readResult.error));
        }
        const buffer = new Uint8Array(readResult.value);
        info.data.set(buffer, offset);
        if (readResult.value.byteLength < DEFAULT_READ_SIZE) {
          loop = false;
        }
        offset += readResult.value.byteLength;
      }
      return info;
    } catch (e: any) {
      window.electronAPI.log('error', `Can't view the file: ${e.toString()}`);
    } finally {
      await history.closeFile(fd);
    }
  } catch (e: any) {
    window.electronAPI.log('error', `Can't view the file: ${e.toString()}`);
  } finally {
    await history.stop();
  }
}

async function _getFileInfo(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  fileInfo: DetectedFileType,
  fileName: EntryName,
): Promise<FileContentInfo | undefined> {
  const statsResult = await entryStat(workspaceHandle, path);
  if (!statsResult.ok || !statsResult.value.isFile()) {
    return;
  }

  const openResult = await openFile(workspaceHandle, path, { read: true });
  if (!openResult.ok) {
    return;
  }
  const info: FileContentInfo = {
    data: new Uint8Array(Number((statsResult.value as EntryStatFile).size)),
    extension: fileInfo.extension,
    contentType: fileInfo.type,
    fileName: fileName,
    path: path,
    fileId: statsResult.value.id,
  };

  const fd = openResult.value;
  try {
    let loop = true;
    let offset = 0;
    while (loop) {
      const readResult = await readFile(workspaceHandle, openResult.value, offset, DEFAULT_READ_SIZE);
      if (!readResult.ok) {
        throw Error(JSON.stringify(readResult.error));
      }
      const buffer = new Uint8Array(readResult.value);
      info.data.set(buffer, offset);
      if (readResult.value.byteLength < DEFAULT_READ_SIZE) {
        loop = false;
      }
      offset += readResult.value.byteLength;
    }
    return info;
  } catch (e: any) {
    window.electronAPI.log('error', `Can't view the file: ${e.toString()}`);
  } finally {
    await closeFile(workspaceHandle, fd);
  }
}

async function loadFile(): Promise<void> {
  loaded.value = false;
  contentInfo.value = undefined;
  detectedFileType.value = null;
  atDateTime.value = undefined;
  handlerComponent.value = null;
  handlerReadyRef.value = false;
  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Failed to retrieve workspace handle');
    return;
  }
  const path = getDocumentPath();
  if (!path) {
    window.electronAPI.log('error', 'Failed to retrieve document path');
  }
  const fileName = await Path.filename(path);
  if (!fileName) {
    window.electronAPI.log('error', 'Failed to retrieve the file name');
    return;
  }

  const timestamp = Number(getCurrentRouteQuery().timestamp);
  if (!Number.isNaN(timestamp)) {
    atDateTime.value = DateTime.fromMillis(timestamp);
  }

  const fileInfoSerialized = getCurrentRouteQuery().fileTypeInfo;
  if (!fileInfoSerialized) {
    window.electronAPI.log('error', 'Failed to retrieve file type info');
    return;
  }
  const fileInfo: DetectedFileType = Base64.toObject(fileInfoSerialized) as DetectedFileType;
  detectedFileType.value = fileInfo;

  const info = timestamp
    ? await _getFileInfoAt(workspaceHandle, path, DateTime.fromMillis(timestamp), fileInfo, fileName)
    : await _getFileInfo(workspaceHandle, path, fileInfo, fileName);

  if (!info) {
    contentInfo.value = undefined;
    handlerComponent.value = null;
    handlerReadyRef.value = false;
    informationManager.present(
      new Information({
        message: 'fileViewers.errors.titles.genericError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    if (!timestamp) {
      await openWithSystem(path);
    }
    await navigateTo(Routes.Documents, { query: { workspaceHandle: workspaceHandle, documentPath: await Path.parent(path) } });
    return;
  }

  contentInfo.value = info;
  if (timestamp) {
    atDateTime.value = DateTime.fromMillis(timestamp);
  }

  // Load the appropriate component after file content is ready
  loadComponent();

  loaded.value = true;
}

function loadComponent(): void {
  // Set the handler component based on the file type
  switch (handlerMode.value) {
    case FileHandlerMode.Edit:
      handlerComponent.value = FileEditor;
      break;
    case FileHandlerMode.View:
      handlerComponent.value = FileViewer;
      break;
    default:
      handlerComponent.value = null;
  }
  if (!handlerComponent.value) {
    handlerReadyRef.value = false;
    throw new Error(`No component for file with extension '${detectedFileType.value!.extension}'`);
  }
}

async function checkSaved(): Promise<boolean> {
  if (saveState.value !== SaveState.Saved && saveState.value !== SaveState.None) {
    const answer = await askQuestion('fileEditors.saving.notSavedTitle', 'fileEditors.saving.notSavedQuestion', {
      yesText: 'fileEditors.saving.notSavedDiscard',
      noText: 'fileEditors.saving.notSavedStay',
    });
    if (answer === Answer.No) {
      return false;
    }
  }
  return true;
}

onBeforeRouteLeave(async () => {
  return await checkSaved();
});

onBeforeRouteUpdate(async () => {
  return await checkSaved();
});

onMounted(async () => {
  handlerMode.value = getFileHandlerMode();
  const query = getCurrentRouteQuery();
  readOnly.value = Boolean(query.readOnly);

  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    const infoResult = await getWorkspaceInfo(workspaceHandle);
    if (infoResult.ok) {
      isReader.value = infoResult.value.currentSelfRole === WorkspaceRole.Reader;
    }
  }

  const clientInfoResult = await getClientInfo();
  if (clientInfoResult.ok) {
    userInfo.value = clientInfoResult.value;
  } else {
    window.electronAPI.log('error', `Failed to retrieve user info ${JSON.stringify(clientInfoResult.error)}`);
  }

  await loadFile();

  hideHeader();

  // For FileHandler, we want to hide the sidebar but remember if it was visible
  // so we can restore it when leaving
  if (isSidebarMenuVisible.value) {
    hideSidebarMenu(false); // Don't persist - this is temporary
    sidebarMenuVisibleOnMounted.value = true;
  } else {
    sidebarMenuVisibleOnMounted.value = false;
  }

  if (savedIconRef.value) {
    attachMouseOverTooltip(savedIconRef.value.$el, 'fileEditors.saving.tooltipSaved');
  }
  if (unsavedIconRef.value) {
    attachMouseOverTooltip(unsavedIconRef.value.$el, 'fileEditors.saving.tooltipUnsaved');
  }
  if (errorIconRef.value) {
    attachMouseOverTooltip(errorIconRef.value.$el, 'fileEditors.saving.tooltipError');
  }
  if (offlineIconRef.value) {
    attachMouseOverTooltip(offlineIconRef.value.$el, 'fileEditors.saving.tooltipOffline');
  }
});

onUnmounted(async () => {
  cancelRouteWatch();
  // Ensure header is visible when leaving handler
  showHeader();

  // Restore sidebar menu visibility if it was visible when we hide it
  if (sidebarMenuVisibleOnMounted.value) {
    showSidebarMenu(false); // Don't persist - just restore the temporary state
  }
});

async function openWithSystem(path: FsPath): Promise<boolean> {
  if (!isDesktop()) {
    return false;
  }

  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Failed to retrieve workspace handle');
    return false;
  }

  const fileName = (await Path.filename(path)) ?? '';
  const result = await getSystemPath(workspaceHandle, path);

  if (!result.ok) {
    await informationManager.present(
      new Information({
        message: fileName ? { key: 'FoldersPage.open.fileFailed', data: { name: fileName } } : 'FoldersPage.open.fileFailedGeneric',
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    return false;
  } else {
    window.electronAPI.openFile(result.value);
    return true;
  }
}

async function copyPath(path: FsPath): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Failed to retrieve workspace handle');
    return;
  }
  copyPathLinkToClipboard(path, workspaceHandle, informationManager);
}

async function showDetails(): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    const entry = await entryStat(workspaceHandle, getDocumentPath());
    if (!entry.ok) {
      await informationManager.present(
        new Information({
          message: 'fileViewers.errors.statsFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      return;
    }
    const modal = await modalController.create({
      component: FileDetailsModal,
      cssClass: 'file-details-modal',
      componentProps: {
        entry: entry.value,
        workspaceHandle: workspaceHandle,
        ownProfile: userInfo.value ? userInfo.value.currentProfile : undefined,
      },
    });
    await modal.present();
    await modal.onWillDismiss();
  }
}

async function openEditor(path: FsPath): Promise<void> {
  // Here we want to ensure the router goes back to documents page
  // before opening the editor for routing purposes (history stack)
  // Could probably be improved with better routing management
  // maybe avoiding routing between viewer and editor by loading components directly
  await routerGoBack();
  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    await openPath(workspaceHandle, path, informationManager, fileOperationManager, { useEditor: true });
  }
}

async function onClick(event: MouseEvent): Promise<void> {
  event.preventDefault();
  if (!event.target) {
    return;
  }
  const target = event.target as Element;
  let linkTag!: HTMLAnchorElement;
  if (target.tagName === 'A') {
    linkTag = target as HTMLAnchorElement;
  } else if (target.tagName === 'SPAN' && target.parentElement && target.parentElement.tagName === 'A') {
    linkTag = target.parentElement as HTMLAnchorElement;
  } else {
    return;
  }
  // `.href` get resolved, `getAttribute('href') is what actually in the href`
  const link = linkTag.href;
  const href = linkTag.getAttribute('href') ?? '';
  // We exclude invalid links or anchors
  if (!link || href.startsWith('#') || !URL.canParse(link)) {
    return;
  }
  const answer = await askQuestion('fileViewers.openLink.title', 'fileViewers.openLink.question', { yesText: 'fileViewers.openLink.yes' });
  if (answer === Answer.Yes) {
    Env.Links.openUrl(link);
  }
}

async function downloadFile(): Promise<void> {
  const result = await openDownloadConfirmationModal(storageManager);
  if (result === MsModalResult.Cancel) {
    return;
  }

  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    window.electronAPI.log('error', 'Failed to retrieve workspace handle');
    return;
  }
  const workspaceInfoResult = await getWorkspaceInfo(workspaceHandle);
  if (!workspaceInfoResult.ok) {
    window.electronAPI.log(
      'error',
      `Failed to retrieve workspace info: ${workspaceInfoResult.error.tag} (${workspaceInfoResult.error.error})`,
    );
    return;
  }

  if (!contentInfo.value) {
    window.electronAPI.log('error', 'No content info when trying to download a file');
    return;
  }

  await downloadEntry({
    name: contentInfo.value.fileName,
    workspaceHandle: workspaceHandle,
    workspaceId: workspaceInfoResult.value.id,
    path: contentInfo.value.path,
    informationManager: informationManager,
    fileOperationManager: fileOperationManager,
  });
}

async function onSaveStateChange(newState: SaveState): Promise<void> {
  if (newState !== SaveState.Saved) {
    showSaveStateText.value = true;
  } else {
    setTimeout(() => {
      if (saveState.value === SaveState.Saved) {
        showSaveStateText.value = false;
      }
    }, 3000);
  }
  saveState.value = newState;
}

async function openSmallDisplayActionMenu(): Promise<void> {
  const modal = await modalController.create({
    component: SmallDisplayViewerActionMenu,
    cssClass: 'viewer-action-menu-modal',
    showBackdrop: true,
    breakpoints: [0, 0.5, 1],
    expandToScroll: false,
    initialBreakpoint: 0.5,
    componentProps: {},
  });
  await modal.present();
  const { data } = await modal.onDidDismiss();
  if (data !== undefined) {
    switch (data.action) {
      case FileHandlerAction.Details:
        await showDetails();
        break;
      case FileHandlerAction.CopyPath:
        if (contentInfo.value) {
          await copyPath(contentInfo.value.path);
        }
        break;
      case FileHandlerAction.Download:
        await downloadFile();
        break;
      case FileHandlerAction.OpenWithSystem:
        if (contentInfo.value) {
          await openWithSystem(contentInfo.value.path);
        }
        break;
      default:
        console.warn('No ViewerAction match found');
    }
  }
}
</script>

<style scoped lang="scss">
.container {
  height: 100%;
  background-color: var(--parsec-color-light-secondary-premiere);

  .file-handler {
    display: flex;
    flex-direction: column;
    height: 100%;

    &--loading {
      justify-content: center;
      flex-direction: column;
    }

    &-topbar {
      display: flex;
      align-items: center;
      gap: 1rem;
      padding: 1rem 2rem;
      border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
      background: var(--parsec-color-light-secondary-white);

      #trigger-toggle-menu-button {
        flex-shrink: 0;
        --fill-color: var(--parsec-color-light-secondary-grey);
        padding: 0.625rem;
        border-radius: var(--parsec-radius-12);
        cursor: pointer;
        &:hover {
          background: var(--parsec-color-light-secondary-premiere);
          --fill-color: var(--parsec-color-light-secondary-hard-grey);
        }
      }

      @include ms.responsive-breakpoint('sm') {
        padding: 0.75rem 1rem;
      }

      .file-icon {
        width: 2rem;
        height: 2rem;
        min-width: 2rem;
        min-height: 2rem;
        margin-left: 0.5rem;

        @include ms.responsive-breakpoint('sm') {
          width: 1.75rem;
          height: 1.75rem;
          min-width: 1.75rem;
          min-height: 1.75rem;
        }
      }

      &__title {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        overflow: hidden;

        ion-text {
          color: var(--parsec-color-light-secondary-text);
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }

        .subtitles-sm {
          color: var(--parsec-color-light-secondary-grey);
        }
      }

      &-buttons {
        margin-left: auto;
        display: flex;
        gap: 0.5rem;
        flex-shrink: 0;

        &:hover {
          border-color: var(--parsec-color-light-primary-100);
        }

        &__item:not(.toggle-menu) {
          color: var(--parsec-color-light-secondary-text);
          border-radius: var(--parsec-radius-8);
          transition: all 150ms linear;

          &::part(native) {
            --background: none;
            --background-hover: none;
            padding: 0.625rem 1rem;

            @include ms.responsive-breakpoint('sm') {
              padding: 0.5rem;
            }
          }

          &:hover {
            background: var(--parsec-color-light-secondary-medium);
            color: var(--parsec-color-light-secondary-text);
          }

          .item-icon {
            width: 1rem;
            font-size: 1rem;
            margin-right: 0.5rem;
            --fill-color: var(--parsec-color-light-secondary-text);

            @include ms.responsive-breakpoint('sm') {
              min-width: 1.125rem;
              width: 1.125rem;
              font-size: 1.125rem;
              margin-right: 0;
            }
          }
        }

        .action-menu {
          font-size: 1.125rem;
          margin-right: 0;

          @include ms.responsive-breakpoint('sm') {
            font-size: 1.25rem;
          }
        }

        .toggle-menu {
          margin-right: 0.5rem;
          position: relative;
          margin-left: 0.5rem;
          --background: none;
          color: var(--parsec-color-light-secondary-hard-grey);

          &::part(native) {
            --background: none;
            --background-hover: none;
          }

          ion-icon {
            font-size: 1rem;
            margin-right: 0.5rem;
            color: var(--parsec-color-light-secondary-hard-grey);

            @include ms.responsive-breakpoint('md') {
              margin-right: 0;
            }
          }

          @include ms.responsive-breakpoint('sm') {
            font-size: 1.125rem;
            margin-right: 0;
          }

          &:hover {
            color: var(--parsec-color-light-secondary-soft-text);

            ion-icon {
              color: var(--parsec-color-light-secondary-soft-text);
            }
          }

          &::before {
            content: '';
            position: absolute;
            top: 50%;
            left: -0.5rem;
            transform: translateY(-50%);
            width: 1px;
            height: 1.5rem;
            background: var(--parsec-color-light-secondary-disabled);
          }
        }
      }

      .save-info {
        display: flex;
        flex-shrink: 0;
        align-items: center;
        gap: 0.5rem;

        @include ms.responsive-breakpoint('sm') {
          min-width: 1.125rem;
          width: 1.125rem;
          font-size: 1.125rem;
          margin-right: 0;
        }

        &-text {
          text-overflow: ellipsis;

          &-fade-in {
            visibility: visible;
            opacity: 1;
            transition: opacity 0.5s linear;
          }

          &-fade-out {
            visibility: hidden;
            opacity: 0;
            transition:
              visibility 0s 3s,
              opacity 3s ease-in;
          }

          &-ok {
            color: var(--parsec-color-light-secondary-hard-grey);
          }

          &-ko {
            color: var(--parsec-color-light-danger-500);
          }
        }

        &-spinner {
          height: 1.125rem;
          width: 1.125rem;
          background: none;
        }

        &-icon {
          font-size: 1.125rem;
          width: 1.125rem;

          &-ok {
            color: var(--parsec-color-light-primary-600);
          }
          &-ko {
            color: var(--parsec-color-light-secondary-grey);
            --fill-color: var(--parsec-color-light-secondary-grey);
          }
        }
      }
    }
  }
}
</style>
