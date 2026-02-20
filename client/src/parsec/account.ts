// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultDeviceName } from '@/common/device';
import { isWeb } from '@/parsec/environment';
import { getClientConfig } from '@/parsec/internals';
import { constructSaveStrategy, listAvailableDevices, PrimaryProtectionStrategy } from '@/parsec/login';
import {
  AccountAuthMethodStrategy,
  AccountAuthMethodStrategyTag,
  AccountCreateAuthMethodError,
  AccountCreateError,
  AccountCreateErrorTag,
  AccountCreateRegistrationDeviceError,
  AccountCreateSendValidationEmailError,
  AccountCreateSendValidationEmailErrorTag,
  AccountDeleteProceedError,
  AccountDeleteSendValidationEmailError,
  AccountHandle,
  AccountInfo,
  AccountInfoError,
  AccountInvitation,
  AccountListAuthMethodsError,
  AccountListInvitationsError,
  AccountListRegistrationDevicesError,
  AccountListRegistrationDevicesErrorTag,
  AccountLoginError,
  AccountLoginStrategy,
  AccountLoginStrategyTag,
  AccountLogoutError,
  AccountRecoverProceedError,
  AccountRecoverSendValidationEmailError,
  AccountRegisterNewDeviceError,
  AuthMethodInfo,
  AvailableDevice,
  AvailableDeviceTypeTag,
  DeviceAccessStrategy,
  DeviceSaveStrategy,
  ParsecAddr,
  RegistrationDevice,
  Result,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { libparsec } from '@/plugins/libparsec';
import { Env } from '@/services/environment';
import { DateTime } from 'luxon';

function getAccountDefaultDeviceName(): string {
  return `Account_${getDefaultDeviceName()}`;
}

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
    const result = await libparsec.accountCreate1SendValidationEmail(getClientConfig().configDir, server, email);
    if (!result.ok) {
      return result;
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
    return await libparsec.accountCreate1SendValidationEmail(getClientConfig().configDir, this.accountInfo.server, this.accountInfo.email);
  }

  async validateCode(code: Array<string>): Promise<Result<null, AccountCreateError>> {
    if (!this.accountInfo) {
      return { ok: false, error: { tag: AccountCreateErrorTag.Internal, error: 'invalid_state' } };
    }
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

  async createAccount(authentication: AccountAuthMethodStrategy): Promise<Result<null, AccountCreateError>> {
    if (!this.accountInfo || !this.code) {
      return { ok: false, error: { tag: AccountCreateErrorTag.Internal, error: 'invalid_state' } };
    }
    return await libparsec.accountCreate3Proceed(
      getClientConfig().configDir,
      this.accountInfo.server,
      this.code.join(''),
      {
        label: `${this.accountInfo.firstName} ${this.accountInfo.lastName}`,
        email: this.accountInfo.email,
      },
      authentication,
    );
  }

  async reset(): Promise<void> {
    this.accountInfo = undefined;
    this.code = undefined;
  }
}

export const ParsecAccountAccess = {
  usePasswordForLogin(email: string, password: string): AccountLoginStrategy {
    return {
      tag: AccountLoginStrategyTag.Password,
      email: email,
      password: password,
    };
  },

  useMasterSecretForLogin(masterSecret: Uint8Array): AccountLoginStrategy {
    return {
      tag: AccountLoginStrategyTag.MasterSecret,
      masterSecret: masterSecret,
    };
  },

  usePasswordForCreate(password: string): AccountAuthMethodStrategy {
    return {
      tag: AccountAuthMethodStrategyTag.Password,
      password: password,
    };
  },

  useMasterSecretForCreate(masterSecret: Uint8Array): AccountAuthMethodStrategy {
    return {
      tag: AccountAuthMethodStrategyTag.MasterSecret,
      masterSecret: masterSecret,
    };
  },
};

class _ParsecAccount {
  handle: AccountHandle | undefined = undefined;
  skipped: boolean = false;
  serverAddr: ParsecAddr | undefined = undefined;

  async init(): Promise<void> {
    if (!Env.isAccountEnabled()) {
      return;
    }
    if (Env.isAccountAutoLoginEnabled()) {
      const TEST_PASSWORD = 'P@ssw0rd.';
      window.electronAPI.log('info', `Using Parsec Account auto-login, server is '${Env.getAccountServer()}'`);
      // Create test account
      const newAccountResult = await libparsec.testNewAccount(Env.getAccountServer());
      if (!newAccountResult.ok) {
        window.electronAPI.log(
          'error',
          `No auto-login possible, testNewAccount failed: ${newAccountResult.error.tag} (${newAccountResult.error.error})`,
        );
        return;
      }
      // Login to the test account
      const loginResult = await this.login(ParsecAccountAccess.useMasterSecretForLogin(newAccountResult.value[1]), Env.getAccountServer());
      if (!loginResult.ok) {
        window.electronAPI.log('error', `Failed to login: ${loginResult.error.tag} (${loginResult.error.error})`);
        return;
      }
      // Add a password authentication
      window.electronAPI.log('info', `Setting new password to test Parsec Account: '${TEST_PASSWORD}'`);
      const addAuthResult = await libparsec.accountCreateAuthMethod(loginResult.value, {
        tag: AccountAuthMethodStrategyTag.Password,
        password: TEST_PASSWORD,
      });
      if (!addAuthResult.ok) {
        window.electronAPI.log(
          'error',
          `Failed to add new password authentication: ${addAuthResult.error.tag} (${addAuthResult.error.error})`,
        );
      }
      return;
    }

    const startedAccounts = await libparsec.listStartedAccounts();
    if (startedAccounts.length > 0) {
      window.electronAPI.log('info', `${startedAccounts.length} account(s) already started, reusing if possible`);
      const infoResult = await libparsec.accountInfo(startedAccounts[0]);
      if (infoResult.ok) {
        this.handle = startedAccounts[0];
        this.serverAddr = infoResult.value.serverAddr;
        await this.registerAllDevices();
      } else {
        window.electronAPI.log('error', `One account started but failed to get info: ${infoResult.error.tag} (${infoResult.error.error})`);
      }
    }
  }

  getHandle(): AccountHandle | undefined {
    return this.handle;
  }

  getServer(): string | undefined {
    return this.serverAddr;
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

  async login(authentication: AccountLoginStrategy, server: string): Promise<Result<AccountHandle, AccountLoginError>> {
    const result = await libparsec.accountLogin(getClientConfig().configDir, server, authentication);
    if (!result.ok) {
      return result;
    }
    this.handle = result.value;
    this.serverAddr = server;
    await this.registerAllDevices();
    return result;
  }

  private async registerAllDevices(): Promise<void> {
    if (!this.handle) {
      return;
    }
    const listResult = await this.listRegistrationDevices();
    if (!listResult.ok) {
      window.electronAPI.log('error', `Failed to list registration devices: ${listResult.error.tag} (${listResult.error.error})`);
      return;
    }
    const availableDevices = await listAvailableDevices(false);

    for (const regDevice of listResult.value) {
      const existingDevice = availableDevices.find(
        (ad) =>
          ad.organizationId === regDevice.organizationId &&
          ad.userId === regDevice.userId &&
          (ad.ty.tag === AvailableDeviceTypeTag.AccountVault || ad.ty.tag === AvailableDeviceTypeTag.Keyring),
      );
      if (existingDevice !== undefined) {
        continue;
      }
      let saveStrategy!: DeviceSaveStrategy;
      if (isWeb()) {
        saveStrategy = constructSaveStrategy(PrimaryProtectionStrategy.useAccountVault(this.handle));
      } else {
        saveStrategy = constructSaveStrategy(PrimaryProtectionStrategy.useKeyring());
      }

      const regResult = await libparsec.accountRegisterNewDevice(
        this.handle,
        regDevice.organizationId,
        regDevice.userId,
        getAccountDefaultDeviceName(),
        saveStrategy,
      );
      if (!regResult.ok) {
        window.electronAPI.log('error', `Failed to register new device: ${regResult.error.tag} (${regResult.error.error})`);
      }
    }
  }

  async listInvitations(): Promise<Result<Array<AccountInvitation>, AccountListInvitationsError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountListInvitationsError>();
    }
    const result = await libparsec.accountListInvitations(this.handle);
    if (result.ok) {
      return {
        ok: true,
        value: result.value.map(([invitationAddr, organizationId, token, type]) => {
          return { organizationId: organizationId, token: token, type: type, addr: invitationAddr };
        }),
      };
    }
    return result;
  }

  async listRegistrationDevices(): Promise<Result<Array<RegistrationDevice>, AccountListRegistrationDevicesError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountListRegistrationDevicesError>();
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

  async registerNewDevice(registrationDevice: RegistrationDevice): Promise<Result<AvailableDevice, AccountRegisterNewDeviceError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountRegisterNewDeviceError>();
    }
    const availableDevices = await listAvailableDevices(false);
    const existingDevice = availableDevices.find(
      (ad) =>
        ad.organizationId === registrationDevice.organizationId &&
        ad.userId === registrationDevice.userId &&
        ad.ty.tag === AvailableDeviceTypeTag.AccountVault,
    );
    if (existingDevice !== undefined) {
      return { ok: true, value: existingDevice };
    }
    const regResult = await libparsec.accountRegisterNewDevice(
      this.handle,
      registrationDevice.organizationId,
      registrationDevice.userId,
      getAccountDefaultDeviceName(),
      constructSaveStrategy(PrimaryProtectionStrategy.useAccountVault(this.handle)),
    );
    return regResult;
  }

  async createRegistrationDevice(accessStrategy: DeviceAccessStrategy): Promise<Result<null, AccountCreateRegistrationDeviceError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountCreateRegistrationDeviceError>();
    }
    return await libparsec.accountCreateRegistrationDevice(this.handle, accessStrategy);
  }

  addressMatchesAccountServer(addr: string): boolean {
    if (!this.serverAddr) {
      return false;
    }
    try {
      const addrUrl = new URL(addr.replace('parsec3:', 'https:'));
      const accountUrl = new URL(this.serverAddr.replace('parsec3:', 'https:'));
      return addrUrl.host === accountUrl.host && addrUrl.port === accountUrl.port;
    } catch (e: any) {
      window.electronAPI.log('error', `Failed to compare device address with server address: ${e.toString()}`);
      return false;
    }
  }

  async isDeviceRegistered(device: AvailableDevice): Promise<Result<boolean, AccountListRegistrationDevicesError>> {
    if (!this.addressMatchesAccountServer(device.serverAddr)) {
      return { ok: false, error: { tag: AccountListRegistrationDevicesErrorTag.Internal, error: 'no-server-match' } };
    }
    const infoResult = await this.getInfo();
    if (!infoResult.ok || device.humanHandle.email !== infoResult.value.humanHandle.email) {
      return { ok: false, error: { tag: AccountListRegistrationDevicesErrorTag.Internal, error: 'no-email-match' } };
    }

    try {
      const result = await this.listRegistrationDevices();
      if (!result.ok) {
        return result;
      }
      return {
        ok: true,
        value: result.value.find((rd) => rd.organizationId === device.organizationId && rd.userId === device.userId) !== undefined,
      };
    } catch (e: any) {
      window.electronAPI.log('error', `Failed to compare device address with server address: ${e.toString()}`);
      return { ok: false, error: { tag: AccountListRegistrationDevicesErrorTag.Internal, error: e.toString() } };
    }
  }

  async getInfo(): Promise<Result<AccountInfo, AccountInfoError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountInfoError>();
    }
    return await libparsec.accountInfo(this.handle);
  }

  async logout(): Promise<Result<null, AccountLogoutError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountLogoutError>();
    }
    const result = await libparsec.accountLogout(this.handle);
    if (result.ok) {
      this.handle = undefined;
      this.serverAddr = undefined;
    }
    return result;
  }

  async requestAccountDeletion(): Promise<Result<null, AccountDeleteSendValidationEmailError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountDeleteSendValidationEmailError>();
    }
    return await libparsec.accountDelete1SendValidationEmail(this.handle);
  }

  async confirmAccountDeletion(code: Array<string>): Promise<Result<null, AccountDeleteProceedError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountDeleteProceedError>();
    }
    const result = await libparsec.accountDelete2Proceed(this.handle, code.join(''));
    if (result.ok) {
      await libparsec.accountLogout(this.handle);
      this.handle = undefined;
    }
    return result;
  }

  async recoveryRequest(email: string, server: string): Promise<Result<null, AccountRecoverSendValidationEmailError>> {
    return await libparsec.accountRecover1SendValidationEmail(getClientConfig().configDir, server, email);
  }

  async recoveryProceed(
    email: string,
    authentication: AccountAuthMethodStrategy,
    code: Array<string>,
    server: string,
  ): Promise<Result<null, AccountRecoverProceedError>> {
    return await libparsec.accountRecover2Proceed(getClientConfig().configDir, server, code.join(''), email, authentication);
  }

  async sendRecoveryEmail(email: string, server: string): Promise<Result<null, AccountRecoverSendValidationEmailError>> {
    return await libparsec.accountRecover1SendValidationEmail(getClientConfig().configDir, server, email);
  }

  async listAuthenticationMethod(): Promise<Result<Array<AuthMethodInfo>, AccountListAuthMethodsError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountListAuthMethodsError>();
    }
    const accountInfoResult = await libparsec.accountInfo(this.handle);
    const listResult = await libparsec.accountListAuthMethods(this.handle);
    if (listResult.ok) {
      listResult.value = listResult.value.map((method) => {
        (method as AuthMethodInfo).current = accountInfoResult.ok && accountInfoResult.value.inUseAuthMethod === method.authMethodId;
        (method as AuthMethodInfo).createdOn = DateTime.fromSeconds(method.createdOn as any as number);
        return method;
      });
    }
    return listResult as Result<Array<AuthMethodInfo>, AccountListAuthMethodsError>;
  }

  async updatePassword(password: string): Promise<Result<null, AccountCreateAuthMethodError>> {
    if (!this.handle) {
      return generateNoHandleError<AccountCreateAuthMethodError>();
    }
    return libparsec.accountCreateAuthMethod(this.handle, { tag: AccountAuthMethodStrategyTag.Password, password: password });
  }
}

const ParsecAccount = new _ParsecAccount();

export { AccountCreationStepper, ParsecAccount };
