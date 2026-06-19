// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export function pxToRem(value: number): number {
  return value / 16;
}

export function toHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('');
}

export function escapeHTML(s: string): string {
  return s.replace(/[<>]/g, '').replace(/'/g, '\u2019').replace(/"/g, 'u201D');
}

export function sanitizeCustomTranslations(obj: object): object {
  return Object.fromEntries(
    Object.entries(obj).map(([k, v]) => {
      if (v !== null && typeof v === 'object') {
        return [k, sanitizeCustomTranslations(v)];
      } else if (typeof v === 'string') {
        return [k, escapeHTML(v)];
      }
      return [k, v];
    }),
  );
}
