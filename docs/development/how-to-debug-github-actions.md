<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# GitHub Action Tips and Tricks

## How to connect remotely to GitHub action for debugging

To debug a GitHub Action's Job, you need to preemptively configure the said job with one of the following methods:

- [With Upterm](#via-upterm)
- [With Ngrok](#via-ngrok)

Either way, once a method is applied, it's just a matter of the job running and then connecting via SSH using the provided information displayed by one of the actions.

### Via Upterm

The action [Debugging with SSH] allows connecting to the GitHub VM via [Upterm].
This action is simple to use as you don't have to provide any credentials.
It will configure an SSH server that is populated with your SSH public key.

> [!NOTE]
> The action provide a remote session that use [`tmux`].

If you want to use this action, add the following snippet under `jobs.<name>.steps`:

```yaml
- name: Setup upterm session
  uses: lhotari/action-upterm@v1
  with:
    limit-access-to-actor: true
    wait-timeout: 5 # Wait 5 min before passing to the next step
```

The source code of the action can be found at [`lhotari/action-upterm`]

[Debugging with SSH]: https://github.com/marketplace/actions/debugging-with-ssh
[Upterm]: https://upterm.dev/
[`tmux`]: https://github.com/tmux/tmux
[`lhotari/action-upterm`]: https://github.com/lhotari/action-upterm

### Via Ngrok

If you'd prefer to use [Ngrok] instead of [Upterm], you can use the action [Debug via SSH].

This action is not as easy to use as the [upterm](#via-upterm) one as you'll need to have a Ngrok account to obtain a valid token.

If you want to use the Ngrok action, you need to add the following lines under `jobs.<name>.steps`:

```yaml
- name: Start SSH session
  uses: luchihoratiu/debug-via-ssh@main
  with:
    NGROK_AUTH_TOKEN: _ngrok_token_here_
    SSH_PASS: _ssh_password_to_connect_
    NGROK_REGION: eu
```

> [!NOTE]
> We pass the token and password as plain text because we (the person debugging) do not have the required permission to set/add repository secrets (require to be a repository administrator).

The source code of the action can be found at [`luchihoratiu/debug-via-ssh`]

[Ngrok]: https://ngrok.com/
[Debug via SSH]: https://github.com/marketplace/actions/debug-via-ssh
[`luchihoratiu/debug-via-ssh`]: https://github.com/luchihoratiu/debug-via-ssh
