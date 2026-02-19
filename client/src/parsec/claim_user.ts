// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  AnyClaimRetrievedInfoTag,
  AnyClaimRetrievedInfoUser,
  AvailableDevice,
  ClaimerRetrieveInfoError,
  ClaimFinalizeError,
  ClaimInProgressError,
  ConnectionHandle,
  DeviceSaveStrategy,
  HumanHandle,
  Result,
  SASCode,
  UserClaimFinalizeInfo,
  UserClaimInProgress1Info,
  UserClaimInProgress2Info,
  UserClaimInProgress3Info,
} from '@/parsec';
import { getClientConfig } from '@/parsec/internals';
import { libparsec, ParsecInvitationAddr, ParsecInvitationRedirectionURL } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export class UserClaim {
  handle: ConnectionHandle | null;
  canceller: ConnectionHandle | null;
  SASCodeChoices: SASCode[];
  correctSASCode: SASCode;
  guestSASCode: SASCode;
  device: AvailableDevice | null;
  greeter: HumanHandle | null;
  preferredGreeter: HumanHandle | null;

  constructor() {
    this.handle = null;
    this.canceller = null;
    this.guestSASCode = '';
    this.correctSASCode = '';
    this.SASCodeChoices = [];
    this.device = null;
    this.greeter = null;
    this.preferredGreeter = null;
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
    this.preferredGreeter = null;
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

  async retrieveInfo(
    invitationLink: ParsecInvitationAddr | ParsecInvitationRedirectionURL,
  ): Promise<Result<AnyClaimRetrievedInfoUser, ClaimerRetrieveInfoError>> {
    this._assertState(true, true);

    const clientConfig = getClientConfig();
    const result = await libparsec.claimerRetrieveInfo(clientConfig, invitationLink);
    if (result.ok) {
      if (result.value.tag !== AnyClaimRetrievedInfoTag.User) {
        throw Error('Unexpected tag');
      }
      this.handle = result.value.handle;
      this.preferredGreeter = result.value.preferredGreeter?.humanHandle ?? null;
    }
    return result as Result<AnyClaimRetrievedInfoUser, ClaimerRetrieveInfoError>;
  }

  async initialWaitAllAdministrators(): Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerUserWaitAllPeers(this.canceller, this.handle!);
    if (result.ok) {
      this.SASCodeChoices = result.value.greeterSasChoices;
      this.correctSASCode = result.value.greeterSas;
      this.greeter = result.value.greeterHumanHandle;
      this.handle = result.value.handle;
    }
    this.canceller = null;
    return result;
  }

  async denyTrust(): Promise<Result<null, ClaimInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerUserInProgress1DoDenyTrust(this.canceller, this.handle!);
    this.handle = null;
    this.canceller = null;
    return result;
  }

  async signifyTrust(): Promise<Result<UserClaimInProgress2Info, ClaimInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerUserInProgress1DoSignifyTrust(this.canceller, this.handle!);
    if (result.ok) {
      this.guestSASCode = result.value.claimerSas;
      this.handle = result.value.handle;
    }
    this.canceller = null;
    return result;
  }

  async waitHostTrust(): Promise<Result<UserClaimInProgress3Info, ClaimInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerUserInProgress2DoWaitPeerTrust(this.canceller, this.handle!);
    this.canceller = null;
    if (result.ok) {
      this.handle = result.value.handle;
    }
    return result;
  }

  async doClaim(deviceLabel: string, userName: string, email: string): Promise<Result<UserClaimFinalizeInfo, ClaimInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerUserInProgress3DoClaim(this.canceller, this.handle!, deviceLabel, {
      email: email,
      label: userName,
    });
    if (result.ok) {
      this.handle = result.value.handle;
    }
    this.canceller = null;
    return result;
  }

  async finalize(saveStrategy: DeviceSaveStrategy): Promise<Result<AvailableDevice, ClaimFinalizeError>> {
    this._assertState(true, false);
    const result = await libparsec.claimerUserFinalizeSaveLocalDevice(this.handle!, saveStrategy);
    if (result.ok) {
      result.value.createdOn = DateTime.fromSeconds(result.value.createdOn as any as number);
      result.value.protectedOn = DateTime.fromSeconds(result.value.protectedOn as any as number);
      this.device = result.value;
    }
    this.handle = null;
    return result;
  }
}
