// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  AnyClaimRetrievedInfoDevice,
  AnyClaimRetrievedInfoTag,
  AvailableDevice,
  ClaimInProgressError,
  ClaimerRetrieveInfoError,
  ClientEvent,
  ConnectionHandle,
  DeviceClaimFinalizeInfo,
  DeviceClaimInProgress1Info,
  DeviceClaimInProgress2Info,
  DeviceClaimInProgress3Info,
  DeviceFileType,
  DeviceLabel,
  DeviceSaveStrategy,
  HumanHandle,
  Result,
  SASCode,
} from '@/parsec';
import { needsMocks } from '@/parsec/environment';
import { DEFAULT_HANDLE, MOCK_WAITING_TIME, getClientConfig, wait } from '@/parsec/internals';
import { libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export class DeviceClaim {
  handle: ConnectionHandle | null;
  canceller: ConnectionHandle | null;
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
    if (this.canceller !== null && !needsMocks()) {
      await libparsec.cancel(this.canceller);
    }
    if (this.handle !== null && !needsMocks()) {
      await libparsec.claimerGreeterAbortOperation(this.handle);
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

  async retrieveInfo(invitationLink: string): Promise<Result<AnyClaimRetrievedInfoDevice, ClaimerRetrieveInfoError>> {
    function eventCallback(_handle: number, event: ClientEvent): void {
      console.log('On event', event);
    }

    this._assertState(true, true);

    if (!needsMocks()) {
      const clientConfig = getClientConfig();
      const result = await libparsec.claimerRetrieveInfo(clientConfig, eventCallback, invitationLink);
      if (result.ok) {
        if (result.value.tag !== AnyClaimRetrievedInfoTag.Device) {
          throw Error('Unexpected tag');
        }
        this.handle = result.value.handle;
        this.greeter = result.value.greeterHumanHandle;
      }
      return result as Result<AnyClaimRetrievedInfoDevice, ClaimerRetrieveInfoError>;
    } else {
      await wait(MOCK_WAITING_TIME);
      this.handle = DEFAULT_HANDLE;
      this.greeter = {
        email: 'gale@waterdeep.faerun',
        // cspell:disable-next-line
        label: 'Gale Dekarios',
      };
      return {
        ok: true,
        value: {
          tag: AnyClaimRetrievedInfoTag.Device,
          handle: DEFAULT_HANDLE,
          greeterUserId: '1234',
          greeterHumanHandle: {
            email: 'gale@waterdeep.faerun',
            // cspell:disable-next-line
            label: 'Gale Dekarios',
          },
        },
      };
    }
  }

  async initialWaitHost(): Promise<Result<DeviceClaimInProgress1Info, ClaimInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
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
      return {
        ok: true,
        value: {
          handle: DEFAULT_HANDLE,
          greeterUserId: '1234',
          greeterHumanHandle: {
            email: 'gale@waterdeep.faerun',
            // cspell:disable-next-line
            label: 'Gale Dekarios',
          },
          greeterSas: this.correctSASCode,
          greeterSasChoices: this.SASCodeChoices,
        },
      };
    }
  }

  async denyTrust(): Promise<Result<null, ClaimInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerDeviceInProgress1DoDenyTrust(this.canceller, this.handle!);
      this.handle = null;
      this.canceller = null;
      return result;
    } else {
      return { ok: true, value: null };
    }
  }

  async signifyTrust(): Promise<Result<DeviceClaimInProgress2Info, ClaimInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
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
      return {
        ok: true,
        value: {
          handle: DEFAULT_HANDLE,
          claimerSas: '1337',
        },
      };
    }
  }

  async waitHostTrust(): Promise<Result<DeviceClaimInProgress3Info, ClaimInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
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
      return {
        ok: true,
        value: {
          handle: DEFAULT_HANDLE,
        },
      };
    }
  }

  async doClaim(deviceLabel: DeviceLabel): Promise<Result<DeviceClaimFinalizeInfo, ClaimInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      const result = await libparsec.claimerDeviceInProgress3DoClaim(
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.canceller,
        this.handle!,
        deviceLabel,
      );
      if (result.ok) {
        this.handle = result.value.handle;
      }
      this.canceller = null;
      return result;
    } else {
      await wait(MOCK_WAITING_TIME);
      return {
        ok: true,
        value: {
          handle: DEFAULT_HANDLE,
        },
      };
    }
  }

  async finalize(saveStrategy: DeviceSaveStrategy): Promise<Result<AvailableDevice, ClaimInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      const result = await libparsec.claimerDeviceFinalizeSaveLocalDevice(
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.handle!,
        saveStrategy,
      );
      if (result.ok) {
        result.value.createdOn = DateTime.fromSeconds(result.value.createdOn as any as number);
        result.value.protectedOn = DateTime.fromSeconds(result.value.protectedOn as any as number);
        this.device = result.value;
      }
      this.handle = null;
      return result;
    } else {
      this.handle = null;
      this.device = {
        keyFilePath: '/path',
        serverUrl: 'https://parsec.invalid',
        createdOn: DateTime.utc(),
        protectedOn: DateTime.utc(),
        organizationId: 'MyOrg',
        userId: 'userid',
        deviceId: 'deviceid',
        humanHandle: {
          label: 'A',
          email: 'a@b.c',
        },
        deviceLabel: 'a@b',
        ty: DeviceFileType.Password,
      };
      return { ok: true, value: this.device };
    }
  }
}
