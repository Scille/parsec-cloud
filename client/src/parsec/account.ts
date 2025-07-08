// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultDeviceName } from '@/common/device';
import { getClientConfig, wait } from '@/parsec/internals';
import { SaveStrategy } from '@/parsec/login';
import {
  AccountAccess,
  AccountAccessStrategy,
  AccountCreateError,
  AccountCreateErrorTag,
  AccountCreateSendValidationEmailError,
  AccountCreateSendValidationEmailErrorTag,
  AccountDeleteProceedError,
  AccountDeleteProceedErrorTag,
  AccountDeleteSendValidationEmailError,
  AccountGetHumanHandleError,
  AccountHandle,
  AccountInvitation,
  AccountListInvitationsError,
  AccountListRegistrationDevicesError,
  AccountLoginWithMasterSecretError,
  AccountLoginWithPasswordError,
  AccountLoginWithPasswordErrorTag,
  AccountLogoutError,
  AccountRegisterNewDeviceError,
  AccountRegisterNewDeviceErrorTag,
  AvailableDevice,
  DeviceFileType,
  HumanHandle,
  InvitationType,
  RegistrationDevice,
  Result,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { libparsec } from '@/plugins/libparsec';
import { Env } from '@/services/environment';
import { DateTime } from 'luxon';

class AccountCreationStepper {
  accountInfo?: {
    server: string;
    email: string;
    firstName: string;
    lastName: string;
  };
  code?: Array<string>;

  public get lastName(): string {
    return this.accountInfo?.lastName ?? '';
  }

  public get firstName(): string {
    return this.accountInfo?.firstName ?? '';
  }

  public get email(): string {
    return this.accountInfo?.email ?? '';
  }

  public get server(): string {
    return this.accountInfo?.server ?? '';
  }

  async start(
    firstName: string,
    lastName: string,
    email: string,
    server: string,
  ): Promise<Result<null, AccountCreateSendValidationEmailError>> {
    let result!: Result<null, AccountCreateSendValidationEmailError>;
    if (Env.isAccountMocked()) {
      await wait(1500);
      result = { ok: true, value: null };
    } else {
      result = await libparsec.accountCreate1SendValidationEmail(getClientConfig().configDir, server, email);
      if (!result.ok) {
        return result;
      }
    }
    this.accountInfo = {
      server: server,
      email: email,
      firstName: firstName,
      lastName: lastName,
    };
    return result;
  }

  async resendCode(): Promise<Result<null, AccountCreateSendValidationEmailError>> {
    if (!this.accountInfo) {
      return { ok: false, error: { tag: AccountCreateSendValidationEmailErrorTag.Internal, error: 'invalid_state' } };
    }
    if (Env.isAccountMocked()) {
      await wait(1500);
      return { ok: true, value: null };
    } else {
      return await libparsec.accountCreate1SendValidationEmail(
        getClientConfig().configDir,
        this.accountInfo.server,
        this.accountInfo.email,
      );
    }
  }

  async validateCode(code: Array<string>): Promise<Result<null, AccountCreateError>> {
    if (!this.accountInfo) {
      return { ok: false, error: { tag: AccountCreateErrorTag.Internal, error: 'invalid_state' } };
    }
    if (Env.isAccountMocked()) {
      await wait(1500);
      if (code.join('') === 'ABCDEF') {
        return { ok: true, value: null };
      }
      return { ok: false, error: { tag: AccountCreateErrorTag.InvalidValidationCode, error: 'invalid-token' } };
    } else {
      const result = await libparsec.accountCreate2CheckValidationCode(
        getClientConfig().configDir,
        this.accountInfo.server,
        code.join(''),
        this.accountInfo.email,
      );
      if (result.ok) {
        this.code = code;
      }
      return result;
    }
  }

  async createAccount(authentication: AccountAccess): Promise<Result<null, AccountCreateError>> {
    if (!this.accountInfo || !this.code) {
      return { ok: false, error: { tag: AccountCreateErrorTag.Internal, error: 'invalid_state' } };
    }
    if (authentication.strategy !== AccountAccessStrategy.Password) {
      return { ok: false, error: { tag: AccountCreateErrorTag.Internal, error: 'invalid_authentication' } };
    }
    if (Env.isAccountMocked()) {
      await wait(1500);
      return { ok: true, value: null };
    } else {
      return await libparsec.accountCreate3Proceed(
        getClientConfig().configDir,
        this.accountInfo.server,
        this.code.join(''),
        {
          label: `${this.accountInfo.firstName} ${this.accountInfo.lastName}`,
          email: this.accountInfo.email,
        },
        authentication.password,
      );
    }
  }

  async reset(): Promise<void> {
    this.accountInfo = undefined;
    this.code = undefined;
  }
}

export const ParsecAccountAccess = {
  usePassword(email: string, password: string): AccountAccess {
    return {
      strategy: AccountAccessStrategy.Password,
      email: email,
      password: password,
    };
  },

  useMasterSecret(secret: Uint8Array): AccountAccess {
    return {
      strategy: AccountAccessStrategy.MasterSecret,
      secret: secret,
    };
  },
};

class _ParsecAccount {
  handle: AccountHandle | undefined = undefined;
  skipped: boolean = false;

  constructor() {
    if (Env.isAccountAutoLoginEnabled()) {
      console.log(`Using Parsec Account auto-login, server is '${Env.getAccountServer()}'`);
      libparsec.testNewAccount(Env.getAccountServer()).then((result) => {
        if (!result.ok) {
          console.error(`No auto-login possible, testNewAccount failed: ${result.error.tag} (${result.error.error})`);
          return;
        }
        this.login(ParsecAccountAccess.useMasterSecret(result.value[1]), Env.getAccountServer());
      });
    }
  }

  getHandle(): AccountHandle | undefined {
    return this.handle;
  }

  isLoggedIn(): boolean {
    return this.handle !== undefined;
  }

  markSkipped(): void {
    this.skipped = true;
  }

  isSkipped(): boolean {
    return this.skipped;
  }

  async login(
    authentication: AccountAccess,
    server: string,
  ): Promise<Result<AccountHandle, AccountLoginWithMasterSecretError | AccountLoginWithPasswordError>> {
    if (Env.isAccountMocked()) {
      if (!Env.isAccountAutoLoginEnabled()) {
        await wait(2000);
      }
      if (authentication.strategy === AccountAccessStrategy.Password && authentication.password === 'BigP@ssw0rd.') {
        this.handle = 1;
        return { ok: true, value: 1 };
      }
      return { ok: false, error: { tag: AccountLoginWithPasswordErrorTag.BadPasswordAlgorithm, error: 'Invalid authentication' } };
    } else {
      if (authentication.strategy === AccountAccessStrategy.Password) {
        const result = await libparsec.accountLoginWithPassword(
          getClientConfig().configDir,
          server,
          authentication.email,
          authentication.password,
        );
        if (result.ok) {
          this.handle = result.value;
        }
        return result;
      } else if (authentication.strategy === AccountAccessStrategy.MasterSecret) {
        const result = await libparsec.accountLoginWithMasterSecret(getClientConfig().configDir, server, authentication.secret);
        if (result.ok) {
          this.handle = result.value;
        }
        return result;
      } else {
        return { ok: false, error: { tag: AccountLoginWithPasswordErrorTag.BadPasswordAlgorithm, error: 'Unknown authentication method' } };
      }
    }
  }

  async listInvitations(): Promise<Result<Array<AccountInvitation>, AccountListInvitationsError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountListInvitationsError>();
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      return { ok: true, value: [{ organizationId: 'BlackMesa', token: 'abcdefgh', type: InvitationType.User }] };
    }
    const result = await libparsec.accountListInvitations(this.handle);
    if (result.ok) {
      return {
        ok: true,
        value: result.value.map(([organizationId, token, type]) => {
          return { organizationId: organizationId, token: token, type: type };
        }),
      };
    }
    return result;
  }

  async listRegistrationDevices(): Promise<Result<Array<RegistrationDevice>, AccountListRegistrationDevicesError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountListRegistrationDevicesError>();
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      return { ok: true, value: [] };
    }
    const result = await libparsec.accountListRegistrationDevices(this.handle);
    if (result.ok) {
      return {
        ok: true,
        value: result.value.map(([organizationId, userId]) => {
          return {
            organizationId: organizationId,
            userId: userId,
          };
        }),
      };
    }
    return result;
  }

  async registerNewDevice(
    registrationDevice: RegistrationDevice,
    authentication: AccountAccess,
  ): Promise<Result<AvailableDevice, AccountRegisterNewDeviceError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountRegisterNewDeviceError>();
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      return {
        ok: true,
        value: {
          keyFilePath: '/',
          createdOn: DateTime.now(),
          protectedOn: DateTime.now(),
          serverUrl: 'parsec3://localhost:6770?no_ssl=true',
          organizationId: 'BlackMesa',
          userId: 'abcd',
          deviceId: 'abcd',
          humanHandle: { label: 'Gordon Freeman', email: 'gordon.freeman@blackmesa.nm' },
          deviceLabel: 'HEV Suit',
          ty: DeviceFileType.Password,
        },
      };
    }
    if (authentication.strategy !== AccountAccessStrategy.Password) {
      return { ok: false, error: { tag: AccountRegisterNewDeviceErrorTag.Internal, error: 'invalid-auth' } };
    }
    const saveStrategy = SaveStrategy.usePassword(authentication.password);
    return await libparsec.accountRegisterNewDevice(
      this.handle,
      registrationDevice.organizationId,
      registrationDevice.userId,
      getDefaultDeviceName(),
      saveStrategy,
    );
  }

  async getInfo(): Promise<Result<HumanHandle, AccountGetHumanHandleError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountGetHumanHandleError>();
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      return { ok: true, value: { email: 'gordon.freeman@blackmesa.nm', label: 'Gordon Freeman' } };
    } else {
      return await libparsec.accountGetHumanHandle(this.handle);
    }
  }

  async logout(): Promise<Result<null, AccountLogoutError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountLogoutError>();
    }
    if (Env.isAccountMocked()) {
      this.handle = undefined;
      return { ok: true, value: null };
    }
    const result = await libparsec.accountLogout(this.handle);
    if (result.ok) {
      this.handle = undefined;
    }
    return result;
  }

  async requestAccountDeletion(): Promise<Result<null, AccountDeleteSendValidationEmailError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountDeleteSendValidationEmailError>();
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      return { ok: true, value: null };
    } else {
      return await libparsec.accountDelete1SendValidationEmail(this.handle);
    }
  }

  async confirmAccountDeletion(code: Array<string>): Promise<Result<null, AccountDeleteProceedError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountDeleteProceedError>();
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      if (code.join('') !== 'ABCDEF') {
        return { ok: false, error: { tag: AccountDeleteProceedErrorTag.InvalidValidationCode, error: 'invalid-code' } };
      }
      this.handle = undefined;
      return { ok: true, value: null };
    } else {
      const result = await libparsec.accountDelete2Proceed(this.handle, code.join(''));
      if (result.ok) {
        this.handle = undefined;
      }
      return result;
    }
  }
}

const ParsecAccount = new _ParsecAccount();

export { AccountCreationStepper, ParsecAccount };
