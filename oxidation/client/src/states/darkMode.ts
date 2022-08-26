// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

export function toggleDarkMode(theme: string): void {
  if (theme && ['light', 'dark'].includes(theme)) {
    document.body.classList.toggle('dark', theme === 'dark');
  } else {
    document.body.classList.toggle('dark', window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);
  }
}
