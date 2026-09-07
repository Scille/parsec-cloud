// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultDeviceName } from '@/common/device';
import {
  AvailableDevice,
  CancelHandle,
  ClaimFinalizeError,
  ClaimInProgressError,
  ClientDeleteShamirRecoveryError,
  ClientGetSelfShamirRecoveryError,
  ClientGetSelfShamirRecoveryErrorTag,
  ClientListShamirRecoveriesForOthersError,
  ClientListShamirRecoveriesForOthersErrorTag,
  ClientNewShamirRecoveryInvitationError,
  ClientSetupShamirRecoveryError,
  ClientStartShamirRecoveryInvitationGreetError,
  DeviceSaveStrategy,
  ExchangeHandle,
  GreetInProgressError,
  NewInvitationInfo,
  OtherShamirRecoveryInfo,
  Result,
  SelfShamirRecoveryInfo,
  SelfShamirRecoveryInfoSetupAllValid,
  SelfShamirRecoveryInfoTag,
  ShamirRecoveryClaimAddShareError,
  ShamirRecoveryClaimInitialInfo,
  ShamirRecoveryClaimInProgress1Info,
  ShamirRecoveryClaimInProgress2Info,
  ShamirRecoveryClaimInProgress3Info,
  ShamirRecoveryClaimMaybeFinalizeInfo,
  ShamirRecoveryClaimMaybeRecoverDeviceInfo,
  ShamirRecoveryClaimPickRecipientError,
  ShamirRecoveryClaimRecoverDeviceError,
  ShamirRecoveryClaimShareInfo,
  ShamirRecoveryGreetInitialInfo,
  ShamirRecoveryGreetInProgress1Info,
  ShamirRecoveryGreetInProgress2Info,
  ShamirRecoveryGreetInProgress3Info,
  UserID,
  UserInfo,
} from '@/parsec/types';
import { getUserInfo } from '@/parsec/user';
import { generateNoHandleError } from '@/parsec/utils';
import { libparsec } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';
import { DateTime } from 'luxon';

const SHAMIR_THRESHOLD = 2;

export async function getRequiredShamirThreshold(): Promise<number> {
  return SHAMIR_THRESHOLD;
}

export async function getSelfShamirRecovery(): Promise<Result<SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError>> {
  const handle = getConnectionHandle();

  if (handle === null) {
    return generateNoHandleError<ClientGetSelfShamirRecoveryError>();
  }
  const result = await libparsec.clientGetSelfShamirRecovery(handle);
  if (result.ok) {
    (result.value as SelfShamirRecoveryInfo).isUsable = () => false;
    if (result.value.tag !== SelfShamirRecoveryInfoTag.NeverSetup) {
      result.value.createdOn = DateTime.fromSeconds(result.value.createdOn as any as number);
    }
    if (
      result.value.tag === SelfShamirRecoveryInfoTag.SetupAllValid ||
      result.value.tag === SelfShamirRecoveryInfoTag.SetupWithRevokedRecipients
    ) {
      (result.value as SelfShamirRecoveryInfo).isUsable = () => true;
      try {
        const users = await Promise.all(
          result.value.perRecipientShares.keys().map(async (userId) => {
            const userResult = await getUserInfo(userId);
            if (userResult.ok) {
              return userResult.value;
            }
            throw new Error(userResult.error.tag);
          }),
        );
        (result.value as SelfShamirRecoveryInfoSetupAllValid).recipients = users.sort((u1, u2) =>
          u1.humanHandle.label.localeCompare(u2.humanHandle.label),
        );
      } catch (_err: any) {
        return { ok: false, error: { tag: ClientGetSelfShamirRecoveryErrorTag.Internal, error: 'Failed to get user info' } };
      }
    } else if (result.value.tag === SelfShamirRecoveryInfoTag.Deleted) {
      result.value.deletedOn = DateTime.fromSeconds(result.value.deletedOn as any as number);
    }
  }
  return result as Result<SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError>;
}

export async function deleteSelfShamirRecovery(): Promise<Result<null, ClientDeleteShamirRecoveryError>> {
  const handle = getConnectionHandle();

  if (handle === null) {
    return generateNoHandleError<ClientDeleteShamirRecoveryError>();
  }
  return await libparsec.clientDeleteShamirRecovery(handle);
}

