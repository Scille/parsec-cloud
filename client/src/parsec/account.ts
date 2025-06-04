// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { wait } from '@/parsec/internals';
import {
  AccountError,
  AccountErrorTag,
  AccountHandle,
  DeviceAccessStrategy,
  DeviceAccessStrategyTag,
  DeviceSaveStrategy,
  Result,
} from '@/parsec/types';
import { Env } from '@/services/environment';

class AccountCreationStepper {
  stepHandle?: number;
  server?: string;
  email?: string;
  public firstName?: string;
  public lastName?: string;

  async start(firstName: string, lastName: string, email: string, server: string): Promise<Result<number, AccountError>> {
    let result!: Result<number, AccountError>;
    if (Env.isAccountMocked()) {
      await wait(1500);
      this.stepHandle = 1;
      result = { ok: true, value: 1 };
    } else {
      throw new Error('NOT IMPLEMENTED');
      /*
      const result = await parsecAccountStartProcess(firstName, lastName, email, server);
      if (result.ok) {
        stepHandle = result.value
      } else {
        return result;
      }
      */
    }
    this.server = server;
    this.email = email;
    this.firstName = firstName;
    this.lastName = lastName;
    return result;
  }

  async resendCode(): Promise<Result<null, AccountError>> {
    if (Env.isAccountMocked()) {
      await wait(1500);
      return { ok: true, value: null };
    } else {
      throw new Error('NOT IMPLEMENTED');
      /*
      return await parsecAccountResendEmail();
      */
    }
  }

  async validateEmail(code: Array<string>): Promise<Result<number, AccountError>> {
    if (this.stepHandle !== 1) {
      throw new Error('INVALID STEP');
    }
    if (Env.isAccountMocked()) {
      await wait(1500);
      if (!code.every((v, i) => v === ['1', '2', '3', '4', '5', '6'][i])) {
        return { ok: false, error: { tag: AccountErrorTag.InvalidCode, error: 'INVALID CODE' } };
      }
      this.stepHandle = 2;
      return { ok: true, value: 2 };
    } else {
      throw new Error('NOT IMPLEMENTED');
      /*
      const result = await parsecAccountValidateEmail(code);
      if (result.ok) {
        stepHandle = result.value;
      }
      return result;
      */
    }
  }

  async createAccount(_authentication: DeviceSaveStrategy): Promise<Result<null, AccountError>> {
    if (this.stepHandle !== 2) {
      throw new Error('INVALID STEP');
    }
    if (Env.isAccountMocked()) {
      await wait(1500);
      return { ok: true, value: null };
    } else {
      throw new Error('NOT IMPLEMENTED');
      /*
      return await parsecAccountCreate(authentication);
      */
    }
  }

  async reset(): Promise<void> {
    this.stepHandle = undefined;
  }
}

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

  async login(email: string, authentication: DeviceAccessStrategy, _server: string): Promise<Result<AccountHandle, AccountError>> {
    if (this.skipped) {
      window.electronAPI.log('warn', 'Parsec Auth marked as skipped but login() called');
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      if (email === 'a@b.c' && authentication.tag === DeviceAccessStrategyTag.Password && authentication.password === 'BigP@ssw0rd.') {
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
