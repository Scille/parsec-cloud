// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  AnyClaimRetrievedInfoDevice,
  AnyClaimRetrievedInfoTag,
  AvailableDevice,
  ClaimFinalizeError,
  ClaimInProgressError,
  ClaimerRetrieveInfoError,
  ConnectionHandle,
  DeviceClaimFinalizeInfo,
  DeviceClaimInProgress1Info,
  DeviceClaimInProgress2Info,
  DeviceClaimInProgress3Info,
  DeviceLabel,
  DeviceSaveStrategy,
  HumanHandle,
  Result,
  SASCode,
} from '@/parsec';
import { getClientConfig } from '@/parsec/internals';
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
    if (this.canceller !== null) {
      await libparsec.cancel(this.canceller);
    }
    if (this.handle !== null) {
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
    this._assertState(true, true);

    const clientConfig = getClientConfig();
    const result = await libparsec.claimerRetrieveInfo(clientConfig, invitationLink);
    if (result.ok) {
      if (result.value.tag !== AnyClaimRetrievedInfoTag.Device) {
        throw Error('Unexpected tag');
      }
      this.handle = result.value.handle;
      this.greeter = result.value.greeterHumanHandle;
    }
    return result as Result<AnyClaimRetrievedInfoDevice, ClaimerRetrieveInfoError>;
  }

  async initialWaitHost(): Promise<Result<DeviceClaimInProgress1Info, ClaimInProgressError>> {
    this._assertState(true, false);

    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerDeviceInitialDoWaitPeer(this.canceller, this.handle!);
    if (result.ok) {
      this.SASCodeChoices = result.value.greeterSasChoices;
      this.correctSASCode = result.value.greeterSas;
      this.handle = result.value.handle;
    }
    this.canceller = null;
    return result;
  }

  async denyTrust(): Promise<Result<null, ClaimInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerDeviceInProgress1DoDenyTrust(this.canceller, this.handle!);
    this.handle = null;
    this.canceller = null;
    return result;
  }

  async signifyTrust(): Promise<Result<DeviceClaimInProgress2Info, ClaimInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerDeviceInProgress1DoSignifyTrust(this.canceller, this.handle!);
    if (result.ok) {
      this.guestSASCode = result.value.claimerSas;
      this.handle = result.value.handle;
    }
    this.canceller = null;
    return result;
  }

  async waitHostTrust(): Promise<Result<DeviceClaimInProgress3Info, ClaimInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerDeviceInProgress2DoWaitPeerTrust(this.canceller, this.handle!);
    this.canceller = null;
    if (result.ok) {
      this.handle = result.value.handle;
    }
    return result;
  }

  async doClaim(deviceLabel: DeviceLabel): Promise<Result<DeviceClaimFinalizeInfo, ClaimInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerDeviceInProgress3DoClaim(this.canceller, this.handle!, deviceLabel);
    if (result.ok) {
      this.handle = result.value.handle;
    }
    this.canceller = null;
    return result;
  }

  async finalize(saveStrategy: DeviceSaveStrategy): Promise<Result<AvailableDevice, ClaimFinalizeError>> {
    this._assertState(true, false);
    const result = await libparsec.claimerDeviceFinalizeSaveLocalDevice(this.handle!, saveStrategy);
    if (result.ok) {
      result.value.createdOn = DateTime.fromSeconds(result.value.createdOn as any as number);
      result.value.protectedOn = DateTime.fromSeconds(result.value.protectedOn as any as number);
      this.device = result.value;
    }
    this.handle = null;
    return result;
  }
}