export async function getOthersShamirRecovery(): Promise<Result<Array<OtherShamirRecoveryInfo>, ClientListShamirRecoveriesForOthersError>> {
  const handle = getConnectionHandle();

  if (handle === null) {
    return generateNoHandleError<ClientListShamirRecoveriesForOthersError>();
  }
  const result = await libparsec.clientListShamirRecoveriesForOthers(handle);

  if (result.ok) {
    try {
      result.value = await Promise.all(
        result.value.map(async (shamirInfo) => {
          shamirInfo.createdOn = DateTime.fromSeconds(shamirInfo.createdOn as any as number);
          const userResult = await getUserInfo(shamirInfo.userId);
          if (userResult.ok) {
            (shamirInfo as OtherShamirRecoveryInfo).owner = userResult.value;
          } else {
            throw new Error(userResult.error.tag);
          }
          return shamirInfo;
        }),
      );
    } catch (_err: any) {
      return { ok: false, error: { tag: ClientListShamirRecoveriesForOthersErrorTag.Internal, error: 'Failed to get user info' } };
    }
  }
  return result as Result<Array<OtherShamirRecoveryInfo>, ClientListShamirRecoveriesForOthersError>;
}

export async function setupShamirRecovery(
  users: Array<UserInfo>,
  threshold: number,
  partsPerRecipient: number,
): Promise<Result<null, ClientSetupShamirRecoveryError>> {
  const handle = getConnectionHandle();

  if (handle === null) {
    return generateNoHandleError<ClientSetupShamirRecoveryError>();
  }
  const map = new Map(users.map((user) => [user.id, partsPerRecipient]));
  return await libparsec.clientSetupShamirRecovery(handle, map as any as Map<UserID, number>, threshold);
}

export async function shamirRecoveryInvite(
  targetUser: UserID,
  sendEmail?: boolean,
): Promise<Result<NewInvitationInfo, ClientNewShamirRecoveryInvitationError>> {
  const handle = getConnectionHandle();

  if (handle === null) {
    return generateNoHandleError<ClientNewShamirRecoveryInvitationError>();
  }
  return await libparsec.clientNewShamirRecoveryInvitation(handle, targetUser, sendEmail ?? false);
}

export async function startShamirGreet(
  token: string,
): Promise<Result<ShamirRecoveryGreetInitialInfo, ClientStartShamirRecoveryInvitationGreetError>> {
  const handle = getConnectionHandle();

  if (handle === null) {
    return generateNoHandleError<ClientStartShamirRecoveryInvitationGreetError>();
  }
  return await libparsec.clientStartShamirRecoveryInvitationGreet(handle, token);
}

export async function shamirPickRecipient(
  handle: ExchangeHandle,
  recipientId: UserID,
): Promise<Result<ShamirRecoveryClaimInitialInfo, ShamirRecoveryClaimPickRecipientError>> {
  return await libparsec.claimerShamirRecoveryPickRecipient(handle, recipientId);
}

export class ShamirClaimer {
  info: ShamirRecoveryClaimInitialInfo;
  canceller: CancelHandle | undefined;
  greeterSasCode: string;
  greeterSasChoices: Array<string>;
  claimerSasCode: string;
  stepHandle: ExchangeHandle;

  constructor(info: ShamirRecoveryClaimInitialInfo) {
    this.info = info;
    this.canceller = undefined;
    this.greeterSasChoices = [];
    this.greeterSasCode = '';
    this.claimerSasCode = '';
    this.stepHandle = info.handle;
  }

  async initialWaitGreeter(): Promise<Result<ShamirRecoveryClaimInProgress1Info, ClaimInProgressError>> {
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerShamirRecoveryInitialDoWaitPeer(this.canceller, this.stepHandle);
    if (result.ok) {
      this.greeterSasCode = result.value.greeterSas;
      this.greeterSasChoices = result.value.greeterSasChoices;
      this.stepHandle = result.value.handle;
    }
    this.canceller = undefined;
    return result;
  }

  async denyTrust(): Promise<Result<null, ClaimInProgressError>> {
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerShamirRecoveryInProgress1DoDenyTrust(this.canceller, this.stepHandle);
    this.canceller = undefined;
    return result;
  }

  async signifyTrust(): Promise<Result<ShamirRecoveryClaimInProgress2Info, ClaimInProgressError>> {
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerShamirRecoveryInProgress1DoSignifyTrust(this.canceller, this.stepHandle);
    if (result.ok) {
      this.stepHandle = result.value.handle;
      this.claimerSasCode = result.value.claimerSas;
    }
    this.canceller = undefined;
    return result;
  }

