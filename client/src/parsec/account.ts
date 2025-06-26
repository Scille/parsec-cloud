// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getClientConfig, wait } from '@/parsec/internals';
import {
  AccountAccess,
  AccountAccessStrategy,
  AccountCreateProceedError,
  AccountCreateProceedErrorTag,
  AccountError,
  AccountErrorTag,
  AccountHandle,
  AccountInfo,
  AccountSendEmailValidationTokenError,
  AccountSendEmailValidationTokenErrorTag,
  Result,
} from '@/parsec/types';
import { AccountCreateStepTag, libparsec } from '@/plugins/libparsec';
import { Env } from '@/services/environment';

class AccountCreationStepper {
  accountInfo?: {
    server: string;
    email: string;
    firstName: string;
    lastName: string;
  };

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
  ): Promise<Result<null, AccountSendEmailValidationTokenError>> {
    let result!: Result<null, AccountSendEmailValidationTokenError>;
    if (Env.isAccountMocked()) {
      await wait(1500);
      result = { ok: true, value: null };
    } else {
      console.log(server);
      result = await libparsec.accountCreateSendValidationEmail(email, getClientConfig().configDir, server);
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

  async resendCode(): Promise<Result<null, AccountSendEmailValidationTokenError>> {
    if (!this.accountInfo) {
      return { ok: false, error: { tag: AccountSendEmailValidationTokenErrorTag.Internal, error: 'invalid_state' } };
    }
    if (Env.isAccountMocked()) {
      await wait(1500);
      return { ok: true, value: null };
    } else {
      return await libparsec.accountCreateSendValidationEmail(this.accountInfo.email, getClientConfig().configDir, this.accountInfo.server);
    }
  }

  async validateCode(code: Array<string>): Promise<Result<null, AccountCreateProceedError>> {
    if (!this.accountInfo) {
      return { ok: false, error: { tag: AccountCreateProceedErrorTag.Internal, error: 'invalid_state' } };
    }
    if (Env.isAccountMocked()) {
      await wait(1500);
      if (code.join('') === 'ABCDEF') {
        return { ok: true, value: null };
      }
      return { ok: false, error: { tag: AccountCreateProceedErrorTag.InvalidValidationToken, error: 'invalid-token' } };
    } else {
      return await libparsec.accountCreateProceed(
        { tag: AccountCreateStepTag.CheckCode, code: code.join('') },
        this.accountInfo.email,
        getClientConfig().configDir,
        this.accountInfo.server,
      );
    }
  }

  async createAccount(authentication: AccountAccess): Promise<Result<null, AccountCreateProceedError>> {
    if (!this.accountInfo) {
      return { ok: false, error: { tag: AccountCreateProceedErrorTag.Internal, error: 'invalid_state' } };
    }
    if (authentication.strategy !== AccountAccessStrategy.Password) {
      return { ok: false, error: { tag: AccountCreateProceedErrorTag.Internal, error: 'invalid_authentication' } };
    }
    if (Env.isAccountMocked()) {
      await wait(1500);
      return { ok: true, value: null };
    } else {
      return await libparsec.accountCreateProceed(
        {
          tag: AccountCreateStepTag.Create,
          humanLabel: `${this.accountInfo.firstName} ${this.accountInfo.lastName}`,
          password: authentication.password,
        },
        this.accountInfo.email,
        getClientConfig().configDir,
        this.accountInfo.server,
      );
    }
  }

  async reset(): Promise<void> {
    this.accountInfo = undefined;
  }
}

export const ParsecAccountAccess = {
  usePassword(password: string): AccountAccess {
    return {
      strategy: AccountAccessStrategy.Password,
      password: password,
    };
  },
};

class _ParsecAccount {
  handle: AccountHandle | undefined = undefined;
  skipped: boolean = false;

  constructor() {
    if (Env.isAccountAutoLoginEnabled()) {
      this.login('a@b.c', ParsecAccountAccess.usePassword('BigP@ssw0rd.'), Env.getAccountServer());
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

  async login(email: string, authentication: AccountAccess, _server: string): Promise<Result<AccountHandle, AccountError>> {
    if (this.skipped) {
      window.electronAPI.log('warn', 'Parsec Auth marked as skipped but login() called');
    }
    if (Env.isAccountMocked()) {
      if (!Env.isAccountAutoLoginEnabled()) {
        await wait(2000);
      }
      if (authentication.strategy === AccountAccessStrategy.Password && authentication.password === 'BigP@ssw0rd.') {
        this.handle = 1;
        return { ok: true, value: 1 };
      }
      return { ok: false, error: { tag: AccountErrorTag.InvalidAuthentication, error: 'Invalid authentication' } };
    } else {
      throw new Error('NOT IMPLEMENTED');
      // return await parsecAccountLogin(email, password, _server);
    }
  }

  async getInfo(): Promise<Result<AccountInfo, AccountError>> {
    if (this.skipped) {
      window.electronAPI.log('warn', 'Parsec Auth marked as skipped but getInfo() called');
      return { ok: false, error: { tag: AccountErrorTag.Internal, error: 'marked as skipped' } };
    }
    if (!this.handle) {
      return { ok: false, error: { tag: AccountErrorTag.NotLoggedIn, error: 'not logged in' } };
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      return { ok: true, value: { email: 'gordon.freeman@blackmesa.nm', name: 'Gordon Freeman' } };
    } else {
      throw new Error('NOT IMPLEMENTED');
      // return await libparsec.accountGetInfo(handle);
    }
  }

  async logout(): Promise<void> {
    if (!this.handle) {
      return;
    }
    if (!Env.isAccountMocked()) {
      console.log('Log out');
      // await libparsec.accountLogOut(this.handle);
    }
    this.handle = undefined;
  }

  async requestAccountDeletion(): Promise<Result<null, AccountError>> {
    if (!this.handle) {
      return { ok: false, error: { tag: AccountErrorTag.NotLoggedIn, error: 'not-logged-in' } };
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      return { ok: true, value: null };
    } else {
      throw new Error('NOT IMPLEMENTED');
      // return await libparsec.requestAccountDeletion(this.handle);
    }
  }

  async confirmAccountDeletion(code: Array<string>): Promise<Result<null, AccountError>> {
    if (!this.handle) {
      return { ok: false, error: { tag: AccountErrorTag.NotLoggedIn, error: 'not-logged-in' } };
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      if (code.join('') !== 'ABCDEF') {
        return { ok: false, error: { tag: AccountErrorTag.InvalidCode, error: 'invalid-code' } };
      }
      this.handle = undefined;
      return { ok: true, value: null };
    } else {
      throw new Error('NOT IMPLEMENTED');
      // const result = await libparsec.confirmAccountDeletion(this.handle, code);
      // if (result.ok) {
      //   this.handle = undefined;
      // }
      // return result;
    }
  }
}

const ParsecAccount = new _ParsecAccount();

export { AccountCreationStepper, ParsecAccount };
