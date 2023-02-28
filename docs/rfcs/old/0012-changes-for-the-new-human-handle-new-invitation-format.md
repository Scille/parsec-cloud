# Changes for the new human handle + new invitation format

From [ISSUE-1078](https://github.com/Scille/parsec-cloud/issues/1078)

## Compatibility considerations

Currently API is in version 1.1, the changes discussed here are compatible for some part
(mainly adding an optional `human_handle` field) and incompatible for the rest.

So we will release an API version 1.2 and another version 2.0.

Given we already have a server running with a base of clients, here are our requirements:

- New client release MUST support API 2.0
- New server release MUST support API 1.2 and 2.0. This way existing clients with
  API 1.1 will use connect with server API 1.2 and new clients will use API 2.0
- New client release COULD support API 1.2 however it's not really important given
  there is currently no other Parsec server instance apart from our SAAS service.
  The only real reason to do that would be to allow invitation between existing client
  and new client (though it means more work in the GUI so not sure if it's worth it...)
- Once the Parsec SAAS service no longer receive connection with API 1.2 a
  new server release COULD drop API 1.2

Considering the Parsec application version policy, we could keep v1.xx.yy for
now and switch to v2.xx.yy when API 1.1 is dropped.

**Q1: Should we support invitation between current and new release Client ?**

## Human handle

See [RFC-0009](0009-use-email-as-main-human-identifier.md)

*tl,dr:* add a new human_handle field to the user certificate that provide email&label
(e.g. `John Doe <john.doe@example.com>`).

## Shorten invitation URL

### Current url format

> parsec://parsec.example.com/my_org?action=claim_user&user_id=John&rvk=P25GRG3XPSZKBEKXYQFBOLERWQNEDY3AO43MVNZCLPXPKN63JRYQssss

RVK, `user_id` and `organization_id` are no longer needed in the invitation
URL (can be retrieved once connected to the backend).

### New url format

> parsec://parsec.example.com/my_org?action=claim_user&token=abcd9064

This stick to the current url format as much as possible (organization id and
action fields also help for readability, useful for debugging & user support).

- We may want to add something to inform this is a new format (for instance
`action=claim_user_v2`), but not sure what...
- `user_id` field is not really useful given it's only a technical id now
- we shouldn't add the email as field in the url given 1) it makes the url longer
  2) email are complex to encode 3) this information can be retrieved from the
  backend if we need to display it to the user

**Q2: should we add a version field in the url ?**

## Anonymous login abuse

Currently there is nothing to prevent anybody from doing an anonymous connection to the backend.
Of course anonymous connection allows very few things so it should be fine *in theory*.
However it's always better to avoid this kind of behaviour (attacker could try to slow
the service with many idle connections, internals are more exposed etc.)

So it would be better to require the organization/token informations during handshake.
This would allow the backend to close the connection right away in case of bad
organization/token.
On top of that the couple organization/token should be considered random enough to prevent
from brute force attack (even considering an attacker knowing a valid organization name,
a 8 characters token represent 4 billions possibilities, of which only a couple are
valid invitation at a given time).

Note the backend already enforce a 3 seconds timeout for the client to initiate
the handshake (this has initially be added to prevent stall when both client and
server think it's up to the other peer to speak first, typically occurring when
messing with parsec-over-ssl).

### Bootstrap organization handshake, answer frame

```json
{
    "handshake": "answer",
    "type": "anonymous",
    "client_api_version": [2, 0],
    "organization_id": "my_org",
    "operation": "bootstrap_organization",
    "token": "abcd9064"
}
```

Current bootstrap organization URL format already contains organization_id
and token (e.g. `parsec://parsec.example.com/my_org?action=bootstrap_organization&token=1234ABCD`)

### Claim user handshake, answer frame

```json
{
    "handshake": "answer",
    "type": "anonymous",
    "client_api_version": [2, 0],
    "organization_id": "my_org",
    "operation": "claim_user",
    "token": "abcd9064"
}
```

### Claim device handshake, answer frame

```json
{
    "handshake": "answer",
    "type": "anonymous",
    "client_api_version": [2, 0],
    "organization_id": "my_org",
    "operation": "claim_device",
    "token": "abcd9064"
}
```

## New organization bootstrap

Given token is provided as part of the handshake, no need to provide it
in the organization_bootstrap command.
As well, given operation is specified during the handshake, anonymous connection
can only do a single operation.
Backend could close connection as soon as the organization_bootstrap command succeed
or if the client try to do any unauthorized command.

