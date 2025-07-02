// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getClientConfig, wait } from '@/parsec/internals';
import {
  AccountAccess,
  AccountAccessStrategy,
  AccountCreateError,
  AccountCreateErrorTag,
  AccountCreateSendValidationEmailError,
  AccountCreateSendValidationEmailErrorTag,
  AccountGetHumanHandleErrorTag,
  AccountHandle,
  AccountLoginWithMasterSecretError,
  AccountLoginWithPasswordError,
  AccountLoginWithPasswordErrorTag,
  HumanHandle,
  Result,
} from '@/parsec/types';
import { AccountGetHumanHandleError, libparsec } from '@/plugins/libparsec';
import { Env } from '@/services/environment';

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

  async getInfo(): Promise<Result<HumanHandle, AccountGetHumanHandleError>> {
    if (!this.handle) {
      return { ok: false, error: { tag: AccountGetHumanHandleErrorTag.Internal, error: 'not logged in' } };
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      return { ok: true, value: { email: 'gordon.freeman@blackmesa.nm', label: 'Gordon Freeman' } };
    } else {
      return await libparsec.accountGetHumanHandle(this.handle);
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

  // async requestAccountDeletion(): Promise<Result<null, AccountError>> {
  //   if (!this.handle) {
  //     return { ok: false, error: { tag: AccountErrorTag.NotLoggedIn, error: 'not-logged-in' } };
  //   }
  //   if (Env.isAccountMocked()) {
  //     await wait(2000);
  //     return { ok: true, value: null };
  //   } else {
  //     throw new Error('NOT IMPLEMENTED');
  //     // return await libparsec.requestAccountDeletion(this.handle);
  //   }
  // }

  // async confirmAccountDeletion(code: Array<string>): Promise<Result<null, AccountError>> {
  //   if (!this.handle) {
  //     return { ok: false, error: { tag: AccountErrorTag.NotLoggedIn, error: 'not-logged-in' } };
  //   }
  //   if (Env.isAccountMocked()) {
  //     await wait(2000);
  //     if (code.join('') !== 'ABCDEF') {
  //       return { ok: false, error: { tag: AccountErrorTag.InvalidCode, error: 'invalid-code' } };
  //     }
  //     this.handle = undefined;
  //     return { ok: true, value: null };
  //   } else {
  //     throw new Error('NOT IMPLEMENTED');
  //     // const result = await libparsec.confirmAccountDeletion(this.handle, code);
  //     // if (result.ok) {
  //     //   this.handle = undefined;
  //     // }
  //     // return result;
  //   }
  // }
}

const ParsecAccount = new _ParsecAccount();

export { AccountCreationStepper, ParsecAccount };
