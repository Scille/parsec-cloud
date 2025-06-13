<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
<!-- cspell: words 4OIJ-VLMK-EJOM-H77B-WT4N-XFVS-IM -->

# Simple User Facing Token

## Overview

How to ease the user input for entering a token

## Background & Motivation

When designing the API for Parsec Account more specifically for the account creation and deletion confirmation steps, we require the user to provide a token.
Currently, those tokens consist of 16 bytes randomly generated

If it where provided to the user in [base32] encoding, that represent 32 character:

```shell
$ openssl rand 16 | base32 | tr -d = | sed 's/\(.\{4,4\}\)/\1 /g' | xargs echo | tr ' ' '-'
4OIJ-VLMK-EJOM-H77B-WT4N-XFVS-IM
```

> We choose [base32] because it contains only upper letter and digit with $1$ and $0$ removed
> to prevent confusion with `I` and `O`.
> It's easy on the client side to convert lower to upper letter and $1$ to `I` and $0$ to `O`.

[base32]: https://en.wikipedia.org/wiki/Base32

## Goals and Non-Goals

The goal is to simply the user input of the tokens to confirm the account creation/deletion.

## Design

Instead of using plain token for those operations, we instead use a text code:

The server would generate those codes using the [base32] alphabet, their length would be configurable via a settings (cli option, env var).
Those codes are then sent via email (the current communication channel).

That would require minor modification on the protocol for account creation/deletion as it would take a `AccountCreationCode`/`AccountDeletionCode` of arbitrary length (remember the length is server configurable).
Nevertheless, we should not allow using code smaller than 6 chars (so the client need to check for that).

For UI configuration, we could add a new anonymous command that return the length of those server generated codes.

## Alternatives Considered

- Initially, we discussed generating a token by deriving that code (the server sent that code, the client derivate it to get a token).

  That approach does not provide any benefit as the generated token would have been constraint by code (we consider that code as less entropy than the token).
  But does allow reusing the current protocol definition (i.e. the client provide a token)

- If we do not need the length of the code to be configurable, we could have something similar to the SAS code (but longer than 4 char)

  This change sightly the approach as it would be `libparsec` that generate that code, that simplify the implementation as we would not need to add another command for the code length for UI configuration.
  We could update to the configurable length approach later on.

## Security/Privacy/Compliance

- Smaller token means something easier to brute force, that's why the size of the token should depend on its lifetime
- In addition to limiting the lifetime of a token/code, we could also count the number of tries to create/delete an account.

  That number could be used to invalidate a token/code.

  That something important to consider as some [password strength testing tool](https://bitwarden.com/password-strength/) indicate that a code of 6 character is estimated to be brute forced in 2 min (3 hours for 8 char and centuries for 26 char (e.g. 16 bytes encoded in [base32])).

  > [!IMPORTANT]
  > Those estimations are biased as we cannot indicate the alphabet constraint, the reality are the code as less entropy, so the duration is exaggerated.
