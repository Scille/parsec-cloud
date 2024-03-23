// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const builder = require('electron-builder');

const PARSEC_SCHEME = 'parsec3';

/**
 * @returns {{
 *   mode: 'test' | 'prod',
 *   platform: 'linux' | 'win32' | 'darwin',
 * }}
 */
function cli() {
  const { Command, Option } = require('commander');
  const program = new Command();

  program.name('electron-packager').description('Package the electron app');

  program.addOption(new Option('--mode <mode>', 'package mode').choices(['test', 'prod']).default('test').makeOptionMandatory(true));
  program.addOption(
    new Option('--platform <platform>', 'Build electron for <platform>')
      .choices(['linux', 'win32', 'darwin', builder.DEFAULT_TARGET])
      .default(builder.DEFAULT_TARGET)
      .makeOptionMandatory(true),
  );

  program.parse();
  return program.opts();
}

/**
 * @param {string} platform
 * @return {Map<builder.Platform, Map<builder.Arch, string[]>>}
 */
function getBuildTargets(platform) {
  switch (platform) {
    case 'linux':
      return builder.Platform.LINUX.createTarget();
    case 'darwin':
      return builder.Platform.MAC.createTarget();
    case 'win32':
      return builder.Platform.WINDOWS.createTarget();
    case builder.DEFAULT_TARGET:
      return builder.Platform.current().createTarget();
    default:
      throw new Error(`Unknown platform: ${platform}`);
  }
}
const OPTS = cli();
console.debug(OPTS);

const BUILD_TARGETS = getBuildTargets(OPTS.platform);
console.log('BUILD_TARGETS', BUILD_TARGETS);

/**
 * @type {import('electron-builder').Configuration}
 * @see https://www.electron.build/configuration/configuration
 */
const options = {
  appId: 'cloud.parsec.parsec-v3',
  protocols: {
    name: 'Parsec-v3',
    schemes: [PARSEC_SCHEME],
  },

  compression: OPTS.mode === 'test' ? 'store' : 'normal',

  directories: {
    buildResources: 'assets',
  },

  files: ['assets/**/*', '!assets/installer.nsh', 'build/**/*', '!build/**/*.msi', 'app/**/*'],

  publish: {
    provider: 'github',
  },

  // Asar is the electron archive format to bundle all the resources together.
  // Node files are shared library, hence keeping them unpacked avoid weird trick
  // when they must be loaded by the OS.
  // This is especially important on Snap given there the shared library rpath
  // gets patched to load it dependencies (e.g. the libssl bundled in the snap,
  // not the one on the host system).
  asarUnpack: ['**/*.node'],

  win: {
    target: 'nsis',
  },

  nsis: {
    allowElevation: true,
    oneClick: false,
    allowToChangeInstallationDirectory: true,
    include: 'assets/installer.nsh',
  },

  mac: {
    target: 'dmg',
    category: 'public.app-category.productivity',
  },

  linux: {
    synopsis: 'Secure cloud framework',
    description: 'Parsec is an open-source cloud-based application that allow simple yet cryptographically secure file hosting.',
    category: 'Office Network FileTransfer FileSystem Security',
    desktop: {
      MimeType: `x-scheme-handler/${PARSEC_SCHEME}`,
    },
    target: 'snap',
  },

  snap: {
    base: 'core22',
    grade: 'devel',
    allowNativeWayland: true,
    stagePackages: ['default', 'fuse3', 'libssl3'],
    confinement: 'classic',
  },

  beforePack: './scripts/before-pack.js',

  extends: null,
};
builder.build({
  targets: BUILD_TARGETS,
  publish: 'never',
  config: options,
});
