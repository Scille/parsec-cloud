// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  AnyClaimRetrievedInfoTag,
  AnyClaimRetrievedInfoUser,
  AvailableDevice,
  ClaimInProgressError,
  ClaimerRetrieveInfoError,
  ClientEvent,
  ConnectionHandle,
  DeviceFileType,
  DeviceSaveStrategy,
  InviteInfoInvitationCreatedByTag,
  Result,
  SASCode,
  UserClaimFinalizeInfo,
  UserClaimInProgress1Info,
  UserClaimInProgress2Info,
  UserClaimInProgress3Info,
  UserClaimInitialInfo,
  UserOnlineStatus,
} from '@/parsec';
import { needsMocks } from '@/parsec/environment';
import { DEFAULT_HANDLE, MOCK_WAITING_TIME, getClientConfig, wait } from '@/parsec/internals';
import { libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export class UserClaim {
  handle: ConnectionHandle | null;
  cancellers: Array<ConnectionHandle>;
  SASCodeChoices: SASCode[];
  correctSASCode: SASCode;
  guestSASCode: SASCode;
  device: AvailableDevice | null;
  greeter: UserClaimInitialInfo | null;
  possibleGreeters: Array<UserClaimInitialInfo>;

  constructor() {
    this.handle = null;
    this.cancellers = [];
    this.guestSASCode = '';
    this.correctSASCode = '';
    this.SASCodeChoices = [];
    this.device = null;
    this.greeter = null;
    this.possibleGreeters = [];
  }

  async abort(): Promise<void> {
    if (!needsMocks()) {
      for (const canceller of this.cancellers) {
        await libparsec.cancel(canceller);
      }
    }
    if (this.handle !== null && !needsMocks()) {
      await libparsec.claimerGreeterAbortOperation(this.handle);
    }
    this.cancellers = [];
    this.handle = null;
    this.guestSASCode = '';
    this.correctSASCode = '';
    this.SASCodeChoices = [];
    this.device = null;
    this.greeter = null;
    this.possibleGreeters = [];
  }

  _assertState(nullCanceller: boolean, nullHandle: boolean): void {
    if (nullCanceller && this.cancellers.length !== 0) {
      throw Error('Canceller should be null');
    } else if (!nullCanceller && this.cancellers.length === 0) {
      throw Error('Canceller should not be null');
    }
    if (nullHandle && this.handle !== null) {
      throw Error('Handle should be null');
    } else if (!nullHandle && this.handle === null) {
      throw Error('Handle should not be null');
    }
  }

  async retrieveInfo(invitationLink: string): Promise<Result<AnyClaimRetrievedInfoUser, ClaimerRetrieveInfoError>> {
    function eventCallback(_handle: number, event: ClientEvent): void {
      console.log('On event', event);
    }

    this._assertState(true, true);

    if (!needsMocks()) {
      const clientConfig = getClientConfig();
      const result = await libparsec.claimerRetrieveInfo(clientConfig, eventCallback, invitationLink);
      if (result.ok) {
        if (result.value.tag !== AnyClaimRetrievedInfoTag.User) {
          throw Error('Unexpected tag');
        }
        for (const initInfo of result.value.userClaimInitialInfos) {
          this.possibleGreeters.push(initInfo);
        }
      }
      return result as Result<AnyClaimRetrievedInfoUser, ClaimerRetrieveInfoError>;
    } else {
      await wait(MOCK_WAITING_TIME);
      this.possibleGreeters.push({
        handle: DEFAULT_HANDLE,
        greeterUserId: '1',
        greeterHumanHandle: { email: 'a@b.c', label: 'Aaaaaaaaa Bbbbbbbb' },
        onlineStatus: UserOnlineStatus.Unknown,
        lastGreetingAttemptJoinedOn: null,
      });
      this.possibleGreeters.push({
        handle: DEFAULT_HANDLE,
        greeterUserId: '2',
        greeterHumanHandle: { email: 'd@e.f', label: 'Dddddddddddddd Eeeeeeeeeeeeee' },
        onlineStatus: UserOnlineStatus.Unknown,
        lastGreetingAttemptJoinedOn: null,
      });
      this.possibleGreeters.push({
        handle: DEFAULT_HANDLE,
        greeterUserId: '3',
        greeterHumanHandle: { email: 'g@h.i', label: 'Gggggggggggg Hhhhh' },
        onlineStatus: UserOnlineStatus.Unknown,
        lastGreetingAttemptJoinedOn: null,
      });
      return {
        ok: true,
        value: {
          tag: AnyClaimRetrievedInfoTag.User,
          claimerEmail: 'shadowheart@swordcoast.faerun',
          userClaimInitialInfos: this.possibleGreeters,
          createdBy: {
            tag: InviteInfoInvitationCreatedByTag.User,
            userId: 'ewferhgiuerg',
            humanHandle: {
              email: 'gordon.freeman@blackmesa.nm',
              label: 'Gordon Freeman',
            },
          },
        },
      };
    }
  }

  private async _doWaitPeer(
    canceller: ConnectionHandle,
    info: UserClaimInitialInfo,
  ): Promise<Result<[UserClaimInitialInfo, UserClaimInProgress1Info], ClaimInProgressError>> {
    const result = await libparsec.claimerUserInitialDoWaitPeer(canceller, info.handle);
    if (result.ok) {
      return { ok: true, value: [info, result.value] };
    }
    return result;
  }

  async initialWaitHost(): Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>> {
    if (!needsMocks()) {
      const promises: Array<Promise<Result<[UserClaimInitialInfo, UserClaimInProgress1Info], ClaimInProgressError>>> = [];
      for (const info of this.possibleGreeters) {
        const canceller = await libparsec.newCanceller();
        this.cancellers.push(canceller);
        promises.push(this._doWaitPeer(canceller, info));
      }
      const result = await Promise.any(promises);
      // Cancel remaining
      for (const canceller of this.cancellers) {
        await libparsec.cancel(canceller);
      }
      this.cancellers = [];
      // Wait for other promises to finish
      await Promise.allSettled(promises);

      if (result.ok) {
        const initInfo = result.value[0];
        const progressInfo = result.value[1];

        this.greeter = initInfo;
        this.handle = progressInfo.handle;
        this.SASCodeChoices = progressInfo.greeterSasChoices;
        this.correctSASCode = progressInfo.greeterSas;
        return { ok: true, value: progressInfo };
      } else {
        return { ok: false, error: result.error };
      }
    } else {
      this.SASCodeChoices = ['1ABC', '2DEF', '3GHI', '4JKL'];
      this.correctSASCode = '2DEF';
      return {
        ok: true,
        value: {
          handle: DEFAULT_HANDLE,
          greeterSas: '2DEF',
          greeterSasChoices: ['1ABC', '2DEF', '3GHI', '4JKL'],
        },
      };
    }
  }

  async denyTrust(): Promise<Result<null, ClaimInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.cancellers.push(await libparsec.newCanceller());
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerUserInProgress1DoDenyTrust(this.cancellers[0], this.handle!);
      this.handle = null;
      this.cancellers = [];
      return result;
    } else {
      return { ok: true, value: null };
    }
  }

  async signifyTrust(): Promise<Result<UserClaimInProgress2Info, ClaimInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.cancellers.push(await libparsec.newCanceller());
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerUserInProgress1DoSignifyTrust(this.cancellers[0], this.handle!);
      if (result.ok) {
        this.guestSASCode = result.value.claimerSas;
        this.handle = result.value.handle;
      }
      this.cancellers = [];
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

  async waitHostTrust(): Promise<Result<UserClaimInProgress3Info, ClaimInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.cancellers.push(await libparsec.newCanceller());
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerUserInProgress2DoWaitPeerTrust(this.cancellers[0], this.handle!);
      this.cancellers = [];
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

  async doClaim(deviceLabel: string, userName: string, email: string): Promise<Result<UserClaimFinalizeInfo, ClaimInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.cancellers.push(await libparsec.newCanceller());
      const result = await libparsec.claimerUserInProgress3DoClaim(
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.cancellers[0],
        this.handle!,
        deviceLabel,
        { email: email, label: userName },
      );
      if (result.ok) {
        this.handle = result.value.handle;
      }
      this.cancellers = [];
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
      const result = await libparsec.claimerUserFinalizeSaveLocalDevice(
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
