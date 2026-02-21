// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { availableDeviceMatchesServer } from '@/common/device';
import {
  AvailableDevice,
  getOrganizationHandle,
  listAvailableDevices,
  ParsedParsecAddrTag,
  parseFileLink,
  parseParsecAddr,
} from '@/parsec';
import {
  backupCurrentOrganization,
  currentRouteIs,
  getConnectionHandle,
  navigateTo,
  RouteBackup,
  Routes,
  switchOrganization,
} from '@/router';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { modalController, popoverController } from '@ionic/vue';
import { Base64 } from 'megashark-lib';

async function handleJoinLink(link: string): Promise<void> {
  if (!currentRouteIs(Routes.Home) && getConnectionHandle() !== null) {
    await switchOrganization(null, true);
  }
  await navigateTo(Routes.Home, { query: { claimLink: link } });
}

async function handleBootstrapLink(link: string): Promise<void> {
  if (!currentRouteIs(Routes.Home) && getConnectionHandle() !== null) {
    await switchOrganization(null, true);
  }
  await navigateTo(Routes.Home, { query: { bootstrapLink: link } });
}

async function handleAsyncEnrollmentLink(link: string): Promise<void> {
  if (!currentRouteIs(Routes.Home) && getConnectionHandle() !== null) {
    await switchOrganization(null, true);
  }
  await navigateTo(Routes.Home, { query: { asyncEnrollmentLink: link } });
}

async function handleTotpReset(link: string): Promise<void> {
  if (!currentRouteIs(Routes.Home) && getConnectionHandle() !== null) {
    await switchOrganization(null, true);
  }
  await navigateTo(Routes.Home, { query: { totpResetLink: link } });
}

async function handleFileLink(link: string, informationManager: InformationManager): Promise<void> {
  const result = await parseFileLink(link);
  if (!result.ok) {
    informationManager.present(
      new Information({
        message: 'link.invalidFileLink',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }
  const linkData = result.value;
  // Check if the org we want is already logged in
  const handle = await getOrganizationHandle({ id: linkData.organizationId, server: { hostname: linkData.hostname, port: linkData.port } });

  // We have a matching organization already opened
  if (handle) {
    if (getConnectionHandle() && handle !== getConnectionHandle()) {
      await switchOrganization(handle, true);
    }

    const routeData: RouteBackup = {
      handle: handle,
      data: {
        route: Routes.Workspaces,
        params: { handle: handle },
        query: { fileLink: link },
      },
    };
    await navigateTo(Routes.Loading, { skipHandle: true, replace: true, query: { loginInfo: Base64.fromObject(routeData) } });
  } else {
    // Check if we have a device with the org
    const devices = await listAvailableDevices();
    let matchingDevice: AvailableDevice | undefined;
    for (const device of devices) {
      // Pre-matching on org id only, in case we don't find a server match. This could happen
      // with different DNS, IP addresses instead of hostname, ...
      if (device.organizationId === linkData.organizationId && !matchingDevice) {
        matchingDevice = device;
      }
      if (
        (await availableDeviceMatchesServer(device, { hostname: linkData.hostname, port: linkData.port })) &&
        device.organizationId === linkData.organizationId
      ) {
        matchingDevice = device;
        // Full match takes priority, we can break out the loop
        break;
      }
    }
    if (!matchingDevice) {
      await informationManager.present(
        new Information({
          message: { key: 'link.orgNotFound', data: { organization: linkData.organizationId } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      return;
    }
    if (!currentRouteIs(Routes.Home)) {
      await backupCurrentOrganization();
    }
    await navigateTo(Routes.Home, { replace: true, skipHandle: true, query: { deviceId: matchingDevice.deviceId, fileLink: link } });
  }
}

export async function handleParsecLink(link: string, informationManager: InformationManager): Promise<void> {
  if (await modalController.getTop()) {
    informationManager.present(
      new Information({
        message: 'link.appIsBusy',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }
  if (await popoverController.getTop()) {
    await popoverController.dismiss();
  }
  const result = await parseParsecAddr(link);

  if (!result.ok) {
    await informationManager.present(
      new Information({
        message: 'link.invalid',
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    return;
  }
  if (result.value.tag === ParsedParsecAddrTag.InvitationUser || result.value.tag === ParsedParsecAddrTag.InvitationDevice) {
    await handleJoinLink(link);
  } else if (result.value.tag === ParsedParsecAddrTag.WorkspacePath) {
    await handleFileLink(link, informationManager);
  } else if (result.value.tag === ParsedParsecAddrTag.OrganizationBootstrap) {
    await handleBootstrapLink(link);
  } else if (result.value.tag === ParsedParsecAddrTag.AsyncEnrollment) {
    await handleAsyncEnrollmentLink(link);
  } else if (result.value.tag === ParsedParsecAddrTag.TOTPReset) {
    await handleTotpReset(link);
  } else {
    window.electronAPI.log('warn', `Unhandled link type '${result.value.tag}'`);
  }
}