```python
#  Backend to client
{
    "handshake": "challenge",
    "challenge": "123ABC",
    "supported_api_versions": [[2, 0], [1, 2]]
}

# Client to backend
{
    "handshake": "answer",
    "type": "anonymous",
    "client_api_version": [2, 0],
    "organization_id": "my_org",
    "operation": "bootstrap_organization",
    "token": "abcd9064"
}

# Backend to client
{
    "handshake": "result",
    "result": "ok"
}

# Client to backend
{
    "cmd": "organization_bootstrap",
    "user_certificate": "...",
    "device_certificate": "...",
    "root_verify_key": "..."
}

# Backend to client
{
    "status": "ok"
}
```

**Q3: There is `bootstarp_organization` (in url's action field and in
handshake's operation field) and `organization_bootstrap` (command name)...
should we stick to a single format ?**

## New ~user~ human invitation

Discussed in [rfc-0008](0008-smoothening-the-invite-process.md)

### Human vs user

Currently we invite user (relying on `user_id`), however we will switch to invite
human (relying on email which is the technical part of the `human_handle`).

User without human_handle are still allowed but considered as system user (useful
for future work on SGX), creating them is done by directly using `user_create`
command (if needed we could in the future provide a cli command to export system users).

Note we should still allow a non human to invite a human (required for backward
compatibility, and potentially interesting for SGX applications).

### API format

```python
# Client to backend
{
    "cmd": "invite_new",
    "type": "user",
    "email": "alice@example.com",
    "send_email": true
}

# Backend to client
{
    "result": "ok",
    "token": "abcd9064"
}
```

- invitation is really similar between device and client, the api is designed to
  share as much as possible (hence the `type` field)
- `send_email` option allow to prevent backend from sending an invitation mail
  to the invitee (useful for testing, maybe useful for advanced usecases)
- in case `send_email=false`, the `url` field returned can be used to display
  the url to the inviter.

### About the URL

Invitation url used to be considered as core-only (i.e. not part of the api
module given it was only used to be passed between clients and not to
communicate with the backend). This is no longer true given the backend should
be able to send a mail containing the url to the invitee.

So we should move the url parsing code into the api module (hence considering the
url format as part of the API).
This is a good idea anyway to pave the way for very different clients like the mobile version.

Note we could have the backend returns an `url` field instead of the `token` field.
However this has multiple cons:

- it makes parsing harder
- backend must always be configured with it own domain information (versus
  only needing to known it own domain in case we want to send emails with url)
- it's very common in test to use the token just after sending an invitation
- it's fairly easy to generate the url client side from the token and backend address

## New device invitation

This is more tricky than it seems...

In theory it should be the same than inviting human, but sending an email to
yourself seems strange. On the other hand not sending an email means we
have to stick to url passing between devices which is also not great...

The less worst solution I see is to have a unified system between human and device invite:

- Inviter go to the invitation tab in the GUI, there is two buttons "New user" and "New device"
- Clicking on "New user" open a field to fill the email and a "send" button to proceed
- Clicking on the "New device" directly proceed given the user's email is already known
- Proceed means a human_user_invite/device_invite command is send to the backend.
  The client GUI then displays "An invitation mail has been sent to xxx@example.com"
- The list of pending invitations is updated with the last one first.
  There is two buttons on each invitation line: "show invitation url" (in grey)
  and "proceed" (in primary color)

What this means is device invitation is only possible for user having a human_handle
(at least if we send email). I think this is not really an issue (if needed
we could still provide a cli command relying on `device_create` to export new
devices without going through the invitation process).
I think for sake of clarity&simplicity we should consider invitation as a
process always involving human, hence enforcing the need for a human_handle
even when not sending mail.

```python
# Client to backend
{
    "cmd": "invite_new",
    "type": "device",
    "send_email": true
}

# Backend to client
{
    "result": "ok",
    "token": "abcd9064"
}
```

## Cancel Invitation

Regarding human invitation, I don't think we should have a time limit but only
allow the invitation creator to cancel the invitation whenever he wants (the
GUI should display a "cancel" button in the invitations list).

```python
# Client to backend
{
    "cmd": "invite_delete",
    "reason": "cancelled",
    "token": "abcd9064"
}

# Backend to client
{
    "result": "ok",
}
```

Regarding devices, multiple devices invitation doesn't seems really useful,
but it's just simpler to allow it in order to mimic human invitation behaviour.

## List pending invitations

There is no need to list past invitations given there can be determined by querying
user manifests. This shave of a bit of complexity ;-)

