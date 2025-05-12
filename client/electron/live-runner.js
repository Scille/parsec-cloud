// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/* eslint-disable no-undef */
/* eslint-disable @typescript-eslint/no-var-requires */
const cp = require('child_process');
const chokidar = require('chokidar');
const { platform } = require('node:process');
const electron = process.env.ELECTRON_OVERRIDE_DIST_PATH ? `${process.env.ELECTRON_OVERRIDE_DIST_PATH}/electron` : require('electron');

let child = null;
const npmCmd = process.platform === 'win32' ? 'npm.cmd' : 'npm';
const reloadWatcher = {
  debouncer: null,
  ready: false,
  watcher: null,
  restarting: false,
};

function runBuild() {
  return new Promise((resolve) => {
    const tempChild = cp.spawn(
      npmCmd,
      ['run', 'build'],
      // Recent versions of node (since >= 20) now require to set `shell: true` when executing batch script.
      // eslint-disable-next-line max-len
      // https://nodejs.org/en/blog/vulnerability/april-2024-security-releases-2#command-injection-via-args-parameter-of-child_processspawn-without-shell-option-enabled-on-windows-cve-2024-27980---high
      platform === 'win32' ? { shell: true } : undefined,
    );
    tempChild.once('exit', () => {
      resolve();
    });
    tempChild.stdout.pipe(process.stdout);
  });
}

async function spawnElectron() {
  if (child !== null) {
    child.stdin.pause();
    child.kill();
    child = null;
    await runBuild();
  }
  child = cp.spawn(electron, ['--inspect=5858', './']);
  child.on('exit', () => {
    if (!reloadWatcher.restarting) {
      process.exit(0);
    }
  });
  child.stdout.pipe(process.stdout);
}

function setupReloadWatcher() {
  reloadWatcher.watcher = chokidar
    .watch(['./src', './build'], {
      // Ignore dotfiles
      ignored: /[/\\]\./,
      persistent: true,
    })
    .on('ready', () => {
      reloadWatcher.ready = true;
    })
    .on('all', () => {
      if (reloadWatcher.ready) {
        clearTimeout(reloadWatcher.debouncer);
        reloadWatcher.debouncer = setTimeout(async () => {
          console.log('Restarting');
          reloadWatcher.restarting = true;
          await spawnElectron();
          reloadWatcher.restarting = false;
          reloadWatcher.ready = false;
          clearTimeout(reloadWatcher.debouncer);
          reloadWatcher.debouncer = null;
          reloadWatcher.watcher = null;
          setupReloadWatcher();
        }, 500);
      }
    });
}

(async () => {
  await runBuild();
  await spawnElectron();
  setupReloadWatcher();
})();
