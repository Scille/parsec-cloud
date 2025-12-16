// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Result } from '@/parsec/types';

export function generateNoHandleError<T>(): Result<any, T> {
  return { ok: false, error: { tag: 'Internal', error: 'No handle' } as any };
}

export function stringifyError(error: { tag: string; error: string }): string {
  return `${error.tag} (${error.error})`;
}
