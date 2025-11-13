// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const builder = require('electron-builder');
const os = require('node:os');

const PARSEC_SCHEME = 'parsec3';

/**
 * @returns {{
 *   mode: 'test' | 'prod',
 *   platform: 'linux' | 'win32' | 'darwin',
 *   targets: string[],
 *   export: boolean,
 *   nightly: boolean,
 *   sign: boolean,
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
  program.addOption(new Option('--export', 'Export the configuration to JSON'));
  program.addOption(new Option('--nightly', 'The current build is a nightly build').default(false));
  program.addOption(new Option('--sign', 'Sign the package').default(false));
  program.argument('[target...]', 'Targets to build');

  program.parse();
  return {
    ...program.opts(),
    targets: program.args,
  };
}

/**
 * @param {string} platform
 * @param {string[]} targets
 * @return {Map<builder.Platform, Map<builder.Arch, string[]>>}
 */
function getBuildTargets(platform, targets) {
  switch (platform) {
    case 'linux':
      return builder.Platform.LINUX.createTarget(targets);
    case 'darwin':
      return builder.Platform.MAC.createTarget(targets);
    case 'win32':
      return builder.Platform.WINDOWS.createTarget(targets);
    case builder.DEFAULT_TARGET:
      return builder.Platform.current().createTarget(targets);
    default:
      throw new Error(`Unknown platform: ${platform}`);
  }
}
const OPTS = cli();
console.warn(OPTS);

const BUILD_TARGETS = getBuildTargets(OPTS.platform, OPTS.targets);
console.warn('BUILD_TARGETS', BUILD_TARGETS);

// The machine arch the electron-builder is running on.
process.env.BUILD_MACHINE_ARCH = os.machine();

/** @type {import('./assets/publishConfig').CustomPublishOptions} */
const publishConfig = {
  provider: 'custom',
  owner: 'Scille',
  repo: 'parsec-cloud',
  buildMachineArch: process.env.BUILD_MACHINE_ARCH,
  nightlyBuild: OPTS.nightly,
};

const fs = require('node:fs');

fs.mkdirSync('build/assets', { recursive: true });
fs.writeFileSync('build/assets/publishConfig.json', JSON.stringify(publishConfig));

/**
 * @type {Partial<import('app-builder-lib').WindowsConfiguration>}
 */
const WIN_SIGN_OPTIONS = {
  signtoolOptions: {
    certificateSubjectName: 'Scille',
    certificateSha1: '4505A81975EF724601813DF296AB74A07ECFA991',
    timeStampServer: 'http://time.certum.pl',
    rfc3161TimeStampServer: 'http://time.certum.pl',
    signingHashAlgorithms: ['sha256'],
  },
};

/**
 * @type {Partial<import('app-builder-lib').MacConfiguration>}
 */
const MACOS_SIGN_OPTIONS = {
  notarize: {
    teamId: process.env.APPLE_TEAM_ID,
  },
};

const UNSIGNED_ARTIFACT_NAME =
  OPTS.sign || OPTS.platform === 'linux'
    ? 'Parsec_${buildVersion}_${os}_${env.BUILD_MACHINE_ARCH}.${ext}'
    : 'Parsec_${buildVersion}_${os}_${env.BUILD_MACHINE_ARCH}.unsigned.${ext}';

/**
 * @type {import('electron-builder').Configuration}
 * @see https://www.electron.build/configuration/configuration
 */
const options = {
  /* eslint-disable max-len */
  /*
   * Doc mentions that it's used as a CFBundleIdentifier on MacOS, and as an Application User Model ID on Windows
   * (https://www.electron.build/configuration/configuration.html#configuration)
   * The doc for CFBundleIdentifier specifies that this should be in reverse-dns format
   *   https://developer.apple.com/library/archive/documentation/General/Reference/InfoPlistKeyReference/Articles/CoreFoundationKeys.html#//apple_ref/doc/uid/20001431-102070
   * The doc for Application User Model ID specifies that it should be in the `CompanyName.ProductName.SubProduct.VersionInformation` format
   *   https://learn.microsoft.com/en-us/windows/win32/shell/appids?redirectedfrom=MSDN
   * Since this is what determines the name of the app in the installer on Windows but it doesn't seem to have any visible effect
   * on MacOS, we decided to prioritize the Windows format.
   */
  /* eslint-enable max-len */
  appId: 'ParsecCloud.Parsec.Parsec.3',
  productName: 'Parsec',
  artifactName: UNSIGNED_ARTIFACT_NAME,
  buildVersion: '3.6.1-rc.1+dev',
  protocols: {
    name: 'Parsec-v3',
    schemes: [PARSEC_SCHEME],
  },

  compression: OPTS.mode === 'test' ? 'store' : 'normal',

  directories: {
    buildResources: 'assets',
  },

  files: ['assets/**/*', '!assets/installer.nsh', 'build/**/*', '!build/**/*.js.map', '!build/**/*.msi', 'app/**/*'],

  publish: publishConfig,

  // Asar is the electron archive format to bundle all the resources together.
  // Node files are shared library, hence keeping them unpacked avoid weird trick
  // when they must be loaded by the OS.
  // This is especially important on Snap given there the shared library rpath
  // gets patched to load it dependencies (e.g. the libssl bundled in the snap,
  // not the one on the host system).
  asarUnpack: ['**/*.node'],

  win: {
    target: 'nsis',
    ...(OPTS.sign ? WIN_SIGN_OPTIONS : {}),
    extraResources: [
      {
        from: 'node_modules/regedit/vbs',
        to: 'vbs',
        filter: ['**/*'],
      },
    ],
  },

  nsis: {
    allowElevation: true,
    oneClick: false,
    allowToChangeInstallationDirectory: true,
    include: 'assets/installer.nsh',
    guid: '2f56a772-db54-4a32-b264-28c42970f684',
  },

  afterSign: OPTS.sign === false || OPTS.platform !== 'darwin' ? undefined : 'scripts/after-sign.js',

  mac: {
    target: 'default',
    category: 'public.app-category.productivity',
    hardenedRuntime: true,
    entitlements: './macOS/entitlements.plist',
    entitlementsInherit: './macOS/entitlements.plist',
    ...(OPTS.sign ? MACOS_SIGN_OPTIONS : {}),
    // https://www.electron.build/mac#binaries
    binaries: ['build/src/libparsec.node'],
  },

  linux: {
    synopsis: 'Secure cloud framework',
    description: 'Parsec is an open-source cloud-based application that allows simple yet cryptographically secure file hosting.',
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

if (OPTS.export) {
  console.log(JSON.stringify(options, null, 2));
} else {
  builder.build({
    targets: BUILD_TARGETS,
    publish: 'never',
    config: options,
  });
}