To be listed, the invitations has to be stored somewhere:
1 - in the user manifest
2 - in the backend
3 - in the backend as signed data

Solution 1) is the most secure, but it increase complexity (atomicity with the
backend, sync concern with poor connection etc.).

Solution 2) is the most simple but it means trusting the backend with list of invitations.
The downside is of course the backend can lie about this (showing us invitations
we didn't create in order to trick us into accepting a new illegitimate user).

Solution 3) is a trade-of, but I'm not sure it worth the added complexity compared
to solution 2).
Backend can still recycle an already existing invitation, hide the user/device
certificate corresponding to the invited person etc.

In theory this is no big deal because the real security lays in the token exchange
step that is done during claim, but letting the backend the possibility to cheat
on the invitation list doesn't seem right to me...

**Q4: should we use solution 2) or 3) ?**

### API format (using solution 2)

```python
# Client to backend
{
    "cmd": "invite_list"
}

# Backend to client
{
    "result": "ok",
    "invitations": [
        {
            "token": "abc123",
            "type": "user",
            "status": "idle",
            "email": "alice@exemple.com",
            "created": "2021-04-01T12:59:59Z"
        },
        {
            "token": "def456",
            "type": "device",
            "status": "ready",
            "created": "2020-04-01T12:59:59Z"
        }
    ]
}
```

- `status` field is here to show if the invitee client is ready to
proceed to claim (i.e. the invitee has an anonymous connection to the backend
and has started the claim operation).
- `token` field allow to retrieve and display to the user the invitation url
- `created` is not strictly mandatory, but seems easy enough to provide and
  interesting to order the invitations in GUI and provide feedback to the user

### Poll vs pull strategy on claim readiness

Instead of the `status` field (or maybe on top of) we could have a
`invite_claim_ready` event to notify the inviter that the invitee is ready.

However I don't think it's a good thing given it means the inviter would get
a GUI notification out of the blue (the invitee has just click on the invitation
link, he hasn't had time to contact the inviter and maybe isn't ready to do that yet).
This is event worth considering the invitee could go back and forth multiple times,
hence sending multiple events to the inviter...

On the other hand the `status` field in the invitation list allows to easily
show the invitations ready first and provide a big "proceed" button that makes
things obvious.

Another more legitimate use of the `invite_status_changed` event would be know
when to refresh the invitations list.

```python
{
    "status": "ok",
    "event": "invite_status_changed"
}
```

In such usecase we keep the event as simple as possible and notify both ready
and idle status switches.

## New claim system

### Overview

Claim is composed of three steps:
1 - peers discovery
2 - secure channel establishment
3 - actual claim operation

Step 1 consist of waiting for inviter and invitee to be both online and ready
to communicate.

Step 2 is discussed in issue [RFC-0010](0010-user-device-invitation-protocol-v2.md), this is basically a Diffie-Hellman
secured by short authentication strings.

Only step 3 differs between user and device claim.

### 0 - Foreplays

#### Create the invitation

```python
# Inviter command
{
    "cmd": "invite_new",
    "type": "user",  # or `device`
    "email": "alice@example.com",  # Only for user
    "send_email": true
}

{
    "result": "ok",
    "token": "abcd9064"
}
```

URL: `parsec://parsec.example.com/my_org?action=claim_user&token=1234ABCD`

#### Get back invitation informations

```python
# Invitee command
{
    "cmd": "invite_info",
}

{
    "result": "ok",
    "type": "user",  # or `device`
    "email": "alice@example.com",  # Only for user
    "inviter_human_email": "john.doe@example.com",
    "inviter_human_label": "John Doe"
}
```

### 1 - Public key exchange

```python
# Invitee command
{
    "cmd": "invite_1_invitee_wait_peer",
    "invitee_public_key": "..."
}
# Inviter command, can be send before invitee's command
{
    "cmd": "invite_1_inviter_wait_peer",
    "token": "abcd9064",
    "inviter_public_key": "..."
}

# Backend waits for both command
{
    "result": "ok",
    "inviter_public_key": "..."
}
{
    "result": "ok",
    "invitee_public_key": "..."
}
```

### 2 - hash&nonce exchange

