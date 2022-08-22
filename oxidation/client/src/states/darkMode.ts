export function toggleDarkMode(theme: string): void {
  if (theme && ['light', 'dark'].includes(theme)) {
    document.body.classList.toggle('dark', theme === 'dark' ? true : false);
  } else {
    document.body.classList.toggle('dark', window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches ? true : false);
  }
}
