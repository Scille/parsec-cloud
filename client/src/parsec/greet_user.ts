// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  libparsec, SASCode,
} from '@/plugins/libparsec';
import {
  Handle,
  InvitationToken,
  ClientStartInvitationGreetError,
  UserGreetInitialInfo,
  Result,
  UserGreetInProgress1Info,
  GreetInProgressError,
  UserGreetInProgress2Info,
  UserGreetInProgress3Info,
  UserGreetInProgress4Info,
  DeviceLabel,
  HumanHandle,
  UserProfile,
} from '@/parsec/types';
import { wait, MOCK_WAITING_TIME, DEFAULT_HANDLE } from '@/parsec/internals';
import { getParsecHandle } from '@/router/conditions';

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
    if (this.canceller !== null && window.isDesktop()) {
      libparsec.cancel(this.canceller);
    }
    if (this.handle !== null && window.isDesktop()) {
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

    if (clientHandle !== null && window.isDesktop()) {
      const result = await libparsec.clientStartUserInvitationGreet(clientHandle, token);

      if (result.ok) {
        this.handle = result.value.handle;
      }
      return result;
    } else {
      this.handle = DEFAULT_HANDLE;
      await wait(MOCK_WAITING_TIME);
      return new Promise<Result<UserGreetInitialInfo, ClientStartInvitationGreetError>>((resolve, _reject) => {
        resolve({ok: true, value: {handle: DEFAULT_HANDLE}});
      });
    }
  }

  async initialWaitGuest(): Promise<Result<UserGreetInProgress1Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
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
      return new Promise<Result<UserGreetInProgress1Info, GreetInProgressError>>((resolve, _reject) => {
        resolve({ok: true, value: {handle: DEFAULT_HANDLE, greeterSas: this.hostSASCode}});
      });
    }
  }

  async waitGuestTrust(): Promise<Result<UserGreetInProgress2Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
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
      return new Promise<Result<UserGreetInProgress2Info, GreetInProgressError>>((resolve, _reject) => {
        resolve({
          ok: true, value: {handle: DEFAULT_HANDLE, claimerSasChoices: this.SASCodeChoices, claimerSas: this.correctSASCode},
        });
      });
    }
  }

  async signifyTrust(): Promise<Result<UserGreetInProgress3Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.greeterUserInProgress2DoSignifyTrust(this.canceller, this.handle!);
      this.canceller = null;
      if (result.ok) {
        this.handle = result.value.handle;
      }
      return result;
    } else {
      return new Promise<Result<UserGreetInProgress3Info, GreetInProgressError>>((resolve, _reject) => {
        resolve({
          ok: true, value: {handle: DEFAULT_HANDLE},
        });
      });
    }
  }

  async getClaimRequests(): Promise<Result<UserGreetInProgress4Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
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
      return new Promise<Result<UserGreetInProgress4Info, GreetInProgressError>>((resolve, _reject) => {
        resolve({
          ok: true, value: {
            handle: DEFAULT_HANDLE,
            requestedDeviceLabel: this.requestedDeviceLabel,
            requestedHumanHandle: this.requestedHumanHandle,
          },
        });
      });
    }
  }

  async createUser(humanHandle: HumanHandle, deviceLabel: DeviceLabel, profile: UserProfile): Promise<Result<null, GreetInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
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
      return new Promise<Result<null, GreetInProgressError>>((resolve, _reject) => {
        resolve({
          ok: true, value: null,
        });
      });
    }
  }
}
