// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { IValidator } from '@/common/validators';
import { MsReportTheme } from '@/components/core/ms-types';
import { FsPath } from '@/parsec';

export enum Answer {
  No = 0,
  Yes = 1,
}

export interface FolderSelectionOptions {
  title: string;
  subtitle?: string;
  startingPath: FsPath;
}

export interface GetTextOptions {
  title: string;
  subtitle?: string;
  trim?: boolean;
  validator?: IValidator;
  inputLabel?: string;
  placeholder?: string;
  okButtonText?: string;
  defaultValue?: string;
}

export interface GetPasswordOptions {
  title: string;
  subtitle?: string;
  inputLabel?: string;
  okButtonText?: string;
}

export interface MsAlertModalConfig {
  title?: string;
  theme: MsReportTheme;
  message: string;
}

export interface MsModalConfig {
  title?: string;
  theme?: MsReportTheme;
  subtitle?: string;
  closeButton?: {
    visible: boolean;
    onClick?: () => Promise<boolean>;
  };
  cancelButton?: {
    disabled: boolean;
    label: string;
    onClick?: () => Promise<boolean>;
  };
  confirmButton?: {
    disabled: boolean;
    label: string;
    onClick?: () => Promise<boolean>;
  };
}

export enum MsModalResult {
  Cancel = 'cancel',
  Confirm = 'confirm',
}