```python
# Invitee command
{
    "cmd": "invite_2a_invitee_send_hashed_nonce",
    "invitee_hashed_nonce": "H(zyx987)"
}

# Inviter command
{
    "cmd": "invite_2a_inviter_get_hashed_nonce",
    "token": "abcd9064",
}
{
    "result": "ok",
    "invitee_hashed_nonce": "H(zyx987)"
}
# Inviter command
{
    "cmd": "invite_2b_inviter_send_nonce",
    "token": "abcd9064",
    "inviter_nonce": "wvu654"
}

# Invitee answer
{
    "result": "ok",
    "inviter_nonce": "wvu654"
}
# Invitee command
{
    "cmd": "invite_2b_invitee_send_nonce",
    "invitee_nonce": "zyx987"
}
{
    "result": "ok"
}

# Inviter answer
{
    "result": "ok",
    "invitee_nonce": "zyx987"
}
```

### 3 - SAS generation and human verification

```python
# Inviter command
{
    "cmd": "invite_3a_inviter_wait_peer_trust",
    "token": "abcd9064",
}

# Out-of-bound inviter to invitee SAS passing and verification

# Invitee command
{
    "cmd": "invite_3a_invitee_signify_trust",
}

# Inviter answer
{
    "result": "ok",
}
# Invitee answer
{
    "result": "ok",
}

# Invitee command
{
    "cmd": "invite_3b_invitee_wait_peer_trust",
}

# Out-of-bound invitee to inviter SAS passing and verification

# Inviter command
{
    "cmd": "invite_3b_inviter_signify_trust",
    "token": "abcd9064",
}

# Invitee answer
{
    "result": "ok",
}
# Inviter answer
{
    "result": "ok",
}
```

### 4 - Actual user/device creation

```python
# Invitee command
{
    "cmd": "invite_4_invitee_communicate",
    "payload": {  # Encrypted by shared key
        "verify_key": "...",
        # Only for user claim
        "public_key": "...",
        "device_id": "john-doe-ed20@desktop",
        "human_name": "John Doe",
        "human_email": "john.doe@example.com",
    }
}
# Inviter command
{
    "cmd": "invite_4_inviter_communicate",
    "token": "abcd9064",
    "payload": null
}

# Backend waits for both command

# Invitee answer
{
    "result": "ok",
    "payload": null
}
# Inviter answer
{
    "result": "ok",
    "payload": {  # Encrypted by shared key
        "verify_key": "...",
        # Only for user claim
        "public_key": "...",
        "device_id": "john-doe-ed20@desktop",
        "human_name": "John Doe",
        "human_email": "john.doe@example.com",
    }
}

# Invitee command
{
    "cmd": "invite_4_invitee_communicate",
    "payload": null
}

# Inviter user is asked for confirmation here
# Inviter does a user_create/device_create command

# Inviter command
{
    "cmd": "invite_4_inviter_communicate",
    "token": "abcd9064",
    "payload": {  # Encrypted by shared key
        "root_verify_key": "...",
        "device_certificate": "...",
        "user_certificate": "...",  # Only for user claim
        "private_key": "...",  # Only for device claim
    }
}

# Invitee answer
{
    "result": "ok",
    "payload": {  # Encrypted by shared key
        "root_verify_key": "...",
        "device_certificate": "...",
        "user_certificate": "...",  # Only for user claim
        "private_key": "...",  # Only for device claim
    }
}
# Inviter answer
{
    "result": "ok",
    "payload": null
}
```

### 5 - Finish/cancellation

Mark the invitation finished

```python
# Inviter command
{
    "cmd": "invite_delete",
    "token": "abcd9064",
    "reason": "finished"
}
{
    "result": "ok",
}
```

Other reasons:

- `cancelled`: manually cancelled by the user outside of the claim process
- `rotten`: something wrong occurred during the claim process

## Final word

The invite API is composed of 16 routes:

| command                             | inviter only | invitee only |
| ----------------------------------- | ------------ | ------------ |
| invite_new                          | yes          |              |
| invite_delete                       | yes          |              |
| invite_list                         | yes          |              |
| invite_info                         |              | yes          |
| invite_1_invitee_wait_peer          |              | yes          |
| invite_1_inviter_wait_peer          | yes          |              |
| invite_2a_invitee_send_hashed_nonce |              | yes          |
| invite_2a_inviter_get_hashed_nonce  | yes          |              |
| invite_2b_invitee_send_nonce        |              | yes          |
| invite_2b_inviter_send_nonce        | yes          |              |
| invite_3a_invitee_signify_trust     |              | yes          |
| invite_3a_inviter_wait_peer_trust   | yes          |              |
| invite_3b_invitee_wait_peer_trust   |              | yes          |
| invite_3b_inviter_signify_trust     | yes          |              |
| invite_4_invitee_communicate        |              | yes          |
| invite_4_inviter_communicate        | yes          |              |
