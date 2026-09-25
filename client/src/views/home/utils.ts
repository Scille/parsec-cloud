// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { buildParsecAddr, getServerConfig, isSmartcardAvailable, ParsedParsecAddrTag, parseParsecAddr } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import ActivateTotpModal from '@/views/totp/ActivateTotpModal.vue';
import AsyncEnrollmentModal from '@/views/users/AsyncEnrollmentModal.vue';
import { modalController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';

export async function handleTotpReset(link: string, informationManager: InformationManager): Promise<void> {
  const addrResult = await parseParsecAddr(link);

  if (!addrResult.ok || addrResult.value.tag !== ParsedParsecAddrTag.TOTPReset) {
    informationManager.present(
      new Information({
        message: 'HomePage.organizationRequest.totp.invalidLink',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }
  const modal = await modalController.create({
    component: ActivateTotpModal,
    cssClass: 'activate-totp-modal',
    componentProps: {
      params: {
        mode: 'reset',
        link: link,
      },
    },
    canDismiss: true,
    backdropDismiss: true,
    showBackdrop: true,
  });
  await modal.present();
  const { role } = await modal.onDidDismiss();
  await modal.dismiss();

  if (role !== MsModalResult.Confirm) {
    return;
  }
  informationManager.present(
    new Information({
      message: 'Authentication.mfa.mfaSuccess.description',
      level: InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
}

export async function handleAsyncEnrollment(link: string, informationManager: InformationManager): Promise<void> {
  const addrResult = await parseParsecAddr(link);

  if (!addrResult.ok || addrResult.value.tag !== ParsedParsecAddrTag.AsyncEnrollment) {
    informationManager.present(
      new Information({
        message: 'HomePage.organizationRequest.asyncEnrollmentModal.errors.invalidLink',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }

  const addr = await buildParsecAddr(addrResult.value);
  const pkiAvailable = await isSmartcardAvailable();
  const serverConfigResult = await getServerConfig(addr);

  // We don't have PKI and openbao is not configured on the server, can't do anything
  if (
    !pkiAvailable &&
    (!serverConfigResult.ok || !serverConfigResult.value.openbao || serverConfigResult.value.openbao.auths.length === 0)
  ) {
    informationManager.present(
      new Information({
        message: 'HomePage.organizationRequest.asyncEnrollmentModal.errors.pkiSsoNotAvailable',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }

  const modal = await modalController.create({
    component: AsyncEnrollmentModal,
    showBackdrop: true,
    backdropDismiss: false,
    componentProps: {
      link: link,
      addr: addrResult.value,
      serverConfig: serverConfigResult.ok ? serverConfigResult.value : undefined,
      pkiAvailable: pkiAvailable,
    },
    cssClass: 'async-enrollment-modal',
  });
  await modal.present();
  const { role } = await modal.onDidDismiss();
  await modal.dismiss();

  if (role !== MsModalResult.Confirm) {
    return;
  }

  informationManager.present(
    new Information({
      message: 'HomePage.organizationRequest.requestSent.success',
      level: InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
}
