// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const { notarize } = require('@electron/notarize');

/**
 * Inspired by the blog post: https://kilianvalkhof.com/2019/electron/notarizing-your-electron-application/
 * @param {import('app-builder-lib/out/configuration').AfterPackContext} context
 */
async function notarizing(context) {
  const { electronPlatformName, appOutDir, packager } = context;
  if (electronPlatformName !== 'darwin') {
    return;
  }

  const appName = packager.appInfo.productFilename;

  return await notarize({
    tool: 'notarytool',
    teamId: process.env.APPLE_TEAM_ID,
    appleId: process.env.APPLE_ID,
    appleIdPassword: process.env.APPLE_ID_PASSWORD,
    appPath: `${appOutDir}/${appName}.app`,
  });
}

exports.default = notarizing;
