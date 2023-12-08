// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export enum MsReportTheme {
  Info = 'ms-info',
  Success = 'ms-success',
  Warning = 'ms-warning',
  Error = 'ms-error',
}

export enum MsModalResult {
  Cancel = 'cancel',
  Confirm = 'confirm',
}

interface MsOption {
  label: string;
  description?: string;
  key: any;
  disabled?: boolean;
}

export function getMsOptionByKey(options: MsOption[], key: any): MsOption | undefined {
  return options.find((option) => {
    return option.key === key;
  });
}

export type MsDropdownOption = MsOption;
export type MsSorterOption = MsOption;

export interface MsSorterLabels {
  asc: string;
  desc: string;
}
export interface MsDropdownChangeEvent {
  option: MsDropdownOption;
}

export interface MsSorterChangeEvent {
  option: MsSorterOption;
  sortByAsc: boolean;
}

// replace this by Map to manage order of extensions and loop on it when trying to fetch file
export const msImageExtensions: Array<string> = ['svg', 'png', 'webp'];

export enum MsImageName {
  CaretExpand = 'caret-expand',
  Device = 'device',
  FileImport = 'file-import',
  LogoIconGradient = 'logo-icon-gradient',
  LogoRowWhite = 'logo-row-white',
  PasswordLock = 'password-lock',
  SwapArrows = 'swap-arrows',
  WavyCaretUp = 'wavy-caret-up',
}

export interface MsImageResource {
  name: MsImageName;
  data: string;
  isSvg: boolean;
}
