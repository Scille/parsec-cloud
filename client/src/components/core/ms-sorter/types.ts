// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MsOption } from '@/components/core/ms-types';
import { Translatable } from 'megashark-lib';

export interface MsSorterLabels {
  asc: Translatable;
  desc: Translatable;
}

export interface MsSorterChangeEvent {
  option: MsOption;
  sortByAsc: boolean;
}
