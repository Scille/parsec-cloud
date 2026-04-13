// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { OwnDeviceInfo, createDeviceInvitation, listOwnDevices } from '@/parsec';
import { EventDistributor, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import GreetDeviceModal from '@/views/devices/GreetDeviceModal.vue';
import { modalController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';
import { Ref } from 'vue';

function getSortedNonRecoveryDevices(devices: OwnDeviceInfo[]): OwnDeviceInfo[] {
  return devices.filter((d) => !d.isRecovery && !d.isShamir && !d.isRegistration).sort((d) => (d.isCurrent ? -1 : 1));
}

function getLastCreatedDevice(devices: OwnDeviceInfo[]): OwnDeviceInfo | undefined {
  return devices.toSorted((d1, d2) => (d1.createdOn > d2.createdOn ? -1 : 1))[0];
}

export async function refreshOwnDevicesList(informationManager: InformationManager, devices: Ref<OwnDeviceInfo[]>): Promise<void> {
  const result = await listOwnDevices();
  if (result.ok) {
    devices.value = getSortedNonRecoveryDevices(result.value);
  } else {
    informationManager.present(
      new Information({
        message: 'DevicesPage.greet.errors.retrieveDeviceInfoFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    window.electronAPI.log('error', `Failed to list devices ${JSON.stringify(result.error)}`);
  }
}

export async function addDeviceWithGreetModal(
  informationManager: InformationManager,
  eventDistributor: EventDistributor,
  devices: Ref<OwnDeviceInfo[]>,
): Promise<void> {
  const result = await createDeviceInvitation(false);
  if (!result.ok) {
    informationManager.present(
      new Information({
        message: 'DevicesPage.greet.errors.startFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }

  const [_, invitationAddr] = result.value.addr;
  const modal = await modalController.create({
    component: GreetDeviceModal,
    canDismiss: true,
    backdropDismiss: false,
    cssClass: 'greet-organization-modal',
    componentProps: {
      informationManager,
      invitationLink: invitationAddr,
      token: result.value.token,
    },
  });
  await modal.present();
  const modalResult = await modal.onWillDismiss();
  await modal.dismiss();
  if (modalResult.role === MsModalResult.Confirm) {
    await refreshOwnDevicesList(informationManager, devices);
    const lastDevice = getLastCreatedDevice(devices.value);
    if (lastDevice) {
      eventDistributor.dispatchEvent(Events.DeviceCreated, { info: lastDevice });
    }
  }
}
