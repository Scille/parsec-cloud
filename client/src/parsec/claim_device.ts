// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  libparsec,
} from '@/plugins/libparsec';

import {
  DeviceLabel,
  DeviceClaimFinalizeInfo,
  DeviceClaimInProgress1Info,
  DeviceClaimInProgress2Info,
  DeviceClaimInProgress3Info,
  AvailableDevice,
  ClientEvent,
  Handle,
  Result,
  ClaimInProgressError,
  UserOrDeviceClaimInitialInfoDevice,
  SASCode,
  HumanHandle,
  ClaimerRetrieveInfoError,
  DeviceFileType,
} from '@/parsec/types';

import { DEFAULT_HANDLE, MOCK_WAITING_TIME, getClientConfig, wait } from '@/parsec/internals';

export class DeviceClaim {
  handle: Handle | null;
  canceller: Handle | null;
  SASCodeChoices: SASCode[];
  correctSASCode: SASCode;
  guestSASCode: SASCode;
  device: AvailableDevice | null;
  greeter: HumanHandle | null;

  constructor() {
    this.handle = null;
    this.canceller = null;
    this.guestSASCode = '';
    this.correctSASCode = '';
    this.SASCodeChoices = [];
    this.device = null;
    this.greeter = null;
  }

  async abort(): Promise<void> {
    if (this.canceller !== null && window.isDesktop()) {
      libparsec.cancel(this.canceller);
    }
    if (this.handle !== null && window.isDesktop()) {
      libparsec.claimerGreeterAbortOperation(this.handle);
    }
    this.canceller = null;
    this.handle = null;
    this.guestSASCode = '';
    this.correctSASCode = '';
    this.SASCodeChoices = [];
    this.device = null;
    this.greeter = null;
  }

  _assertState(nullCanceller: boolean, nullHandle: boolean): void {
    if (nullCanceller && this.canceller !== null) {
      throw Error('Canceller should be null');
    } else if (!nullCanceller && this.canceller === null) {
      throw Error('Canceller should not be null');
    }
    if (nullHandle && this.handle !== null) {
      throw Error('Handle should be null');
    } else if (!nullHandle && this.handle === null) {
      throw Error('Handle should not be null');
    }
  }

  async retrieveInfo(invitationLink: string):
  Promise<Result<UserOrDeviceClaimInitialInfoDevice, ClaimerRetrieveInfoError>> {
    function eventCallback(event: ClientEvent): void {
      console.log('On event', event);
    }

    this._assertState(true, true);

    if (window.isDesktop()) {
      const clientConfig = getClientConfig();
      const result = await libparsec.claimerRetrieveInfo(clientConfig, eventCallback, invitationLink);
      if (result.ok) {
        this.handle = result.value.handle;
        this.greeter = result.value.greeterHumanHandle;
      }
      return result as Result<UserOrDeviceClaimInitialInfoDevice, ClaimerRetrieveInfoError>;
    } else {
      await wait(MOCK_WAITING_TIME);
      this.handle = DEFAULT_HANDLE;
      this.greeter = {
        email: 'gale@waterdeep.faerun',
        // cspell:disable-next-line
        label: 'Gale Dekarios',
      };
      return new Promise<Result<UserOrDeviceClaimInitialInfoDevice, ClaimerRetrieveInfoError>>((resolve, _reject) => {
        resolve({ok: true, value: {
          tag: 'Device',
          handle: DEFAULT_HANDLE,
          greeterUserId: '1234',
          greeterHumanHandle: {
            email: 'gale@waterdeep.faerun',
            // cspell:disable-next-line
            label: 'Gale Dekarios',
          },
        }});
      });
    }
  }

  async initialWaitHost():
  Promise<Result<DeviceClaimInProgress1Info, ClaimInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerDeviceInitialDoWaitPeer(this.canceller, this.handle!);
      if (result.ok) {
        this.SASCodeChoices = result.value.greeterSasChoices;
        this.correctSASCode = result.value.greeterSas;
        this.handle = result.value.handle;
      }
      this.canceller = null;
      return result;
    } else {
      this.SASCodeChoices = ['5MNO', '6PQR', '7STU', '8VWX'];
      this.correctSASCode = '7STU';
      return new Promise<Result<DeviceClaimInProgress1Info, ClaimInProgressError>>((resolve, _reject) => {
        resolve({ok: true, value: {
          handle: DEFAULT_HANDLE,
          greeterSas: this.correctSASCode,
          greeterSasChoices: this.SASCodeChoices,
        }});
      });
    }
  }

  async signifyTrust():
  Promise<Result<DeviceClaimInProgress2Info, ClaimInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerDeviceInProgress1DoSignifyTrust(this.canceller, this.handle!);
      if (result.ok) {
        this.guestSASCode = result.value.claimerSas;
        this.handle = result.value.handle;
      }
      this.canceller = null;
      return result;
    } else {
      this.guestSASCode = '1337';
      return new Promise<Result<DeviceClaimInProgress2Info, ClaimInProgressError>>((resolve, _reject) => {
        resolve({ok: true, value: {
          handle: DEFAULT_HANDLE,
          claimerSas: '1337',
        }});
      });
    }
  }

  async waitHostTrust():
  Promise<Result<DeviceClaimInProgress3Info, ClaimInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerDeviceInProgress2DoWaitPeerTrust(this.canceller, this.handle!);
      this.canceller = null;
      if (result.ok) {
        this.handle = result.value.handle;
      }
      return result;
    } else {
      await wait(MOCK_WAITING_TIME);
      return new Promise<Result<DeviceClaimInProgress3Info, ClaimInProgressError>>((resolve, _reject) => {
        resolve({ok: true, value: {
          handle: DEFAULT_HANDLE,
        }});
      });
    }
  }

  async doClaim(deviceLabel: DeviceLabel):
  Promise<Result<DeviceClaimFinalizeInfo, ClaimInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
      this.canceller = await libparsec.newCanceller();
      const result = await libparsec.claimerDeviceInProgress3DoClaim(
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.canceller, this.handle!, deviceLabel,
      );
      if (result.ok) {
        this.handle = result.value.handle;
      }
      this.canceller = null;
      return result;
    } else {
      await wait(MOCK_WAITING_TIME);
      return new Promise<Result<DeviceClaimFinalizeInfo, ClaimInProgressError>>((resolve, _reject) => {
        resolve({ok: true, value: {
          handle: DEFAULT_HANDLE,
        }});
      });
    }
  }

  async finalize(password: string):
  Promise<Result<AvailableDevice, ClaimInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerDeviceFinalizeSaveLocalDevice(this.handle!, {tag: 'Password', password: password});
      if (result.ok) {
        this.device = result.value;
      }
      this.handle = null;
      return result;
    } else {
      this.handle = null;
      return new Promise<Result<AvailableDevice, ClaimInProgressError>>((resolve, _reject) => {
        this.device = {
          keyFilePath: '/path',
          organizationId: 'MyOrg',
          deviceId: 'deviceid',
          humanHandle: {
            label: 'A',
            email: 'a@b.c',
          },
          deviceLabel: 'a@b',
          slug: 'slug',
          ty: DeviceFileType.Password,
        };
        resolve({ok: true, value: this.device});
      });
    }
  }
}
