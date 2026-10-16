<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
              <ion-text class="file-title">
                {{ contentInfo.fileName }}
              </ion-text>
              <ion-text
                class="file-date"
                v-if="contentInfo.timestamp"
              >
                {{ $msTranslate(I18n.formatDate(contentInfo.timestamp)) }}
              </ion-text>
            </div>
            <div
              v-if="isComponentEditor() && readOnly"
              class="save-info read-only"
            >
              <ion-icon
                :icon="eye"
                class="read-only-icon"
              />
              {{ $msTranslate('fileEditors.saving.readOnly') }}
            </div>
            <div
              v-if="!readOnly && saveState !== SaveState.None"
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
                v-show="showSaveStateText && isLargeDisplay"
                class="save-info-text"
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
                class="file-handler-topbar-buttons__item color-secondary"
                id="file-handler-details"
                @click="showDetails"
                :disabled="pathOpener.currentlyOpening.value"
                fill="clear"
              >
                <ion-icon
                  :icon="informationCircle"
                  class="item-icon button-icon-left"
                />
                {{ $msTranslate('fileViewers.details') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item color-secondary"
                id="file-handler-copy-link"
                @click="copyLink(contentInfo.path)"
                :disabled="pathOpener.currentlyOpening.value"
                fill="clear"
              >
                <ion-icon
                  :icon="link"
                  class="item-icon button-icon-left"
                />
                {{ $msTranslate('fileViewers.copyLink') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item color-secondary"
                id="file-handler-open-editor"
                @click="openEditor(contentInfo.path)"
                :disabled="pathOpener.currentlyOpening.value"
                fill="clear"
                v-if="!contentInfo.timestamp && readOnly && isEditicsEnabledForDocumentType(contentInfo.contentType) && !isReader"
              >
                <ion-icon
                  :icon="create"
                  class="item-icon button-icon-left"
                />
                {{ $msTranslate('fileViewers.openInEditor') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item color-secondary"
                @click="downloadFile"
                v-if="isWeb()"
                :disabled="pathOpener.currentlyOpening.value"
                fill="clear"
              >
                <ms-image
                  :image="DownloadIcon"
                  class="item-icon button-icon-left"
                />
                {{ $msTranslate('fileViewers.download') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item color-secondary"
                id="file-handler-open-with-system"
                @click="openWithSystem(contentInfo.path)"
                v-show="isDesktop() && !contentInfo.timestamp"
                :disabled="pathOpener.currentlyOpening.value"
                fill="clear"
              >
                <ion-icon
                  :icon="open"
                  class="item-icon button-icon-left"
                />
                {{ $msTranslate('fileViewers.openWithDefault') }}
              </ion-button>
              <ion-button
                class="file-handler-topbar-buttons__item toggle-menu color-secondary"
                @click="toggleMainHeader"
                :class="{ 'header-visible': isHeaderVisible() }"
              >
                <ion-icon
                  :icon="isHeaderVisible() ? chevronUp : chevronDown"
                  class="button-icon-left"
                />
                <span v-if="windowWidth > WindowSizeBreakpoints.MD">
                  {{ $msTranslate(isHeaderVisible() ? 'fileViewers.hideMenu' : 'fileViewers.showMenu') }}
                </span>
              </ion-button>
            </div>
            <ion-button
              class="file-handler-topbar-buttons__item action-menu"
              slot="icon-only"
              @click="openSmallDisplayActionMenu"
              v-else
            >
              <ion-icon :icon="ellipsisHorizontal" />
            </ion-button>
          </div>

          <!-- file-handler sub-component -->
          <component
            ref="handlerRef"
            v-if="contentInfo"
            :is="handlerComponent"
            :content-info="contentInfo"
            @file-loaded="pathOpener.pathOpened()"
            @file-error="pathOpener.pathOpened()"
            @on-save-state-change="onSaveStateChange"
            :read-only="readOnly"
            v-bind="isComponentEditor() ? { userInfo: userInfo } : {}"
          />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { getFileIcon, getFileStats } from '@/common/file';
import { DetectedFileType } from '@/common/fileTypes';
import { copyPathLinkToClipboard } from '@/components/files';
import HeaderBackButton from '@/components/header/HeaderBackButton.vue';
import {
  ClientInfo,
  entryStat,
  EntryStatFile,
  FsPath,
  getClientInfo,
  getSystemPath,
  getWorkspaceInfo,
  isDesktop,
  isMobile,
  isWeb,
  Path,
  WorkspaceRole,
} from '@/parsec';
import {
  currentRouteIs,
  getCurrentRouteQuery,
  getDocumentPath,
  getFileHandlerMode,
  getWorkspaceHandle,
  routerGoBack,
  Routes,
  watchRoute,
} from '@/router';
import { isEditicsEnabledForDocumentType } from '@/services/editics';
import { Env } from '@/services/environment';
import useHeaderControl from '@/services/headerControl';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import usePathOpener from '@/services/pathOpener';
import useSidebarMenu from '@/services/sidebarMenu';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { downloadFiles, FileDetailsModal, FileHandlerAction, openDownloadConfirmationModal } from '@/views/files';
import { FileHandlerMode, SaveState } from '@/views/files/handler';
import { FileEditor } from '@/views/files/handler/editor';
import SmallDisplayViewerActionMenu from '@/views/files/handler/SmallDisplayViewerActionMenu.vue';
import { FileViewer } from '@/views/files/handler/viewer';
import { FileContentInfo } from '@/views/files/handler/viewer/utils';
import { IonButton, IonContent, IonIcon, IonPage, IonText, modalController } from '@ionic/vue';
import {
  alert,
  chevronDown,
  chevronUp,
  create,
  ellipsisHorizontal,
  eye,
  informationCircle,
  link,
  open,
  save,
  warning,
} from 'ionicons/icons';
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
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const contentInfo: Ref<FileContentInfo | undefined> = ref(undefined);
const loaded = ref(false);
const { isHeaderVisible, toggleHeader: toggleMainHeader, showHeader, hideHeader } = useHeaderControl();
const { isVisible: isSidebarMenuVisible, hide: hideSidebarMenu, show: showSidebarMenu } = useSidebarMenu();
const handlerComponent: Ref<typeof FileEditor | typeof FileViewer | null> = shallowRef(null);
const handlerRef = useTemplateRef<InstanceType<typeof FileEditor>>('handlerRef');
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
const pathOpener = usePathOpener();

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
    contentInfo.value.workspaceHandle === getWorkspaceHandle() &&
    contentInfo.value.timestamp?.toMillis() === timestamp &&
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

async function loadFile(): Promise<boolean> {
  loaded.value = false;
  contentInfo.value = undefined;
  handlerComponent.value = null;
  const workspaceHandle = getWorkspaceHandle();
  let atTime: DateTime | undefined = undefined;
  if (!workspaceHandle) {
    window.nativeAPI.log('error', 'Failed to retrieve workspace handle');
    return false;
  }
  const path = getDocumentPath();
  if (!path) {
    window.nativeAPI.log('error', 'Failed to retrieve document path');
  }
  const fileName = await Path.filename(path);
  if (!fileName) {
    window.nativeAPI.log('error', 'Failed to retrieve the file name');
    return false;
  }

  const timestamp = Number(getCurrentRouteQuery().timestamp);
  if (!Number.isNaN(timestamp)) {
    atTime = DateTime.fromMillis(timestamp);
  }

  const stats = await getFileStats(workspaceHandle, path, atTime);
  if (!stats) {
    window.nativeAPI.log('error', 'Failed to get file stats');
    return false;
  }

  const fileInfoSerialized = getCurrentRouteQuery().fileTypeInfo;
  if (!fileInfoSerialized) {
    window.nativeAPI.log('error', 'Failed to retrieve file type info');
    return false;
  }
  const fileInfo: DetectedFileType = Base64.toObject(fileInfoSerialized) as DetectedFileType;

  if (!fileInfo) {
    window.nativeAPI.log('error', 'Failed to deserialize file type info');
    return false;
  }

  contentInfo.value = {
    extension: fileInfo.extension,
    contentType: fileInfo.type,
    fileName: fileName,
    path: path,
    workspaceHandle: workspaceHandle,
    timestamp: atTime,
    size: stats.size,
  };

  // Load the appropriate component after file content is ready
  loadComponent();

  loaded.value = true;
  return true;
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
    throw new Error(`No component for file with extension '${contentInfo.value?.extension}'`);
  }
}

async function checkSaved(): Promise<boolean> {
  // Only attempt to save if the editor is active and the document was loaded
  if (saveState.value === SaveState.None) {
    return true;
  }
  // Always try to save when leaving the editor, since the editor/viewer may have
  // unsaved changes not yet reported via `onSaveStateChange` (and saving a document
  // with no modification is a noop).
  if (handlerRef.value?.save) {
    const saved = await handlerRef.value.save();
    if (saved) {
      return true;
    }
    // Save failed, ask the user
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
      isReader.value = infoResult.value.selfRole === WorkspaceRole.Reader;
    }
  }

  const clientInfoResult = await getClientInfo();
  if (clientInfoResult.ok) {
    userInfo.value = clientInfoResult.value;
  } else {
    window.nativeAPI.log('error', `Failed to retrieve user info ${JSON.stringify(clientInfoResult.error)}`);
  }

  if (!(await loadFile())) {
    informationManager.value.present(
      new Information({
        message: 'fileViewers.errors.titles.genericError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    loaded.value = true;
    pathOpener.pathOpened();
    await routerGoBack();
    return;
  }

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
    window.nativeAPI.log('error', 'Failed to retrieve workspace handle');
    return false;
  }

  const fileName = (await Path.filename(path)) ?? '';
  const result = await getSystemPath(workspaceHandle, path);

  if (!result.ok) {
    await informationManager.value.present(
      new Information({
        message: fileName ? { key: 'FoldersPage.open.fileFailed', data: { name: fileName } } : 'FoldersPage.open.fileFailedGeneric',
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    return false;
  } else {
    window.nativeAPI.openFile(result.value);
    return true;
  }
}

async function copyLink(path: FsPath): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    window.nativeAPI.log('error', 'Failed to retrieve workspace handle');
    return;
  }
  copyPathLinkToClipboard(path, workspaceHandle, informationManager.value);
}

async function showDetails(): Promise<void> {
  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    const entry = await entryStat(workspaceHandle, getDocumentPath());
    if (!entry.ok) {
      await informationManager.value.present(
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
  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    await pathOpener.openPath(workspaceHandle, path, { readOnly: false });
  }
}

async function onClick(event: MouseEvent): Promise<void> {
  event.preventDefault();
  if (!event.target) {
    return;
  }
  // Links can contain other elements (`<strong>`, `<code>`, `<img>`, ...)
  const linkTag = (event.target as Element).closest('a');
  if (!linkTag) {
    return;
  }
  // Use the raw attribute rather than `.href`, which gets resolved against the app URL.
  // Parsing it without a base rejects relative links and anchors, which would otherwise
  // open the app itself (a new window on Electron).
  const href = linkTag.getAttribute('href') ?? '';
  if (!URL.canParse(href)) {
    informationManager.value.present(
      new Information({
        message: 'fileViewers.openLink.invalid',
        level: InformationLevel.Warning,
      }),
      PresentationMode.Toast,
    );
    return;
  }
  const url = new URL(href);
  if (!['http:', 'https:'].includes(url.protocol)) {
    informationManager.value.present(
      new Information({
        message: 'fileViewers.openLink.invalid',
        level: InformationLevel.Warning,
      }),
      PresentationMode.Toast,
    );
    return;
  }
  const answer = await askQuestion('fileViewers.openLink.title', 'fileViewers.openLink.question', { yesText: 'fileViewers.openLink.yes' });
  if (answer === Answer.Yes) {
    Env.Links.openUrl(url.href);
  }
}

async function downloadFile(): Promise<void> {
  const result = await openDownloadConfirmationModal(storageManager);
  if (result === MsModalResult.Cancel) {
    return;
  }

  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    window.nativeAPI.log('error', 'Failed to retrieve workspace handle');
    return;
  }
  if (!contentInfo.value) {
    window.nativeAPI.log('error', 'No content info when trying to download a file');
    return;
  }
  const entryResult = await entryStat(workspaceHandle, contentInfo.value.path);

  if (!entryResult.ok) {
    window.nativeAPI.log('error', 'Failed to stat entry when trying to download a file');
    return;
  }
  if (!entryResult.value.isFile()) {
    window.nativeAPI.log('error', 'Entry is not a file');
    return;
  }

  await downloadFiles({
    entries: [entryResult.value as EntryStatFile],
    workspaceHandle: workspaceHandle,
    informationManager: informationManager.value,
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
  if (!contentInfo.value) {
    return;
  }
  const modal = await modalController.create({
    component: SmallDisplayViewerActionMenu,
    cssClass: 'viewer-action-menu-modal',
    showBackdrop: true,
    breakpoints: [0, 0.5, 1],
    expandToScroll: false,
    initialBreakpoint: 0.5,
    componentProps: {
      canOpenWithSystem: !contentInfo.value.timestamp && isDesktop(),
      canEdit:
        !contentInfo.value.timestamp && readOnly.value && isEditicsEnabledForDocumentType(contentInfo.value.contentType) && !isReader.value,
    },
  });
  await modal.present();
  const { data } = await modal.onDidDismiss();
  if (data !== undefined) {
    switch (data.action) {
      case FileHandlerAction.Details:
        await showDetails();
        break;
      case FileHandlerAction.CopyLink:
        await copyLink(contentInfo.value.path);
        break;
      case FileHandlerAction.Download:
        await downloadFile();
        break;
      case FileHandlerAction.Edit:
        await openEditor(contentInfo.value.path);
        break;
      case FileHandlerAction.OpenWithSystem:
        if (contentInfo.value) {
          await openWithSystem(contentInfo.value.path);
        }
        break;
      default:
        window.nativeAPI.log('warn', `No match for selected viewer action '${data.action}'`);
        break;
    }
  }
}
</script>

<style scoped lang="scss">
.container {
  height: 100%;

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
      gap: ms.spacing('gap-3xl');
      padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
      border-bottom: ms.border('thin') solid ms.color('border-base-default');
      background: ms.color('surface-base-page');

      #trigger-toggle-menu-button {
        flex-shrink: 0;
        --fill-color: #{ms.color('icon-neutral-default')};
        padding: ms.spacing('padding-xl');
        border-radius: ms.radius('2xl');
        cursor: pointer;

        &:hover {
          background: ms.color('surface-base-page');
          --fill-color: #{ms.color('icon-neutral-default-hover')};
        }
      }

      @include ms.responsive-breakpoint('sm') {
        padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl');
      }

      .file-icon {
        width: 2rem;
        height: 2rem;
        min-width: 2rem;
        min-height: 2rem;
        margin-left: ms.spacing('padding-lg');

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
        gap: ms.spacing('gap-sm');
        overflow: hidden;

        ion-text {
          color: ms.color('text-base-heading');
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }

        .file-title {
          @include ms.font('body-lg-medium');
        }

        .file-date {
          @include ms.font('body-md-medium');
          color: ms.color('text-base-description');
        }
      }

      &-buttons {
        margin-left: auto;
        display: flex;
        gap: ms.spacing('gap-lg');
        flex-shrink: 0;

        &:hover {
          border-color: ms.color('border-brand-default-subtle');
        }

        .item-icon {
          --fill-color: #{ms.color('icon-neutral-default')};
          max-width: 1rem;
        }

        .action-menu {
          font-size: 1.125rem;
          margin-right: 0;

          @include ms.responsive-breakpoint('sm') {
            font-size: 1.25rem;
          }
        }

        .toggle-menu {
          margin-right: ms.spacing('padding-lg');
          position: relative;
          margin-left: ms.spacing('padding-lg');
          --background: none;
          color: ms.color('text-base-label');
          opacity: ms.opacity('7');

          &::part(native) {
            --background: none;
            --background-hover: none;
          }

          ion-icon {
            font-size: 1rem;
            margin-right: ms.spacing('padding-lg');

            @include ms.responsive-breakpoint('md') {
              margin-right: 0;
            }
          }

          @include ms.responsive-breakpoint('sm') {
            font-size: 1.125rem;
            margin-right: 0;
          }

          &:hover {
            opacity: ms.opacity('10');
          }

          &::before {
            content: '';
            position: absolute;
            top: 50%;
            left: calc(-1 * ms.spacing('padding-lg'));
            transform: translateY(-50%);
            width: 1px;
            height: 1.5rem;
            background: ms.color('border-base-default');
          }
        }
      }

      .save-info {
        display: flex;
        flex-shrink: 0;
        align-items: center;
        gap: ms.spacing('gap-lg');

        @include ms.responsive-breakpoint('sm') {
          min-width: 1.125rem;
          width: 1.125rem;
          font-size: 1.125rem;
          margin-right: 0;
        }

        &.read-only {
          @include ms.font('body-md-medium');
          background: ms.color('surface-neutral-default-subtle');
          color: ms.color('text-base-heading');
          padding: ms.spacing('padding-sm') ms.spacing('padding-2xl');
          border-radius: ms.radius('2xl');
          display: flex;
          align-items: center;
          gap: ms.spacing('gap-lg');

          &-icon {
            font-size: 1rem;
          }
        }

        &-text {
          @include ms.font('label-sm-medium');
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
            color: ms.color('text-base-description');
          }

          &-ko {
            color: ms.color('text-error-default');
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
            color: ms.color('icon-brand-default');
          }
          &-ko {
            color: ms.color('icon-neutral-default');
            --fill-color: #{ms.color('icon-neutral-default')};
          }
        }
      }
    }
  }
}
</style>
