// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MsAlertModal, MsAlertModalConfig, MsReportTheme } from '@/components/core';
import { EntryName, FsPath, SizeInt, UserID, WorkspaceHandle, WorkspaceRole } from '@/parsec';
import { NotificationManager } from '@/services/notificationManager';
import { ToastManager } from '@/services/toastManager';
import { modalController } from '@ionic/vue';
import { v4 as uuid4 } from 'uuid';

export const InformationKey = 'information';

export enum InformationDataType {
  WorkspaceRoleChanged,
  NewWorkspaceAccess,
  UserJoinWorkspace,
  MultipleUsersJoinWorkspace,
  UserSharedDocument,
  AllImportedElements,
  NewDevice,
}

export interface AbstractInformationData {
  type: InformationDataType;
}

export interface WorkspaceRoleChangedData extends AbstractInformationData {
  type: InformationDataType.WorkspaceRoleChanged;
  workspaceHandle: WorkspaceHandle;
  oldRole: WorkspaceRole;
  newRole: WorkspaceRole;
}

// The account owner has been granted access to a workspace
export interface NewWorkspaceAccessData extends AbstractInformationData {
  type: InformationDataType.NewWorkspaceAccess;
  workspaceHandle: WorkspaceHandle;
  role: WorkspaceRole;
}

// A user has been granted access to a workspace
export interface UserJoinWorkspaceData extends AbstractInformationData {
  type: InformationDataType.UserJoinWorkspace;
  workspaceHandle: WorkspaceHandle;
  role: WorkspaceRole;
  userId: UserID;
}

// Multiple users have been granted access to a workspace
export interface MultipleUsersJoinWorkspaceData extends AbstractInformationData {
  type: InformationDataType.MultipleUsersJoinWorkspace;
  workspaceHandle: WorkspaceHandle;
  roles: {
    userId: UserID;
    role: WorkspaceRole;
  }[];
}

// A user has shared a document in a workspace
export interface UserSharedDocumentData extends AbstractInformationData {
  type: InformationDataType.UserSharedDocument;
  workspaceHandle: WorkspaceHandle;
  userId: UserID;
  fileName: EntryName;
  filePath: FsPath;
  fileSize: SizeInt;
}

// All elements the owner's account has been imported is done
export interface AllImportedElementsData extends AbstractInformationData {
  type: InformationDataType.AllImportedElements;
}

// A new device has been added to the account
export interface NewDeviceData extends AbstractInformationData {
  type: InformationDataType.NewDevice;
}

export type InformationData =
  | NewWorkspaceAccessData
  | WorkspaceRoleChangedData
  | UserJoinWorkspaceData
  | MultipleUsersJoinWorkspaceData
  | UserSharedDocumentData
  | AllImportedElementsData
  | NewDeviceData;

export enum InformationLevel {
  Info = 'INFO',
  Success = 'SUCCESS',
  Warning = 'WARNING',
  Error = 'ERROR',
}

export interface InformationOptions {
  message: string;
  level: InformationLevel;
  data?: InformationData;
}

export class Information {
  id: string;
  message: string;
  level: InformationLevel;
  data?: InformationData;
  constructor({ message, level, data }: InformationOptions) {
    this.id = uuid4();
    this.message = message;
    this.level = level;
    this.data = data;
  }

  get theme(): MsReportTheme {
    switch (this.level) {
      case InformationLevel.Info:
        return MsReportTheme.Info;
      case InformationLevel.Success:
        return MsReportTheme.Success;
      case InformationLevel.Warning:
        return MsReportTheme.Warning;
      case InformationLevel.Error:
        return MsReportTheme.Error;
    }
  }
}

export enum PresentationMode {
  Console = 1 << 0,
  Notification = 1 << 1,
  Toast = 1 << 2,
  Modal = 1 << 3,
}

export class InformationManager {
  toastManager: ToastManager;
  notificationManager: NotificationManager;

  constructor() {
    this.toastManager = new ToastManager();
    this.notificationManager = new NotificationManager();
  }

  private async showModal(information: Information): Promise<void> {
    const alertModalConfig: MsAlertModalConfig = {
      theme: information.theme,
      message: information.message,
    };

    await this._createAndPresentModal(alertModalConfig);
  }

  private async _createAndPresentModal(modalConfig: MsAlertModalConfig): Promise<any> {
    const top = await modalController.getTop();
    if (top) {
      top.classList.add('overlapped-modal');
    }
    const modal = await modalController.create({
      component: MsAlertModal,
      canDismiss: true,
      cssClass: 'information-modal',
      componentProps: modalConfig,
    });
    await modal.present();
    const result = await modal.onWillDismiss();
    await modal.dismiss();

    if (top) {
      top.classList.remove('overlapped-modal');
    }
    return result;
  }

  private async showToast(information: Information): Promise<void> {
    this.toastManager.createAndPresent({
      theme: information.theme,
      message: information.message,
    });
  }

  private sendNotification(information: Information): void {
    this.notificationManager.add(information);
  }

  async present(information: Information, presentationMode: PresentationMode): Promise<void> {
    if (presentationMode & PresentationMode.Modal && presentationMode & PresentationMode.Toast) {
      console.error('Error: forbidden usage, toasts and modals are not meant to be used at the same time.');
    }
    if (presentationMode & PresentationMode.Console) {
      this._trace(information);
    }
    if (presentationMode & PresentationMode.Notification) {
      this.sendNotification(information);
    }
    if (presentationMode & PresentationMode.Toast) {
      await this.showToast(information);
    }
    if (presentationMode & PresentationMode.Modal) {
      await this.showModal(information);
    }
  }

  private _trace(information: Information): void {
    console.trace(`[${information.level}] ${information.message}`);
  }
}
