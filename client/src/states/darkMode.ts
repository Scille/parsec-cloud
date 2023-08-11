// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

export function toggleDarkMode(theme: string): void {
  const themes = [
    'light',
    'dark',
    'epilepsya',
    'rainbow',
    'disco',
    'electro',
  ];
  if (theme && themes.includes(theme)) {
    themes.forEach((element) => {
      if (element === theme) {
        document.body.classList.add(element);
      } else {
        document.body.classList.remove(element);
      }
    });
  } else {
    themes.forEach((element) => {
      document.body.classList.remove(element);
    });
    document.body.classList.toggle('dark', window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);
  }
}
