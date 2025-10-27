// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  ClientStartInvitationGreetError,
  ConnectionHandle,
  DeviceLabel,
  GreetInProgressError,
  HumanHandle,
  InvitationToken,
  Result,
  UserGreetInProgress1Info,
  UserGreetInProgress2Info,
  UserGreetInProgress3Info,
  UserGreetInProgress4Info,
  UserGreetInitialInfo,
  UserProfile,
} from '@/parsec';
import { generateNoHandleError } from '@/parsec/utils';
import { SASCode, libparsec } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';

export class UserGreet {
  handle: ConnectionHandle | null;
  canceller: ConnectionHandle | null;
  hostSASCode: SASCode;
  correctSASCode: SASCode;
  SASCodeChoices: SASCode[];
  requestedHumanHandle: HumanHandle | null;
  requestedDeviceLabel: DeviceLabel;

  constructor() {
    this.handle = null;
    this.canceller = null;
    this.hostSASCode = '';
    this.correctSASCode = '';
    this.SASCodeChoices = [];
    this.requestedDeviceLabel = '';
    this.requestedHumanHandle = null;
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
    this.hostSASCode = '';
    this.requestedDeviceLabel = '';
    this.requestedHumanHandle = null;
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

  async startGreet(token: InvitationToken): Promise<Result<UserGreetInitialInfo, ClientStartInvitationGreetError>> {
    const clientHandle = getConnectionHandle();

    if (clientHandle !== null) {
      const result = await libparsec.clientStartUserInvitationGreet(clientHandle, token);

      if (result.ok) {
        this.handle = result.value.handle;
      }
      return result;
    }
    return generateNoHandleError<ClientStartInvitationGreetError>();
  }

  async initialWaitGuest(): Promise<Result<UserGreetInProgress1Info, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterUserInitialDoWaitPeer(this.canceller, this.handle!);
    this.canceller = null;
    if (result.ok) {
      this.handle = result.value.handle;
      this.hostSASCode = result.value.greeterSas;
    }
    return result;
  }

  async waitGuestTrust(): Promise<Result<UserGreetInProgress2Info, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterUserInProgress1DoWaitPeerTrust(this.canceller, this.handle!);
    if (result.ok) {
      this.handle = result.value.handle;
      this.SASCodeChoices = result.value.claimerSasChoices;
      this.correctSASCode = result.value.claimerSas;
    }
    this.canceller = null;
    return result;
  }

  async denyTrust(): Promise<Result<null, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterUserInProgress2DoDenyTrust(this.canceller, this.handle!);
    this.handle = null;
    this.canceller = null;
    return result;
  }

  async signifyTrust(): Promise<Result<UserGreetInProgress3Info, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterUserInProgress2DoSignifyTrust(this.canceller, this.handle!);
    if (result.ok) {
      this.handle = result.value.handle;
    }
    this.canceller = null;
    return result;
  }

  async getClaimRequests(): Promise<Result<UserGreetInProgress4Info, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterUserInProgress3DoGetClaimRequests(this.canceller, this.handle!);
    this.canceller = null;
    if (result.ok) {
      this.handle = result.value.handle;
      this.requestedHumanHandle = result.value.requestedHumanHandle;
      this.requestedDeviceLabel = result.value.requestedDeviceLabel || '';
    }
    return result;
  }

  async createUser(humanHandle: HumanHandle, profile: UserProfile): Promise<Result<null, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterUserInProgress4DoCreate(
      this.canceller,
      this.handle!,
      humanHandle,
      this.requestedDeviceLabel,
      profile,
    );
    this.canceller = null;
    this.handle = null;
    return result;
  }
}
