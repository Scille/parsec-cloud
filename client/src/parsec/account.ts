// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { wait } from '@/parsec/internals';
import { AccountError, AccountErrorTag, AccountHandle, Result } from '@/parsec/types';
import { Env } from '@/services/environment';

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

  async login(email: string, password: string, _server: string): Promise<Result<AccountHandle, AccountError>> {
    if (this.skipped) {
      window.electronAPI.log('warn', 'Parsec Auth marked as skipped but login() called');
    }
    if (Env.isAccountMocked()) {
      await wait(2000);
      if (email === 'a@b.c' && password === 'P@ssw0rd.') {
        this.handle = 1;
        return { ok: true, value: 1 };
      }
      return { ok: false, error: { tag: AccountErrorTag.InvalidPassword, error: 'Invalid password' } };
    } else {
      throw new Error('NOT IMPLEMENTED');
      // return await parsecAuthLogin(email, password, Env.getAuthServer());
    }
  }

  async logout(): Promise<void> {
    this.handle = undefined;
  }
}

const ParsecAccount = new _ParsecAccount();

export { ParsecAccount };
