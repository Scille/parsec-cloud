<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page v-if="initialized">
    <ion-router-outlet />
  </ion-page>
</template>

<script setup lang="ts">
import * as parsec from '@/parsec';
import { getClientInfo } from '@/parsec';
import { getClientConfig } from '@/parsec/internals';
import { libparsec, ParsecInvitationAddr } from '@/plugins/libparsec';
import { getConnectionHandle, navigateTo, Routes } from '@/router';
import { EventDistributor } from '@/services/eventDistributor';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { IonPage, IonRouterOutlet } from '@ionic/vue';
import { inject, onMounted, ref } from 'vue';

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const initialized = ref(false);

const DEFAULT_HANDLE = 1;
const DEFAULT_HANDLE_WITH_PARSEC_ACCOUNT = 2;

function getDevDefaultHandle(): parsec.ConnectionHandle {
  if (parsec.ParsecAccount.isLoggedIn()) {
    return DEFAULT_HANDLE_WITH_PARSEC_ACCOUNT;
  }
  return DEFAULT_HANDLE;
}

onMounted(async () => {
  initialized.value = false;
  const handle = getConnectionHandle();

  if (!handle) {
    window.electronAPI.log('error', 'Failed to retrieve connection handle while logged in');
    await navigateTo(Routes.Home, { replace: true, skipHandle: true });
    return;
  }

  if (
    !import.meta.env.PARSEC_APP_TESTBED_SERVER ||
    window.isTesting() ||
    (await getClientInfo(handle)).ok === true ||
    import.meta.env.PARSEC_APP_TESTBED_AUTO_LOGIN !== 'true'
  ) {
    initialized.value = true;
    return;
  }

  // When in dev mode, we often open directly a connected page,
  // so a few states are not properly set.
  if (parsec.ParsecAccount.isLoggedIn()) {
    window.electronAPI.log('info', `Parsec Account logged in, default handle should be "${DEFAULT_HANDLE_WITH_PARSEC_ACCOUNT}"`);
  }
  if (handle !== getDevDefaultHandle()) {
    window.electronAPI.log('error', `In dev mode, you should use "${getDevDefaultHandle()}" as the default handle`);
    // eslint-disable-next-line no-alert
    alert(
      `Use "${DEFAULT_HANDLE}" or "${DEFAULT_HANDLE_WITH_PARSEC_ACCOUNT}" (if using ParsecAccount)
as the default handle when not connecting properly`,
    );
    await navigateTo(Routes.Home);
    return;
  }
  window.electronAPI.log('info', 'Page was refreshed, login in a default device');
  injectionProvider.createNewInjections(getDevDefaultHandle(), new EventDistributor());
  // Not filtering the devices, because we need alice first device, not the second one
  const listResult = await parsec.listAvailableDevicesWithError(false);
  let devices: Array<parsec.AvailableDevice> = [];
  if (!listResult.ok) {
    window.electronAPI.log('error', `Error when listing the devices: ${listResult.error.tag} (${listResult.error.error})`);
  } else {
    devices = listResult.value;
  }

  let device = devices.find((d) => d.humanHandle.label === 'Alicey McAliceFace' && d.deviceLabel.includes('dev1'));

  if (!device) {
    device = devices[0];
    window.electronAPI.log('error', `Could not find Alice device, using ${device.humanHandle.label}`);
  }

  const result = await parsec.login(device, parsec.AccessStrategy.usePassword(device, 'P@ssw0rd.'));
  if (!result.ok) {
    window.electronAPI.log('error', `Failed to log in on a default device: ${JSON.stringify(result.error)}`);
  } else if (result.value !== getDevDefaultHandle()) {
    window.electronAPI.log('error', `Lib returned handle ${result.value} instead of ${getDevDefaultHandle()}`);
  } else {
    window.electronAPI.log('info', `Logged in as ${device.humanHandle.label}`);
  }

  if (import.meta.env.PARSEC_APP_CLEAR_CACHE === 'true') {
    await storageManager.clearAll();
  }
  await populate(handle);
  initialized.value = true;
});

