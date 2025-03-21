// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { wait } from '@/parsec/internals';
import { AuthError, AuthErrorTag, AuthHandle, Result } from '@/parsec/types';
import { Env } from '@/services/environment';

class _ParsecAuth {
  handle: AuthHandle | undefined = undefined;
  skipped: boolean = false;

  getHandle(): AuthHandle | undefined {
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

  async login(email: string, password: string): Promise<Result<AuthHandle, AuthError>> {
    if (this.skipped) {
      window.electronAPI.log('warn', 'Parsec Auth marked as skipped but login() called');
    }
    if (Env.isAuthMocked()) {
      await wait(2000);
      if (email === 'a@b.c' && password === 'P@ssw0rd.') {
        this.handle = 1;
        return { ok: true, value: 1 };
      }
      return { ok: false, error: { tag: AuthErrorTag.InvalidPassword, error: 'Invalid password' } };
    } else {
      throw new Error('NOT IMPLEMENTED');
      // return await parsecAuthLogin(email, password, Env.getAuthServer());
    }
  }

  async logout(): Promise<void> {
    this.handle = undefined;
  }
}

const ParsecAuth = new _ParsecAuth();

export { ParsecAuth };
