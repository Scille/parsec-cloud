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
  AccountSendEmailValidationTokenError,
  AccountSendEmailValidationTokenErrorTag,
  Result,
} from '@/parsec/types';
import { libparsec } from '@/plugins/libparsec';
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
      result = await libparsec.accountSendEmailValidationToken(email, getClientConfig().configDir, server);
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
      return await libparsec.accountSendEmailValidationToken(this.accountInfo.email, getClientConfig().configDir, this.accountInfo.server);
    }
  }

  async createAccount(code: Array<string>, authentication: AccountAccess): Promise<Result<null, AccountCreateProceedError>> {
    if (!this.accountInfo) {
      return { ok: false, error: { tag: AccountCreateProceedErrorTag.Internal, error: 'invalid_state' } };
    }
    if (authentication.strategy !== AccountAccessStrategy.Password) {
      return { ok: false, error: { tag: AccountCreateProceedErrorTag.Internal, error: 'invalid_authentication' } };
    }
    if (code.join('') !== 'ABCDEF') {
      return { ok: false, error: { tag: AccountCreateProceedErrorTag.InvalidValidationToken, error: 'invalid_token' } };
    }
    if (Env.isAccountMocked()) {
      await wait(1500);
      return { ok: true, value: null };
    } else {
      return await libparsec.accountCreateProceed(
        `${this.accountInfo.firstName} ${this.accountInfo.lastName}`,
        code.join(''),
        getClientConfig().configDir,
        this.accountInfo.server,
        authentication.password,
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
      await wait(2000);
      if (email === 'a@b.c' && authentication.strategy === AccountAccessStrategy.Password && authentication.password === 'BigP@ssw0rd.') {
        this.handle = 1;
        return { ok: true, value: 1 };
      }
      return { ok: false, error: { tag: AccountErrorTag.InvalidAuthentication, error: 'Invalid authentication' } };
    } else {
      throw new Error('NOT IMPLEMENTED');
      // return await parsecAccountLogin(email, password, _server);
    }
  }

  async logout(): Promise<void> {
    this.handle = undefined;
  }
}

const ParsecAccount = new _ParsecAccount();

export { AccountCreationStepper, ParsecAccount };