async function populate(handle: parsec.ConnectionHandle): Promise<void> {
  if (import.meta.env.PARSEC_APP_POPULATE_DEFAULT_WORKSPACE === 'true') {
    await populateFiles();
    await addReadOnlyWorkspace();
  }
  if (import.meta.env.PARSEC_APP_POPULATE_USERS === 'true') {
    await populateUsers(handle);
  }
}

async function populateUsers(handle: parsec.ConnectionHandle): Promise<void> {
  const USERS = [
    {
      // cspell:disable-next-line
      label: 'Gordon Freeman',
      // cspell:disable-next-line
      email: 'gordon.freeman@blackmesa.nm',
      profile: parsec.UserProfile.Standard,
      revoked: false,
    },
    {
      // cspell:disable-next-line
      label: 'Arthas Menethil',
      // cspell:disable-next-line
      email: 'arthas.menethil@lordaeron.az',
      profile: parsec.UserProfile.Admin,
      revoked: true,
    },
    {
      // cspell:disable-next-line
      label: 'Karlach',
      // cspell:disable-next-line
      email: 'karlach@avernus.hell',
      profile: parsec.UserProfile.Outsider,
      revoked: false,
    },
    {
      // cspell:disable-next-line
      label: 'Artorias',
      // cspell:disable-next-line
      email: 'artorias@abyss.dark',
      profile: parsec.UserProfile.Admin,
      revoked: false,
    },
  ];

  for (const user of USERS) {
    await addUser(handle, user.label, user.email, user.profile, user.revoked);
  }
}

async function addUser(
  connHandle: parsec.ConnectionHandle,
  label: string,
  email: string,
  profile: parsec.UserProfile,
  revoke: boolean,
): Promise<void> {
  const invResult = await parsec.inviteUser(email);
  if (!invResult.ok) {
    console.log(invResult.error);
    return;
  }

  window.electronAPI.log('debug', `Adding user ${label} <${email}>`);

  greetUser(connHandle, invResult.value.token, profile)
    .then(async () => {
      if (revoke) {
        window.electronAPI.log('debug', `Revoking ${label} <${email}>`);
        const result = await parsec.listUsers(true, email);
        if (result.ok && result.value.length > 0) {
          await parsec.revokeUser(result.value[0].id);
        }
      }
    })
    .catch((err: string) => {
      window.electronAPI.log('error', `Greet failed: ${err}`);
    });
  const [invitationAddr, _] = invResult.value.addr;
  claimUser(email, label, invitationAddr).catch((err: string) => {
    window.electronAPI.log('error', `Claim failed: ${err}`);
  });
}

async function claimUser(email: string, label: string, invitationAddr: ParsecInvitationAddr): Promise<void> {
  const retResult = await libparsec.claimerRetrieveInfo(getClientConfig(), invitationAddr);
  if (!retResult.ok) {
    throw new Error(`claimerRetrieveInfo failed: ${retResult.error.error}`);
  }
  let handle = retResult.value.handle;
  let canceller = await libparsec.newCanceller();
  const waitResult = await libparsec.claimerUserWaitAllPeers(canceller, handle);
  if (!waitResult.ok) {
    throw new Error(`claimerUserWaitAllPeers failed: ${waitResult.error.error}`);
  }
  handle = waitResult.value.handle;
  canceller = await libparsec.newCanceller();
  const signifyTrustResult = await libparsec.claimerUserInProgress1DoSignifyTrust(canceller, handle);
  if (!signifyTrustResult.ok) {
    throw new Error(`claimerUserInProgress1DoSignifyTrust failed: ${signifyTrustResult.error.error}`);
  }
  handle = signifyTrustResult.value.handle;
  canceller = await libparsec.newCanceller();
  const waitTrustResult = await libparsec.claimerUserInProgress2DoWaitPeerTrust(canceller, handle);
  if (!waitTrustResult.ok) {
    throw new Error(`claimerUserInProgress2DoWaitPeerTrust failed: ${waitTrustResult.error.error}`);
  }
  handle = waitTrustResult.value.handle;
  canceller = await libparsec.newCanceller();
  const claimResult = await libparsec.claimerUserInProgress3DoClaim(canceller, handle!, 'DeviceLabel', { email: email, label: label });
  if (!claimResult.ok) {
    throw new Error(`claimerUserInProgress3DoClaim failed: ${claimResult.error.error}`);
  }
  handle = claimResult.value.handle;
  const finalizeResult = await libparsec.claimerUserFinalizeSaveLocalDevice(handle, {
    tag: parsec.DeviceSaveStrategyTag.Password,
    password: 'P@ssw0rd.',
  });
  if (!finalizeResult.ok) {
    throw new Error(`claimerUserFinalizeSaveLocalDevice failed: ${finalizeResult.error.error}`);
  }
}

