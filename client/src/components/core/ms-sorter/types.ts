// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MsOption } from '@/components/core/ms-types';

export interface MsSorterLabels {
  asc: string;
  desc: string;
}

export interface MsSorterChangeEvent {
  option: MsOption;
  sortByAsc: boolean;
}
