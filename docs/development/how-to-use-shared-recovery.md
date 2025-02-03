<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
<!-- cspell:ignore xBCEREHKItJ0lPzzEuk-8q0N -->

# How to use shared recovery

## What is shared recovery ?

When a user has lost access to every device they own,
they can regain access to their account if they're able to
gather enough people to invite them back.

For more details on how it is done, see [RFC-1000 - Shamir based recovery](../rfc/1000-shamir-based-recovery.md).

### Glossary

> * share: part of a secret needed to recover the account
> * recipient: someone that has a share in the recovery process
> * threshold: number of share needed to recover the account
> * weight: number of shares a recipient has

## Shared recovery creation

During creation, the user can choose:
- a arbitrary list of recipients (with weights)
- a threshold

The default behavior on the cli when no recipient is specified is to
set all admins of the organization as recipients.

Note that external users are not able to choose their recipients
as they have not access to the list of users.

> This is susceptible to change.

```shell
parsec-cli shared-recovery create --device $device
```

Note that recipients are not contacted at this stage.

Nevertheless they'll be able to see if they are part of a
shared recovery setup.

```shell
parsec-cli shared-recovery list -d $device
```

## Recover access with shared recovery

Shared recovery must be initiated by a recipient.
They'll create an shared recovery invitation with the mail
of the user to recover as recipient.

```shell
parsec-cli invite shared-recovery mail@example.com -d $device
```

The user to recover will receive an email with the invitation url,
and will be able to claim the invitation.

```shell
parsec-cli invite claim $invitation_url
```

The only difference with a device or user invitation is that
the claimer will have to choose the order in which to contact
the recipients. Then the sas code exchanges will happen as
usual, until enough shares have been gathered. At which point
the new device is registered and access is fully recovered.

## Example walkthrough

Given an organization where Alice and Arnold are admins, and Bob is a standard user.

For reference, here is the list of all involved devices:

```shell
parsec-cli device list
870 - Org: Arnold <arnold@example.com> @ label
bc1 - Org: Alice <alice@example.com> @ laptop
ea9 - Org: Bob <bob@example.com> @ laptop
```

First Bob needs to create their shared recovery setup.

```shell
# Bob
parsec-cli shared-recovery create --device ea9
Enter password for the device:
✔ Poll server for new certificates
... Creating shared recovery setup
Choose a threshold between 1 and 2
The threshold is the minimum number of recipients that one must gather to recover the account: 2
✔ Shared recovery setup has been created
```

All organization admins (Alice and Arnold) are recipients, as no recipients were provided.
Then Bob had to choose interactively the threshold.
So Bob's shared recovery is all setup.

Oh no! Bob has lost access to their device.
They must contact an Admin to be invited again through a shared recovery process.

Alice can create the invitation and share the URL with Bob.

```shell
# Alice
parsec-cli invite shared-recovery  bob@example.com -d bc1
✔ Poll server for new certificates
Invitation URL: parsec3://127.0.0.1:6770/Org?no_ssl=true&a=claim_shamir_recovery&p=xBCEREHKItJ0lPzzEuk-8q0N
```

Bob can now start the invitation process.

```shell
#Bob
parsec-cli invite claim "parsec3://127.0.0.1:6770/Org?no_ssl=true&a=claim_shamir_recovery&p=xBCEREHKItJ0lPzzEuk-8q0N"
✔ Retrieving invitation info
2 shares needed for recovery
Choose a person to contact now:
> Alice <alice@example.com> - 1 share(s)
  Arnold <arnold@example.com> - 1 share(s)
```

Bob must choose a person to contact first.
Let's choose Alice first.

In the meantime, Alice must be ready to greet Bob.
First, retrieve the invitation token.

```shell
# Alice
parsec-cli invite list -d bc1
✔ Poll server for new certificates
2 invitations found.
844441ca22d27494fcf312e93ef2ad0d	pending	shamir recovery (Bob <bob@example.com>)
```

Then it can be use to greet Bob.
And proceed to a SAS code exchange.

```shell
# Alice
parsec-cli invite greet -d bc1 844441ca22d27494fcf312e93ef2ad0d
✔ Poll server for new certificates
✔ Retrieving invitation info
✔ Waiting for claimer
Code to provide to claimer: 5CDY
✔ Waiting for claimer
Select code provided by claimer: C8UX
```

Now Bob has one share of the two they need.
So they can repeat the process with Arnold.

```shell
# Bob
parsec-cli invite claim "parsec3://127.0.0.1:6770/Org?no_ssl=true&a=claim_shamir_recovery&p=xBCEREHKItJ0lPzzEuk-8q0N"
# ...
Out of 2 shares needed for recovery, 1 were retrieved.
Choose a person to contact now: Arnold <arnold@example.com> - 1 share(s)
Invitation greeter: Arnold <arnold@example.com>
✔ Waiting the greeter Arnold <arnold@example.com> to start the invitation procedure
Select code provided by greeter: DL9Q
Code to provide to greeter: 2VWL
✔ Waiting for greeter
✔ Waiting for greeter
Enter device label: label
✔ Recovering device
Enter password for the new device:
Confirm password:
```

Once the SAS codes are exchanged, Bob can setup their new device with a label and password.
And so the shared recovery process is fully completed.