async function greetUser(connHandle: parsec.ConnectionHandle, token: string, profile: parsec.UserProfile): Promise<void> {
  const greetResult = await libparsec.clientStartUserInvitationGreet(connHandle, token);
  if (!greetResult.ok) {
    throw new Error(`clientStartUserInvitationGreet failed: ${greetResult.error.error}`);
  }
  let handle = greetResult.value.handle;
  let canceller = await libparsec.newCanceller();
  const waitResult = await libparsec.greeterUserInitialDoWaitPeer(canceller, handle);
  if (!waitResult.ok) {
    throw new Error(`greeterUserInitialDoWaitPeer failed: ${waitResult.error.error}`);
  }
  handle = waitResult.value.handle;
  canceller = await libparsec.newCanceller();
  const waitTrustResult = await libparsec.greeterUserInProgress1DoWaitPeerTrust(canceller, handle);
  if (!waitTrustResult.ok) {
    throw new Error(`greeterUserInProgress1DoWaitPeerTrust failed: ${waitTrustResult.error.error}`);
  }
  handle = waitTrustResult.value.handle;
  canceller = await libparsec.newCanceller();
  const signifyTrustResult = await libparsec.greeterUserInProgress2DoSignifyTrust(canceller, handle);
  if (!signifyTrustResult.ok) {
    throw new Error(`greeterUserInProgress2DoSignifyTrust failed: ${signifyTrustResult.error.error}`);
  }
  handle = signifyTrustResult.value.handle;
  canceller = await libparsec.newCanceller();
  const claimRequestsResult = await libparsec.greeterUserInProgress3DoGetClaimRequests(canceller, handle);
  if (!claimRequestsResult.ok) {
    throw new Error(`greeterUserInProgress3DoGetClaimRequests failed: ${claimRequestsResult.error.error}`);
  }
  handle = claimRequestsResult.value.handle;
  canceller = await libparsec.newCanceller();
  const createResult = await libparsec.greeterUserInProgress4DoCreate(
    canceller,
    handle,
    claimRequestsResult.value.requestedHumanHandle,
    claimRequestsResult.value.requestedDeviceLabel,
    profile,
  );
  if (!createResult.ok) {
    throw new Error(`greeterUserInProgress4DoCreate failed: ${createResult.error.error}`);
  }
}

