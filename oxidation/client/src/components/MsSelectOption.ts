export interface MsSelectOption {
  label: string,
  key: string
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