  async waitGreeterTrust(): Promise<Result<ShamirRecoveryClaimInProgress3Info, ClaimInProgressError>> {
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerShamirRecoveryInProgress2DoWaitPeerTrust(this.canceller, this.stepHandle);
    if (result.ok) {
      this.stepHandle = result.value.handle;
    }
    this.canceller = undefined;
    return result;
  }

  async claim(): Promise<Result<ShamirRecoveryClaimShareInfo, ClaimInProgressError>> {
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.claimerShamirRecoveryInProgress3DoClaim(this.canceller, this.stepHandle);
    if (result.ok) {
      this.stepHandle = result.value.handle;
    }
    this.canceller = undefined;
    return result;
  }

  async addShare(
    globalHandle: ExchangeHandle,
  ): Promise<Result<ShamirRecoveryClaimMaybeRecoverDeviceInfo, ShamirRecoveryClaimAddShareError>> {
    return await libparsec.claimerShamirRecoveryAddShare(globalHandle, this.stepHandle);
  }

  async cancel(): Promise<void> {
    if (!this.canceller) {
      return;
    }
    await libparsec.cancel(this.canceller);
    this.canceller = undefined;
  }
}

export async function shamirRecoverDevice(
  handle: ExchangeHandle,
): Promise<Result<ShamirRecoveryClaimMaybeFinalizeInfo, ShamirRecoveryClaimRecoverDeviceError>> {
  return await libparsec.claimerShamirRecoveryRecoverDevice(handle, getDefaultDeviceName());
}

export async function shamirSaveDevice(
  handle: ExchangeHandle,
  saveStrategy: DeviceSaveStrategy,
): Promise<Result<AvailableDevice, ClaimFinalizeError>> {
  return await libparsec.claimerShamirRecoveryFinalizeSaveLocalDevice(handle, saveStrategy);
}

export class ShamirGreeter {
  canceller: CancelHandle | undefined;
  claimerSasCode: string;
  claimerSasChoices: Array<string>;
  greeterSasCode: string;
  stepHandle: ExchangeHandle;

  constructor(initialHandle: ExchangeHandle) {
    this.canceller = undefined;
    this.claimerSasCode = '';
    this.claimerSasChoices = [];
    this.greeterSasCode = '';
    this.stepHandle = initialHandle;
  }

  async initialWaitClaimer(): Promise<Result<ShamirRecoveryGreetInProgress1Info, GreetInProgressError>> {
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterShamirRecoveryInitialDoWaitPeer(this.canceller, this.stepHandle);
    if (result.ok) {
      this.stepHandle = result.value.handle;
      this.greeterSasCode = result.value.greeterSas;
    }
    this.canceller = undefined;
    return result;
  }

  async waitClaimerTrust(): Promise<Result<ShamirRecoveryGreetInProgress2Info, GreetInProgressError>> {
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterShamirRecoveryInProgress1DoWaitPeerTrust(this.canceller, this.stepHandle);
    if (result.ok) {
      this.stepHandle = result.value.handle;
      this.claimerSasChoices = result.value.claimerSasChoices;
      this.claimerSasCode = result.value.claimerSas;
    }
    this.canceller = undefined;
    return result;
  }

  async denyTrust(): Promise<Result<null, GreetInProgressError>> {
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterShamirRecoveryInProgress2DoDenyTrust(this.canceller, this.stepHandle);
    this.canceller = undefined;
    return result;
  }

  async signifyTrust(): Promise<Result<ShamirRecoveryGreetInProgress3Info, GreetInProgressError>> {
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterShamirRecoveryInProgress2DoSignifyTrust(this.canceller, this.stepHandle);
    if (result.ok) {
      this.stepHandle = result.value.handle;
    }
    this.canceller = undefined;
    return result;
  }

  async finalize(): Promise<Result<null, GreetInProgressError>> {
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterShamirRecoveryInProgress3DoGetClaimRequests(this.canceller, this.stepHandle);
    this.canceller = undefined;
    return result;
  }

  async cancel(): Promise<void> {
    if (!this.canceller) {
      return;
    }
    await libparsec.cancel(this.canceller);
    this.canceller = undefined;
  }
}
