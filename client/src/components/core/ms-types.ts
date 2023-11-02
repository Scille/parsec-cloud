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
  label: string,
  key: any,
  disabled?: boolean,
}

export function getMsOptionByKey(options: MsOption[], key: any): MsOption | undefined {
  return options.find((option) => {
    return option.key === key;
  });
}

export type MsDropdownOption = MsOption;
export type MsSorterOption = MsOption;

export interface MsSorterLabels {
  asc: string,
  desc: string,
}
export interface MsDropdownChangeEvent {
  option: MsDropdownOption,
}

export interface MsSorterChangeEvent {
  option: MsSorterOption,
  sortByAsc: boolean,
}
