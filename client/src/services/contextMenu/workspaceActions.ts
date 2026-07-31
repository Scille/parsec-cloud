// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { matchingStringValidator, workspaceNameValidator } from '@/common/validators';
import { formatWorkspaceDeletionDelay } from '@/components/workspaces/utils';
import {
  ClientRenameWorkspaceErrorTag,
  WorkspaceInfo,
  WorkspaceName,
  getClientInfo,
  getSystemPath,
  isDesktop,
  mountWorkspace,
  archiveWorkspace as parsecArchiveWorkspace,
  createWorkspace as parsecCreateWorkspace,
  getPathLink as parsecGetPathLink,
  renameWorkspace as parsecRenameWorkspace,
  restoreWorkspace as parsecRestoreWorkspace,
  trashWorkspace as parsecTrashWorkspace,
  selfPromoteToWorkspaceOwner,
  unmountWorkspace,
} from '@/parsec';
import { ClientArchiveWorkspaceErrorTag } from '@/plugins/libparsec';
import { Routes, navigateTo } from '@/router';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { recentDocumentManager } from '@/services/recentDocuments';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { useWorkspaceAttributes } from '@/services/workspaceAttributes';
import { WorkspaceMenu } from '@/views/workspaces';
import WorkspaceHiddenModal from '@/views/workspaces/WorkspaceHiddenModal.vue';
import WorkspaceSharingModal from '@/views/workspaces/WorkspaceSharingModal.vue';
import { modalController } from '@ionic/vue';
import { DateTime } from 'luxon';
import {
  Answer,
  Clipboard,
  I18n,
  MsModalResult,
  MsReportTheme,
  Translatable,
  askQuestion,
  getTextFromUser,
  useWindowSize,
} from 'megashark-lib';
import { Ref, inject } from 'vue';

