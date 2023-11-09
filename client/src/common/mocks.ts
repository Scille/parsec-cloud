// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// cSpell:disable

import { DateTime } from 'luxon';

export function getAppVersion(): string {
  return '3.0.0a';
}

export interface Change {
  description: string;
  issue?: string;
}

export interface VersionChange {
  version: string;
  date: DateTime;
  features: Change[];
  fixes: Change[];
  misc: Change[];
}

// Added multiple values for testing
// can be removed before release
export function getChanges(): VersionChange[] {
  return [
    {
      version: '3.0.0a',
      date: DateTime.fromISO('2023-09-01T17:00:00'),
      features: [
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added user greet modal',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
      ],
      fixes: [
        {
          description: 'Made links to accept TOS when creating a new organization clickable',
          issue: '4954',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
      ],
      misc: [
        {
          description: 'Updates the design of the modal to greet a new user',
          issue: '4985',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
        {
          description: 'Added changelog modal',
          issue: '4978',
        },
      ],
    },
  ];
}