async function populateFiles(): Promise<void> {
  // Avoid importing files if unnecessary
  const mockFiles = await import('@/parsec/mock_files');

  window.electronAPI.log('debug', 'Creating mock files');
  const workspaces = await parsec.listWorkspaces(getConnectionHandle());
  if (!workspaces.ok) {
    window.electronAPI.log('error', 'Failed to list workspaces');
    return;
  }
  for (const workspace of workspaces.value) {
    await parsec.createFolder(workspace.handle, '/Folder_éèñÑ');

    for (const fileType in mockFiles.MockFileType) {
      const fileName = `document_${fileType}.${fileType.toLocaleLowerCase()}`;
      const openResult = await parsec.openFile(workspace.handle, `/${fileName}`, {
        write: true,
        truncate: true,
        create: true,
        createNew: true,
      });

      if (!openResult.ok) {
        window.electronAPI.log('error', `Could not open file ${fileName}`);
        continue;
      }
      try {
        const content = await mockFiles.getMockFileContent(fileType as any);
        const writeResult = await parsec.writeFile(workspace.handle, openResult.value, 0, content);
        if (!writeResult.ok) {
          window.electronAPI.log('error', `Failed to write file ${fileName}`);
          continue;
        }
      } finally {
        await parsec.closeFile(workspace.handle, openResult.value);
      }
    }
  }
}

async function addReadOnlyWorkspace(): Promise<void> {
  const devices = await parsec.listAvailableDevices(false);
  const bobDevice = devices.find((d) => d.humanHandle.label === 'Boby McBobFace');
  const aliceDevice = devices.find((d) => d.humanHandle.label === 'Alicey McAliceFace');

  window.electronAPI.log('debug', 'Creating read-only workspace');

  if (!bobDevice || !aliceDevice) {
    window.electronAPI.log('error', 'Could not find Alice or Bob device');
    return;
  }
  const loginResult = await libparsec.clientStart(getClientConfig(), {
    tag: parsec.DeviceAccessStrategyTag.Password,
    password: 'P@ssw0rd.',
    keyFile: bobDevice.keyFilePath,
  });
  if (!loginResult.ok) {
    window.electronAPI.log('error', `Failed to login as Bob: ${loginResult.error.error}`);
    return;
  }
  try {
    const wkResult = await libparsec.clientCreateWorkspace(loginResult.value, 'wksp2');
    if (!wkResult.ok) {
      window.electronAPI.log('error', `Failed to create a workspace as Bob: ${wkResult.error.error}`);
      return;
    }

    /* Does not work currently because of some problems with the testbed */
    // const mockFiles = await import('@/parsec/mock_files');

    // window.electronAPI.log('debug', 'Creating mock files in read-only workspace');

    // const startWkResult = await libparsec.clientStartWorkspace(loginResult.value, wkResult.value);
    // if (!startWkResult.ok) {
    //   window.electronAPI.log('error', `Failed to start workspace: ${startWkResult.error.error}`);
    //   return;
    // }
    // for (const fileType in mockFiles.MockFileType) {
    //   const fileName = `document_${fileType}.${fileType.toLocaleLowerCase()}`;
    //   const openResult = await libparsec.workspaceOpenFile(startWkResult.value, `/${fileName}`, {
    //     write: true,
    //     truncate: true,
    //     create: true,
    //     createNew: true,
    //     read: false,
    //   });
    //   if (!openResult.ok) {
    //     window.electronAPI.log('error', `Could not open file ${fileName}`);
    //     continue;
    //   }
    //   try {
    //     const content = await mockFiles.getMockFileContent(fileType as any);
    //     const writeResult = await libparsec.workspaceFdWrite(startWkResult.value, openResult.value, BigInt(0), content);
    //     if (!writeResult.ok) {
    //       window.electronAPI.log('error', `Failed to write file ${fileName}`);
    //       continue;
    //     }
    //   } finally {
    //     await libparsec.workspaceFdClose(startWkResult.value, openResult.value);
    //   }
    // }

    const shareResult = await libparsec.clientShareWorkspace(
      loginResult.value,
      wkResult.value,
      aliceDevice.userId,
      parsec.WorkspaceRole.Reader,
    );
    if (!shareResult.ok) {
      window.electronAPI.log('error', `Failed to share Bob's workspace with Alice: ${shareResult.error.error}`);
      return;
    }
    window.electronAPI.log('debug', 'Read-only workspace created');
  } catch (e: any) {
    window.electronAPI.log('error', `Error while creating read-only workspace: ${e}`);
  } finally {
    await libparsec.clientStop(loginResult.value);
  }
}
</script>
