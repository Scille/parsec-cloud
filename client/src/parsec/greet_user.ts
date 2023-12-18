// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { DEFAULT_HANDLE, MOCK_WAITING_TIME, wait } from '@/parsec/internals';
import { getParsecHandle } from '@/parsec/routing';
import {
  ClientStartInvitationGreetError,
  DeviceLabel,
  GreetInProgressError,
  Handle,
  HumanHandle,
  InvitationToken,
  Result,
  UserGreetInProgress1Info,
  UserGreetInProgress2Info,
  UserGreetInProgress3Info,
  UserGreetInProgress4Info,
  UserGreetInitialInfo,
  UserProfile,
} from '@/parsec/types';
import { SASCode, libparsec } from '@/plugins/libparsec';

export class UserGreet {
  handle: Handle | null;
  canceller: Handle | null;
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
    if (this.canceller !== null && !needsMocks()) {
      libparsec.cancel(this.canceller);
    }
    if (this.handle !== null && !needsMocks()) {
      libparsec.claimerGreeterAbortOperation(this.handle);
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
    const clientHandle = getParsecHandle();

    if (clientHandle !== null && !needsMocks()) {
      const result = await libparsec.clientStartUserInvitationGreet(clientHandle, token);

      if (result.ok) {
        this.handle = result.value.handle;
      }
      return result;
    } else {
      this.handle = DEFAULT_HANDLE;
      await wait(MOCK_WAITING_TIME);
      return { ok: true, value: { handle: DEFAULT_HANDLE } };
    }
  }

  async initialWaitGuest(): Promise<Result<UserGreetInProgress1Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.greeterUserInitialDoWaitPeer(this.canceller, this.handle!);
      this.canceller = null;
      if (result.ok) {
        this.handle = result.value.handle;
        this.hostSASCode = result.value.greeterSas;
      }
      return result;
    } else {
      this.hostSASCode = '2EDF';
      return {
        ok: true,
        value: { handle: DEFAULT_HANDLE, greeterSas: this.hostSASCode },
      };
    }
  }

  async waitGuestTrust(): Promise<Result<UserGreetInProgress2Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.greeterUserInProgress1DoWaitPeerTrust(this.canceller, this.handle!);
      this.canceller = null;
      if (result.ok) {
        this.handle = result.value.handle;
        this.SASCodeChoices = result.value.claimerSasChoices;
        this.correctSASCode = result.value.claimerSas;
      }
      return result;
    } else {
      await wait(MOCK_WAITING_TIME);
      this.SASCodeChoices = ['1ABC', '2DEF', '3GHI', '4JKL'];
      this.correctSASCode = '2DEF';
      return {
        ok: true,
        value: {
          handle: DEFAULT_HANDLE,
          claimerSasChoices: this.SASCodeChoices,
          claimerSas: this.correctSASCode,
        },
      };
    }
  }

  async signifyTrust(): Promise<Result<UserGreetInProgress3Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.greeterUserInProgress2DoSignifyTrust(this.canceller, this.handle!);
      this.canceller = null;
      if (result.ok) {
        this.handle = result.value.handle;
      }
      return result;
    } else {
      return { ok: true, value: { handle: DEFAULT_HANDLE } };
    }
  }

  async getClaimRequests(): Promise<Result<UserGreetInProgress4Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.greeterUserInProgress3DoGetClaimRequests(this.canceller, this.handle!);
      this.canceller = null;
      if (result.ok) {
        this.handle = result.value.handle;
        this.requestedHumanHandle = result.value.requestedHumanHandle;
        this.requestedDeviceLabel = result.value.requestedDeviceLabel || '';
      }
      return result;
    } else {
      await wait(MOCK_WAITING_TIME);
      this.requestedHumanHandle = {
        label: 'Gordon Freeman',
        email: 'gordon.freeman@blackmesa.nm',
      };
      this.requestedDeviceLabel = 'My Device';
      return {
        ok: true,
        value: {
          handle: DEFAULT_HANDLE,
          requestedDeviceLabel: this.requestedDeviceLabel,
          requestedHumanHandle: this.requestedHumanHandle,
        },
      };
    }
  }

  async createUser(humanHandle: HumanHandle, deviceLabel: DeviceLabel, profile: UserProfile): Promise<Result<null, GreetInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      const result = await libparsec.greeterUserInProgress4DoCreate(
        this.canceller,
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.handle!,
        humanHandle,
        deviceLabel,
        profile,
      );
      this.canceller = null;
      this.handle = null;
      return result;
    } else {
      this.handle = null;
      return { ok: true, value: null };
    }
  }
}
