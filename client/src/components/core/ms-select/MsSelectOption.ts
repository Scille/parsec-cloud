// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export interface MsSelectOption {
  label: string,
  key: string,
  disabled?: boolean,
}

export interface MsSelectSortByLabels {
  asc: string,
  desc: string
}

export interface MsSelectChangeEvent {
  option: MsSelectOption,
  sortByAsc: boolean
}

export function getOptionByKey(options: MsSelectOption[], key: string): MsSelectOption | undefined {
  return options.find((option) => {
    return option.key === key;
  });
}
