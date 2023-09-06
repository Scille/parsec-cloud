<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Mandatory HumanHandle & DeviceLabel fields

## Basic considerations

Human handle field in user certificate is optional.
Device label in device certificate is also optional.

This is due to multiple reasons:

### 1 - Legacy reason

Those fields have been introduced in Parsec v1.13.
Hence all certificates created with an older version lack those fields.

### 2 - Support non-human user reason

The need for human handle appeared from the fact a human wants to be able to
revoke his current account (i.e. his user in Parsec jargon) and re-create one
(typically in case the account is compromised) while retaining his identity
(e.g. user id "Gorge" is already taken by a now revoked user, hence you'll be
known as "Jorge" from now on...).

On the other hand a non-human account are fine with changing there id (typically
revoking `sgx-anti-virus-20230901` and replacing it by `sgx-anti-virus-20230902`).

However in practice we never used non-human accounts.
On top of that the human handle field doesn't prevent from creating non-human account:
the email field can be set to a dummy value (or even to a valid address to contact
the team responsible for this).

### 3 - Hide identity from outsiders

The Outsider profile is to be used for user that should be allowed to work in
an organization while not being trusted enough to know the full identity of
who is part of said organization.

Typical use-case is a contractor working on a specific project (hence only having
access to said project's workspace) while not actually part of the company the
organization belongs to (hence the contractor cannot get the identity of all
person working within the company by dumping the user certificates).

This is achieved by providing the user and device certificates in two flavours:

- regular one
- redacted: i.e. Regular one, minus human handle / device label fields
  which have been set to None

## The issue

The human handle / device label fields are almost exclusively used by the GUI (the lower
layers using the user id / device id).

It is annoying for the GUI to have to deal with the fact those fields might be missing,
and in such case to roll a custom logic to still have something to display.

This RFC propose to move the custom logic with the libparsec lower layer, so that
the GUI has the guarantee those fields are always present from it point of view.

## The solution

tl;dr:

1. server enforce non-redacted certificates to have a human handle / device label
2. when deserializing a device certificate with no device label, the device name
   is used as device label
3. when deserializing a user certificate with no human handle:

  - the user id is used as human handle label
  - `<uncaseify(user_id)>@redacted.invalid` is used as human handle email (see
    [section 3](### 3) for what is `uncaseify`)

4. the human handle is no longer allowed to use the `redacted.invalid` domain as email
5. the human handle doesn't carry the information whether it was generated or entered
   by a user. Instead, the GUI is expected to determine this from the context.
6. make mandatory the human handle & device label fields in the local device file and the
   invitation message formats (given the None value has never been used in practice)

### 1 - Non-redacted certificates: mandatory Human Handle / Device Label

This is already what is done in practice, but we now have to make it a rule (just like
a redacted certificate is not allowed to have those fields present).

The change is minimal: just return a new error status server-side, change nothing in the
client given this status should never occur (and if it occurs anyway it will be caught
by the generic error status handler)

Of course, this rule cannot be enforced when loading existing certificates given
they could have been created with Parsec<1.13.
Instead we rely on specific deserialization rules to handle missing human handle /
device label field (see below).
The big pro of dealing at deserialization is we need no specific code to deal with
legacy certificates: they are simply handled as regular redacted certificates.

### 2 - Redacted certificates: mandatory Device Label

Device label is trivial to handle: `DeviceLabel` type is a superset of `DeviceName`,
hence the device's device name (which is always present) can be provided as device label.

In practice, device certificates will have the following values:

- Certificate created with Parsec<1.13: `MyPC` (i.e. device name field)
- Non-redacted certificate: `My PC` (i.e. the device label entered by the user)
- Redacted certificate: `e5904a3eccd8451b99ba09324104662e` (i.e. the device name field, which is always a UUID)

### 3 - Human handle for redacted certificate

Just like for device label, human handle label is a superset of user id so the latter can
be directly used.

For the human handle email we have to be more cunning (especially given the email *is* the
field used as identifier where label is only displayed to human with no other purpose
whatsoever):

- The user id is case-sensitive while the email is case-insensitive
- The user id accepts unicode characters (encoded as UTF-8) while
  [email address is more messy about it](https://en.wikipedia.org/wiki/International_email)
- The email format is divided into `<username>@<domain>`

For the domain part we use the [standard `.invalid` tld](https://en.wikipedia.org/wiki/.invalid).
On top of that we use `redacted.invalid` as domain, considering the only non-legacy reason
for not having a human handle set is because the certificate is a redacted one (in practice
there is very few organization old enough to need legacy support, hence not having a
specific way to handle not-redacted-but-also-no-human-handle-field is not worth it).

For the username part we cannot use the user id field as-is, but instead apply to it
the `uncaseify` algorithm that encode the case.

`uncaseify` is as follow:

1. replace any `_` by `__`
2. replace any uppercase character by underscore followed by the lowercase version of the character

```python
def uncaseify(txt: str) -> str:
    out = ""
    for c in txt:
        if c.isupper():
            out += "_"
            out += c
        elif c == "_":
            out += "__"
        else:
            out += c
    return out

assert uncaseify("a") == "a"
assert uncaseify("_a_") == "__a__"
assert uncaseify("aAa") == "a_aa"
assert uncaseify("AA") == "_a_a"
assert uncaseify("_A_") == "___a__"
```

`UserID` type has a hard limit of 32 bytes, while the email email address's limit
is 255 bytes (including the domain part and `@`).
The worst case for `uncaseify` is a user id only composed of uppercase (or underscores),
in which case it size is doubled so 64 bytes.
Considering domain is `redacted.invalid` (16 bytes long) and the `@`, there is
238 bytes to accommodate the uncaseified user id (so far more than what is needed).

Finally considering the unicode, the good news is there is nothing to do: our email
format already accept international email address (so a regular user can already use
something like `关羽@蜀.汉`).

### 4 - `redacted.invalid` not allowed for real human handle

Let's consider the following scenario:

1. User 1 (user id = "alice") is created with an old Parsec version, hence it has no human handle
2. User 2 (user id = "bob") is created with human handle email = "alice@redacted.invalid"

Now we end up with Parsec displaying the same human handle email for both user 1 and 2 :(
The easy solution for this is simply to consider `redacted.invalid` as a reserved domain that
is not allowed for real human handle.

Typically, `HumanHandle` method would look like:

```rust
impl HumanHandle {
    pub fn new(email: &str, label: &str) -> Result<Self, &'static str> {
      // Create regular human handle from the provided email & label
      // here we ensure email doesn't use `redacted.invalid` domain
    }

    pub fn new_redacted(user_id: &UserID) -> Self {
      // Here we compute label and email from the user id (hence email uses
      // `redacted.invalid` domain)
      // This method should only be used in the code deserializing `UserCertificate`
    }
```

### 5 - Don't carry presence/absence of the fields to the GUI

As we've seen, legacy certificates are handled just like redacted certificates. On top
of that redacted certificates are only provided to Outsider user.

Given legacy certificates are not that common, there is no need to convey this information
to the GUI.

Regarding redacted certificates, the GUI already has to know the user is an outsider
to adapt what is displayed (e.g. outsider cannot create new workspaces).

Hence there is no need to provide a specific field to inform where the human handle /
device label fields come from.

### 6 - Optional device label / human handle turned mandatory in some schemas

Multiple schemas contains an Optional human handle and/or device label.
They can be divided into two categories:

1. the option was present for future-proofing and never used in practice
2. the option was present for backward compatibility

Items in case 1 are:

- command `invite_info`
- command `human_find` (removed in APIv4 so can be ignored)
- data `invite_device_confirmation` & `invite_user_confirmation`
- data `invite_device_data` & `invite_user_data`
- data `pki_enrollment_answer_payload`
- `device_file_recovery`
- `device_file_password`
- `device_file_smartcard`

Items in case 2 are:

- `legacy_device_file_password`
- `local_device`

So for case 1 we just modify the schema specification to make human handle / device label
mandatory (again, which is expected to have zero impact in practice).

For case 2, we rely use the mechanism developed for redacted certificate to generate
a valid human handle / device label. Of course, local device & it file storage format
has obviously nothing to do with redacted certificate, but this just a convenient
way to handle this exotic case !
