// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import {
  AnyClaimRetrievedInfoTag,
  AvailableDevice,
  buildParsecAddr,
  constructAccessStrategy,
  DeviceAccessStrategy,
  ExchangeHandle,
  ParsedParsecAddrTag,
  parseParsecAddr,
  retrieveInvitationInfo,
  shamirRecoverDevice,
  ShamirRecoveryClaimMaybeFinalizeInfoTag,
  shamirSaveDevice,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { openChooseAuthenticationModal } from '@/views/authentication/utils';
import ClaimShamirRecoveryModal from '@/views/shamir/ClaimShamirRecoveryModal.vue';
import ShamirInstructionsModal from '@/views/shamir/ShamirInstructionsModal.vue';
import ShamirLinkModal from '@/views/shamir/ShamirLinkModal.vue';
import { modalController } from '@ionic/vue';
import { Answer, askQuestion, MsModalResult, openSpinnerModal } from 'megashark-lib';

export async function handleShamirRecovery(
  informationManager: InformationManager,
  link?: string,
): Promise<{ device: AvailableDevice; access: DeviceAccessStrategy } | undefined> {
  const instructionsModal = await modalController.create({
    component: ShamirInstructionsModal,
    canDismiss: true,
    cssClass: 'shamir-instructions-modal',
    backdropDismiss: false,
    showBackdrop: true,
  });
  await instructionsModal.present();
  const instructionsResult = await instructionsModal.onWillDismiss();
  await instructionsModal.dismiss();
  if (instructionsResult.role === MsModalResult.Cancel) {
    return;
  }
  if (!link) {
    const linkModal = await modalController.create({
      component: ShamirLinkModal,
      canDismiss: true,
      cssClass: 'shamir-link-modal',
      backdropDismiss: false,
      showBackdrop: true,
    });
    await linkModal.present();
    const linkModalResult = await linkModal.onWillDismiss();
    await linkModal.dismiss();
    if (linkModalResult.role === MsModalResult.Cancel) {
      return;
    }
    link = linkModalResult.data.link;
  }

  if (!link) {
    return;
  }

  const addrResult = await parseParsecAddr(link);

  if (!addrResult.ok || addrResult.value.tag !== ParsedParsecAddrTag.InvitationShamirRecovery) {
    informationManager.present(
      new Information({
        message: 'HomePage.shamir.invalidLink',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }

  const serverAddr = await buildParsecAddr(addrResult.value);

  const spinnerModal = await openSpinnerModal();

  const infoResult = await retrieveInvitationInfo(link);

  if (!infoResult.ok || infoResult.value.tag !== AnyClaimRetrievedInfoTag.ShamirRecovery) {
    informationManager.present(
      new Information({
        message: 'HomePage.shamir.invalidLink',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    await spinnerModal.dismiss();
    return;
  }

  await spinnerModal.dismiss();

  const modal = await modalController.create({
    component: ClaimShamirRecoveryModal,
    canDismiss: true,
    cssClass: 'claim-shamir-recovery-modal',
    backdropDismiss: false,
    showBackdrop: true,
    expandToScroll: false,
    handle: false,
    componentProps: {
      invitationInfo: infoResult.value,
      informationManager: informationManager,
    },
  });
  await modal.present();
  const { data, role } = await modal.onWillDismiss();
  await modal.dismiss();
  if (role !== MsModalResult.Confirm || !data.info) {
    return;
  }
  let handle: number = data.info.handle;

  const saveStrategy = await openChooseAuthenticationModal(serverAddr);
  if (!saveStrategy) {
    informationManager.present(
      new Information({
        message: 'HomePage.shamir.noAuthentication',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }
  let finalHandle: ExchangeHandle | undefined = undefined;
  while (true) {
    const spinnerModal = await openSpinnerModal();
    try {
      const recoverDeviceResult = await shamirRecoverDevice(handle);

      if (!recoverDeviceResult.ok) {
        informationManager.present(
          new Information({
            message: 'HomePage.shamir.recoverDeviceFailed',
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
        return;
      }
      if (recoverDeviceResult.value.tag === ShamirRecoveryClaimMaybeFinalizeInfoTag.Offline) {
        const answer = await askQuestion('HomePage.shamir.offlineTitle', 'HomePage.shamir.offlineMessage', {
          yesText: 'HomePage.shamir.retry',
          noText: 'HomePage.shamir.giveUp',
        });
        if (answer === Answer.No) {
          break;
        } else {
          handle = recoverDeviceResult.value.handle;
        }
      } else {
        finalHandle = recoverDeviceResult.value.handle;
        break;
      }
    } finally {
      await spinnerModal.dismiss();
    }
  }
  if (!finalHandle) {
    return;
  }
  const result = await shamirSaveDevice(finalHandle, saveStrategy);
  if (!result.ok) {
    informationManager.present(
      new Information({
        message: 'HomePage.shamir.saveDeviceFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    await informationManager.present(
      new Information({
        message: 'HomePage.shamir.recovered',
        level: InformationLevel.Success,
      }),
      PresentationMode.Modal,
    );
    return { device: result.value, access: constructAccessStrategy(result.value, saveStrategy.primaryProtection) };
  }
}
