<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Hexstring format

## Overview

This RFC discusses how we should format a hexstring present in our codebase.

## Background & Motivation

We currently don't have a rule about how we should format hexstring present in our codebase.
This result in PRs that change how the hexstring are formatted:

- [PR-7086 - libparsec/types/tests/certif.rs](https://github.com/Scille/parsec-cloud/pull/7086/files#diff-c8f5bf5725a2c3f14476799fd214bd520c20c526d8bb36fb1bd5c63cb52b5c37)
- [PR-7085 - libparsec/platform_device_loader/tests/list.rs](https://github.com/Scille/parsec-cloud/pull/7085/files#diff-0eb59bce4fc0b8465b793bb89ffde34e21f7b8206d55aff827da0ed499af1b1e)

> I've picked those PRs as example because they're the most recent that I can remember that change the format, nothing more.

We currently have multiple format for long hexstring:

- 54 wide:

  <https://github.com/Scille/parsec-cloud/blob/a01b2787fad539158f0271d93d1df7f999358cb9/libparsec/crates/protocol/tests/misc.rs#L14-L16>

- 64 wide:

  <https://github.com/Scille/parsec-cloud/blob/a01b2787fad539158f0271d93d1df7f999358cb9/libparsec/crates/protocol/tests/authenticated_cmds/v4/events_listen.rs#L101-L105>

  <https://github.com/Scille/parsec-cloud/blob/a01b2787fad539158f0271d93d1df7f999358cb9/libparsec/crates/protocol/tests/authenticated_cmds/v4/invite_1_greeter_wait_peer.rs#L20-L25>

- 74 wide:

  <https://github.com/Scille/parsec-cloud/blob/a01b2787fad539158f0271d93d1df7f999358cb9/libparsec/crates/protocol/tests/authenticated_cmds/v4/device_create.rs#L19-L23>

  <https://github.com/Scille/parsec-cloud/blob/a01b2787fad539158f0271d93d1df7f999358cb9/libparsec/crates/protocol/tests/authenticated_cmds/v4/device_create.rs#L120-L126>

- 78 wide:

  <https://github.com/Scille/parsec-cloud/blob/a01b2787fad539158f0271d93d1df7f999358cb9/libparsec/crates/platform_device_loader/tests/load.rs#L73-L81>

- 80 wide:

  <https://github.com/Scille/parsec-cloud/blob/a01b2787fad539158f0271d93d1df7f999358cb9/libparsec/crates/protocol/tests/anonymous_cmds/v4/organization_bootstrap.rs#L49-L57>

  <https://github.com/Scille/parsec-cloud/blob/a01b2787fad539158f0271d93d1df7f999358cb9/libparsec/crates/protocol/tests/anonymous_cmds/v4/organization_bootstrap.rs#L145-L153>
- 94 wide:

  <https://github.com/Scille/parsec-cloud/blob/a01b2787fad539158f0271d93d1df7f999358cb9/libparsec/crates/crypto/tests/sign.rs#L56-L59>

- greater than 400 wide:

  <https://github.com/Scille/parsec-cloud/blob/a01b2787fad539158f0271d93d1df7f999358cb9/libparsec/crates/platform_device_loader/tests/list.rs#L172-L173>

  <https://github.com/Scille/parsec-cloud/blob/a01b2787fad539158f0271d93d1df7f999358cb9/libparsec/crates/platform_device_loader/tests/list.rs#L144-L145>

## Goals and Non-Goals

The goal is to define how we should format hexstring that span multiple lines.
More precisely how wide should be each lines.

```rust
hex!(
  "68756d616e5f68d...af6db2416c6963657920"
  "74ac68756d616ed...ac652e636f6dae426f62"
//^ -- how wide should be these lines -- ^
  "abcdef"
)
```

Hexstring that are contained on a single line are out-of-scope.

That could be resumed in a single question:

> I have a multiple lines hexstring, how wide should be each lines ?

## Proposal

By doing this RFC, I've looked at the code and the more common width used seems to be $74$ and $64$ where the former seems to be more common.

~~Given that, I would propose that new or edited hexstring to be formatted using a $74$ wide line value.~~

After a discussion with the team (the 2024-04-17), we choose the value $70$ (It's the default value used by `python textwrap.wrap`).

That translate to when generating the string from:

- `fold`:

  ```shell
  fold -w 70 <file>
  ```

- `xxd`:

  ```shell
  xxd -c 35 -ps <file> # 70 / 2 = 35, a byte take 2 char to be represented.
  ```

- `python`:

  ```python
  import textwrap
  print('\n'.join(textwrap.wrap('Text here')))
  ```

## Remarks & open questions

We need a value that balance how wide (number of columns) and how long (number of lines) a hexstring is.
