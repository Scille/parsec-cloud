// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { SearchResult } from '@/parsec';
import { Translatable } from 'megashark-lib';

export interface FileControlsDropdownItemContent {
  label: Translatable;
  callback?: () => Promise<any>;
  icon?: string;
  image?: string;
  children?: FileControlsDropdownItemContent[];
  isActive?: boolean;
  dismissPopover?: boolean;
  dismissToParent?: boolean;
}

export interface FileSearch {
  results: Array<SearchResult>;
  pattern: string;
  active: boolean;
}