export function useWorkspaceActions() {
  const { isLargeDisplay } = useWindowSize();
  const workspaceAttributes = useWorkspaceAttributes();
  const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
  const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
  const storageManager: StorageManager = inject(StorageManagerKey)!;

  async function takeOwnershipOfWorkspace(workspace: WorkspaceInfo): Promise<void> {
    const answer = await askQuestion(
      'WorkspacesPage.missingOwnership.title',
      { key: 'WorkspacesPage.missingOwnership.subtitle', data: { workspace: workspace.name } },
      {
        yesText: 'WorkspacesPage.missingOwnership.yes',
        noText: 'WorkspacesPage.missingOwnership.no',
      },
    );
    if (answer !== Answer.Yes) {
      return;
    }
    const result = await selfPromoteToWorkspaceOwner(workspace.id);
    if (!result.ok) {
      informationManager.value.present(
        new Information({
          message: {
            key: 'WorkspacesPage.missingOwnership.failed',
          },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.value.present(
        new Information({
          message: {
            key: 'WorkspacesPage.missingOwnership.success',
            data: {
              workspace: workspace.name,
            },
          },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  }

  async function restoreWorkspace(workspace: WorkspaceInfo): Promise<void> {
    const answer = await askQuestion(
      'WorkspacesPage.restoreWorkspace.title',
      {
        key: `WorkspacesPage.restoreWorkspace.subtitle${workspace.isTrashed ? 'Trashed' : 'Archived'}`,
        data: { workspace: workspace.name },
      },
      { yesText: 'WorkspacesPage.restoreWorkspace.yes', noText: 'WorkspacesPage.restoreWorkspace.no' },
    );
    if (answer === Answer.No) {
      return;
    }

    const result = await parsecRestoreWorkspace(workspace.id);

    informationManager.value.present(
      new Information({
        message: {
          key: result.ok ? 'WorkspacesPage.restoreWorkspace.restore.success' : 'WorkspacesPage.restoreWorkspace.restore.fail',
          data: { workspace: workspace.name },
        },
        level: result.ok ? InformationLevel.Success : InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }

  async function trashWorkspace(workspace: WorkspaceInfo) {
    let minimumArchivingPeriodInSeconds = 30 * 24 * 3600; // 30 days
    const clientResult = await getClientInfo();

    if (clientResult.ok) {
      minimumArchivingPeriodInSeconds = Number(clientResult.value.serverOrganizationConfig.realmMinimumArchivingPeriodBeforeDeletion);
    }
    // The real deletion date will only be determined once the realm archiving certificate is created.
    // Typically the more the user waits before accepting the confirmation prompt, the more the actual
    // deletion date will differs. In any case we are talking of just a couple of seconds of difference
    // which is no big deal since the archiving period is supposed to be a multiple-days long period.
    const estimatedDeletionDate = DateTime.now().plus({ seconds: minimumArchivingPeriodInSeconds });

    // Warn about the operation
    const answer = await askQuestion(
      'WorkspacesPage.trashWorkspace.question.title',
      {
        key:
          minimumArchivingPeriodInSeconds > 0
            ? 'WorkspacesPage.trashWorkspace.question.subtitleBin'
            : 'WorkspacesPage.trashWorkspace.question.subtitleDelete',
        data: {
          workspace: workspace.name,
          deletionDelay: I18n.translate(formatWorkspaceDeletionDelay(minimumArchivingPeriodInSeconds)),
        },
      },
      {
        yesText: 'WorkspacesPage.trashWorkspace.question.yes',
        noText: 'WorkspacesPage.trashWorkspace.question.no',
      },
    );
    if (answer === Answer.No) {
      return;
    }

    // Ask for confirmation by inputting the workspace name
    const workspaceName = await getTextFromUser(
      {
        title: 'WorkspacesPage.trashWorkspace.title',
        additionalMessage: {
          message: {
            key:
              minimumArchivingPeriodInSeconds > 0
                ? 'WorkspacesPage.trashWorkspace.subtitleBin'
                : 'WorkspacesPage.trashWorkspace.subtitleDelete',
            data: { workspace: workspace.name, deletionDate: I18n.translate(I18n.formatDate(estimatedDeletionDate)) },
          },
          theme: MsReportTheme.Warning,
        },
        trim: true,
        validator: matchingStringValidator(workspace.name),
        inputLabel: {
          key: 'WorkspacesPage.trashWorkspace.label',
          data: { workspace: workspace.name },
        },
        placeholder: I18n.valueAsTranslatable(workspace.name),
        okButtonText:
          minimumArchivingPeriodInSeconds > 0 ? 'WorkspacesPage.trashWorkspace.yesBin' : 'WorkspacesPage.trashWorkspace.yesDelete',
        yesIsDangerous: true,
      },
      isLargeDisplay.value,
    );

    if (!workspaceName || workspaceName.localeCompare(workspace.name) !== 0) {
      // Shouldn't happen
      return;
    }

    const result = await parsecTrashWorkspace(workspace.id, minimumArchivingPeriodInSeconds);

    let message: Translatable = '';
    let level: InformationLevel;

    if (result.ok) {
      message = {
        key:
          minimumArchivingPeriodInSeconds > 0
            ? 'WorkspacesPage.trashWorkspace.trash.successBin'
            : 'WorkspacesPage.trashWorkspace.trash.successDelete',
        data: { workspace: workspace.name },
      };
      level = InformationLevel.Success;
    } else {
      switch (result.error.tag) {
        case ClientArchiveWorkspaceErrorTag.ArchivingPeriodTooShort:
        //   message = 'WorkspacesPage.trashWorkspace.error.tooShort';
        //   break;
        // TODO: when implementing custom duration, either re-enable or delete locale
        case ClientArchiveWorkspaceErrorTag.Offline:
          message = 'WorkspacesPage.trashWorkspace.error.offline';
          break;
        case ClientArchiveWorkspaceErrorTag.WorkspaceDeleted:
        case ClientArchiveWorkspaceErrorTag.WorkspaceNotFound:
          message = 'WorkspacesPage.trashWorkspace.error.notFound';
          break;
        default:
          message = {
            key:
              minimumArchivingPeriodInSeconds > 0
                ? 'WorkspacesPage.trashWorkspace.error.failBin'
                : 'WorkspacesPage.trashWorkspace.error.failDelete',
            data: { workspace: workspace.name },
          };
      }
      level = InformationLevel.Error;
    }
    informationManager.value.present(new Information({ message: message, level: level }), PresentationMode.Toast);
  }

  async function showWorkspace(workspace: WorkspaceInfo): Promise<boolean> {
    if (isDesktop()) {
      const result = await mountWorkspace(workspace.handle);

      if (!result.ok) {
        informationManager.value.present(
          new Information({
            message: {
              key: 'WorkspacesPage.showHideWorkspace.failedShown',
              data: { workspace: workspace.name },
            },
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
        return false;
      } else {
        workspaceAttributes.removeHidden(workspace.id);
        await workspaceAttributes.save();

        await eventDistributor.value.dispatchEvent(Events.WorkspaceMountpointsSync, {
          workspaceId: workspace.id,
          isMounted: true,
        });
      }
    } else {
      workspaceAttributes.removeHidden(workspace.id);
      await workspaceAttributes.save();
    }

    informationManager.value.present(
      new Information({
        message: {
          key: isDesktop() ? 'WorkspacesPage.showHideWorkspace.successDesktopShown' : 'WorkspacesPage.showHideWorkspace.successWebShown',
          data: { workspace: workspace.name },
        },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );

    return true;
  }

  async function hideWorkspace(workspace: WorkspaceInfo): Promise<void> {
    if (isDesktop()) {
      const result = await unmountWorkspace(workspace);

      if (!result.ok) {
        informationManager.value.present(
          new Information({
            message: {
              key: 'WorkspacesPage.showHideWorkspace.failedHidden',
              data: { workspace: workspace.name },
            },
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
        return;
      } else {
        workspaceAttributes.addHidden(workspace.id);
        await workspaceAttributes.save();

        await eventDistributor.value.dispatchEvent(Events.WorkspaceMountpointsSync, {
          workspaceId: workspace.id,
          isMounted: false,
        });
      }
    } else {
      workspaceAttributes.addHidden(workspace.id);
      await workspaceAttributes.save();
    }

    informationManager.value.present(
      new Information({
        message: {
          key: isDesktop() ? 'WorkspacesPage.showHideWorkspace.successDesktopHidden' : 'WorkspacesPage.showHideWorkspace.successWebHidden',
          data: { workspace: workspace.name },
        },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }

  async function seeInExplorer(workspace: WorkspaceInfo): Promise<void> {
    if (workspaceAttributes.isHidden(workspace.id)) {
      const answer = await askQuestion(
        'WorkspacesPage.openInExplorerModal.workspace.title',
        'WorkspacesPage.openInExplorerModal.workspace.description',
        {
          yesText: 'WorkspacesPage.openInExplorerModal.actionConfirm',
          noText: 'WorkspacesPage.openInExplorerModal.actionCancel',
        },
      );

      if (answer === Answer.No) {
        return;
      }
      await showWorkspace(workspace);
    }

    const result = await getSystemPath(workspace.handle, '/');
    if (!result.ok) {
      await informationManager.value.present(
        new Information({
          message: { key: 'FoldersPage.open.folderFailed', data: { name: workspace.name } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      window.nativeAPI.openFile(result.value);
    }
  }

  async function renameWorkspace(workspace: WorkspaceInfo, newName: WorkspaceName): Promise<void> {
    const result = await parsecRenameWorkspace(newName, workspace.id);
    if (result.ok) {
      informationManager.value.present(
        new Information({
          message: { key: 'WorkspacesPage.RenameWorkspaceModal.success', data: { newName: newName } },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
      recentDocumentManager.updateWorkspace(workspace.id, { name: newName });
    } else {
      let message: Translatable = '';
      switch (result.error.tag) {
        case ClientRenameWorkspaceErrorTag.AuthorNotAllowed ||
          ClientRenameWorkspaceErrorTag.InvalidCertificate ||
          ClientRenameWorkspaceErrorTag.InvalidEncryptedRealmName ||
          ClientRenameWorkspaceErrorTag.InvalidKeysBundle:
          message = 'WorkspacesPage.RenameWorkspaceModal.errors.permission';
          break;
        case ClientRenameWorkspaceErrorTag.Offline:
          message = 'WorkspacesPage.RenameWorkspaceModal.errors.offline';
          break;
        default:
          message = { key: 'WorkspacesPage.RenameWorkspaceModal.errors.generic', data: { reason: result.error.tag } };
          console.error(result.error.tag);
          break;
      }
      informationManager.value.present(
        new Information({
          message: message,
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }

  async function unmountWorkspaceWithConfirmation(workspace: WorkspaceInfo): Promise<void> {
    const config = await storageManager.retrieveConfig();

    if (config.skipWorkspaceHiddenWarning === true) {
      await hideWorkspace(workspace);
      return;
    }

    const modal = await modalController.create({
      component: WorkspaceHiddenModal,
      cssClass: 'workspace-hidden-modal',
      componentProps: {
        workspaceName: workspace.name,
      },
    });

    await modal.present();
    const { data, role } = await modal.onWillDismiss();
    await modal.dismiss();

    if (role === MsModalResult.Confirm) {
      if (data?.skipWorkspaceHiddenWarning === true) {
        config.skipWorkspaceHiddenWarning = true;
        await storageManager.storeConfig(config);
      }

      await hideWorkspace(workspace);
    }
  }

  async function openRenameWorkspaceModal(workspace: WorkspaceInfo): Promise<void> {
    const newWorkspaceName = await getTextFromUser(
      {
        title: 'WorkspacesPage.RenameWorkspaceModal.pageTitle',
        trim: true,
        validator: workspaceNameValidator,
        inputLabel: 'WorkspacesPage.RenameWorkspaceModal.label',
        placeholder: 'WorkspacesPage.RenameWorkspaceModal.placeholder',
        okButtonText: 'WorkspacesPage.RenameWorkspaceModal.rename',
        defaultValue: workspace.name,
        selectionRange: [0, workspace.name.length],
      },
      isLargeDisplay.value,
    );

    if (newWorkspaceName) {
      await renameWorkspace(workspace, newWorkspaceName);
    }
  }

  async function copyLinkToClipboard(workspace: WorkspaceInfo): Promise<void> {
    const result = await parsecGetPathLink(workspace.handle, '/');

    if (result.ok) {
      const [_, invitationAddrAsHttpRedirection] = result.value;
      if (!(await Clipboard.writeText(invitationAddrAsHttpRedirection))) {
        informationManager.value.present(
          new Information({
            message: 'WorkspacesPage.linkNotCopiedToClipboard',
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
      } else {
        informationManager.value.present(
          new Information({
            message: 'WorkspacesPage.linkCopiedToClipboard',
            level: InformationLevel.Info,
          }),
          PresentationMode.Toast,
        );
      }
    } else {
      informationManager.value.present(
        new Information({
          message: 'WorkspacesPage.getLinkError',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }

  async function archiveWorkspace(workspace: WorkspaceInfo): Promise<void> {
    const answer = await askQuestion(
      'WorkspacesPage.archiveWorkspace.title',
      { key: 'WorkspacesPage.archiveWorkspace.subtitle', data: { workspace: workspace.name } },
      { yesText: 'WorkspacesPage.archiveWorkspace.yes', noText: 'WorkspacesPage.archiveWorkspace.no' },
    );
    if (answer === Answer.No) {
      return;
    }

    const result = await parsecArchiveWorkspace(workspace.id);
    informationManager.value.present(
      new Information({
        message: {
          key: result.ok ? 'WorkspacesPage.archiveWorkspace.archive.success' : 'WorkspacesPage.archiveWorkspace.archive.fail',
          data: { workspace: workspace.name },
        },
        level: result.ok ? InformationLevel.Success : InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }

  async function workspaceShareClick(workspace: WorkspaceInfo): Promise<void> {
    const modal = await modalController.create({
      component: WorkspaceSharingModal,
      showBackdrop: true,
      handle: false,
      breakpoints: isLargeDisplay ? undefined : [0, 1],
      expandToScroll: false,
      initialBreakpoint: isLargeDisplay ? undefined : 1,
      componentProps: {
        workspaceId: workspace.id,
        workspaceName: workspace.name,
        ownRole: workspace.selfRole,
        informationManager: informationManager.value,
        eventDistributor: eventDistributor.value,
      },
      cssClass: 'workspace-sharing-modal',
    });
    await modal.present();
    await modal.onWillDismiss();
  }

  async function openCreateWorkspaceModal(): Promise<void> {
    const workspaceName = await getTextFromUser(
      {
        title: 'WorkspacesPage.CreateWorkspaceModal.pageTitle',
        trim: true,
        validator: workspaceNameValidator,
        inputLabel: 'WorkspacesPage.CreateWorkspaceModal.label',
        placeholder: 'WorkspacesPage.CreateWorkspaceModal.placeholder',
        okButtonText: 'WorkspacesPage.CreateWorkspaceModal.create',
      },
      isLargeDisplay.value,
    );

    if (!workspaceName) {
      return;
    }
    await createWorkspace(workspaceName);
  }

  async function createWorkspace(name: WorkspaceName): Promise<void> {
    const result = await parsecCreateWorkspace(name);
    if (result.ok) {
      informationManager.value.present(
        new Information({
          message: {
            key: 'WorkspacesPage.newWorkspaceSuccess',
            data: {
              workspace: name,
            },
          },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );

      await navigateTo(Routes.Workspaces, { query: { workspaceMenu: WorkspaceMenu.All } });
    } else {
      informationManager.value.present(
        new Information({
          message: 'WorkspacesPage.newWorkspaceError',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }

  return {
    takeOwnershipOfWorkspace,
    restoreWorkspace,
    trashWorkspace,
    showWorkspace,
    hideWorkspace,
    seeInExplorer,
    renameWorkspace,
    archiveWorkspace,
    copyLinkToClipboard,
    openRenameWorkspaceModal,
    workspaceShareClick,
    unmountWorkspaceWithConfirmation,
    createWorkspace,
    openCreateWorkspaceModal,
  };
}
