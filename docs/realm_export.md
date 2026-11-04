<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Realm export

## HORS-DOCUMENTATION: Créer un environnement pour tester la feature

```shell
# Start PostgreSQL cluster
py.test server/tests/ --run-postgresql-cluster
# Start server using the PostgreSQL cluster as db & blockstore
python -m parsec run --dev --db=postgresql://postgres@localhost:57007/postgres --blockstore=POSTGRESQL
# Create sequester authority RSA key
openssl genrsa -aes256 -out sequester_authority_private.pem 4096
openssl rsa -in sequester_authority_private.pem -pubout -out sequester_authority_public.pem
openssl rsa -in sequester_authority_private.pem  -out sequester_authority_private_decrypted.pem
# Create sequester service 1 RSA key
openssl genrsa -aes256 -out sequester_service_1_private.pem 4096
openssl rsa -in sequester_service_1_private.pem -pubout -out sequester_service_1_public.pem
openssl rsa -in sequester_service_1_private.pem  -out sequester_service_1_private_decrypted.pem
# Create orga
cargo run -p parsec-cli organization create --token=s3cr3t --addr='parsec3://127.0.0.1:6777?no_ssl=true' TestOrg
# Bootstrap orga with sequester
# cspell:disable-next-line
echo password | cargo run -p parsec-cli organization bootstrap --addr='parsec3://127.0.0.1:6777/TestOrg?no_ssl=true&a=bootstrap_organization&p=xBDR3NS1sGEezL_vaQEsbvXG' --device-label=PC1 --label='John Doe' --email=john.doe@example.com --sequester-key=./sequester_authority_public.pem --password-stdin
# Create Sequester service 1 certificate
python -m parsec sequester generate_service_certificate --service-label="Service 1" --service-public-key=../parsec-dummy-sequester-keys/sequester_service_1_public.pem --authority-private-key=../parsec-dummy-sequester-keys/sequester_authority_private_decrypted.pem
# /!\ if the workspace is created before the sequester service, a realm key rotation
# must be triggered otherwise the sequester service won't have access to the workspace.
# To do that, just start the Parsec GUI and connect as a user with OWNER role in the
# workspace.

# Create workspace & add data
cargo run -p parsec-cli workspace create wksp1 --device=0d5
cargo run -p parsec-cli workspace import --workspace=9275cdee7a1942cfae357ebb945f71a6 --device=0d5 ./README.rst /parsec_readme.rst
# Realm human access view
python -m parsec human_accesses --db=postgresql://postgres@localhost:57007/postgres --organization=TestOrg
# Realm export
python -m parsec export_realm --db=postgresql://postgres@localhost:57007/postgres --blockstore=POSTGRESQL --organization=TestOrg --realm=9275cdee7a1942cfae357ebb945f71a6
# Mount realm export (using both sequester and device for decryption)
cargo run -p parsec-cli mount-realm-export \
  ./parsec-export-TestOrg-realm-9275cdee7a1942cfae357ebb945f71a6-20250223T214759Z.sqlite \
  --decryptor sequester:ef9adae7ee9f44cc9f974fdcaaff8839:../parsec-dummy-sequester-keys/sequester_service_1_private_decrypted.pem \
  --decryptor device:0d5
```

Obviously `cargo run -p parsec-cli` commands are for dev, in the actual documentation
we should simply refer to `parsec-cli`.
Same thing for `python -m parsec` should be `parsec`.

Of course actual testing should be done against a real server, not a dev environment
on the same machine ;-p

## 0 - Introduction

Realm vs Workspace: In Parsec vocabulary, workspace and realm are two sides of the same
coin. In a nutshell, the realm is a server-side concept that references the big pile of
encrypted data, while the workspace is the client-side concept that reference those
data once decrypted.

Hence the "realm export" is the operation of exporting from the server all the encrypted
data that, when used with the right decryption keys, will allow us to have a full read
access on a workspace at any point in time up to the export date.

Bird eye view:

- An organization exists with a workspace
- Form the server cli, a realm export is done. This generates a big `.sqlite` file
  containing all encrypted data belonging to this realm.
- The realm export file is transfered to a machine containing the decryption keys:
  - Sequestered service key in case of a sequestered organization.
  - Device key otherwise (i.e. decrypt the realm export from a machine normally
    used to run the Parsec client).

## 1 - A word about sequestered organizations

The whole point of a sequestered organization is to have a way to decrypt data without
relying on the user's device.

This is useful to ensure all data can always be decrypted (e.g. for legal reason).

A sequestered organization works as follow:

1. During its bootstrap, the organization declared as sequestered by providing it
   a sequester authority verify key (i.e. the public part of an RSA key in PEM format).
2. At any time, the sequester authority signing key can be used to create (or revoke)
   sequester service.
3. Each sequester service has its own encryption RSA key that is used by any user doing
   a realm key rotation to encrypt the new key (hence giving to the sequester service
   the same access to the workspace data as any member of the workspace).
4. When access to the data is needed, a realm export is done. Then the realm export file
   is decrypted using the sequester service RSA decryption key.

> Note:
>
> Steps 2 & 4 are very sensitive since they involve the private part of RSA keys.
> For this reason it is recommanded to do those steps on a dedicated machine with no
> access to the internet.

### Examples

Bootstrap a sequestered organization:

```shell
# 1. Create the sequester authority RSA key
openssl genrsa -aes256 -out sequester_authority_private.pem 4096
openssl rsa -in sequester_authority_private.pem -pubout -out sequester_authority_public.pem
# 2. Actual organization bootstrap
parsec-cli organization bootstrap --addr=$BOOTSTRAP_TOKEN --device-label='John PC' --label='John Doe' --email=john.doe@example.com --sequester-key=./sequester_authority_public.pem
```

> Note:
>
> `$BOOTSTRAP_TOKEN` is the token obtained during the organization create step.

Create a new sequester service:

```shell
# 1. Create the sequester service RSA key
$ openssl genrsa -aes256 -out sequester_service_1_private.pem 4096
$ openssl rsa -in sequester_service_1_private.pem -pubout -out sequester_service_1_public.pem
# 2. On a secure machine with access to sequester authority signing key, create
#    the sequester service certificate.
$ parsec sequester generate_service_certificate --service-label="Service 1" --service-public-key=../parsec-dummy-sequester-keys/sequester_service_1_public.pem --authority-private-key=./sequester_authority_private_decrypted.pem
Sequester service certificate Service 1 (id: ef9adae7ee9f44cc9f974fdcaaff8839, timestamp: 2025-02-23T21:19:35.484948Z) exported in ./sequester_service_certificate-ef9adae7ee9f44cc9f974fdcaaff8839-2025-02-23T21:19:35.484948Z.pem
Use parsec sequester create_service command to add it to an organization
# 3. Create the sequester service on the server using the certificate
$ parsec sequester create_service --db=$POSTGRESQL_URL --organization=$ORGANIZATION_ID --service-certificate=./sequester_service_certificate-ef9adae7ee9f44cc9f974fdcaaff8839-2025-02-23T21:19:35.484948Z.pem
Service created
```

> Note:
>
> Once added, the sequester service cannot decrypt any data since no realm key has been
> encrypted with its public key yet.
>
> However the sequester service creation is detected by all Parsec client with a OWNER
> role in the workspace and they automatically trigger a realm key rotation in order
> to ensure the sequester service gets access to the workspace data.
>
> The same behavior is also triggered whenever the sequester service is revoked, this
> time in order to ensure any new data added to the workspace won't be accessible to
> the sequester service.

Revoke a sequester service:

```shell
$ parsec sequester generate_service_revocation_certificate --service-id=ef9adae7ee9f44cc9f974fdcaaff8839 --authority-private-key=../parsec-dummy-sequester-keys/sequester_authority_private_decrypted.pem
Sequester service revocation certificate (sequester service ID: ef9adae7ee9f44cc9f974fdcaaff8839, timestamp: 2025-02-23T21:39:54.702997Z) exported in ./sequester_service_revocation_certificate-ef9adae7ee9f44cc9f974fdcaaff8839-2025-02-23T21:39:54.702997Z.pem
Use parsec sequester revoke_service command to add it to an organization
$ parsec sequester revoke_service --db=$POSTGRESQL_URL --organization=$ORGANIZATION_ID --service-revocation-certificate=./sequester_service_revocation_certificate-ef9adae7ee9f44cc9f974fdcaaff8839-2025-02-23T21:39:54.702997Z.pem
Service revoked
```

> Note:
>
> Revoking a sequester service means it won't get access to any new realm key rotation.
> However it still has access to all data encrypted with previous realm keys and hence
> can still be used to decrypt realm exports up to the point of revocation.

## 2 - Find what realm should be exported

Since the workspace name is encrypted, this information is not available from the server.

However a convenient way to determine which realm is the one to be exported is to
use the `parsec human_accesses` command that list who has access over which realm:

```shell
$ parsec human_accesses --db=$POSTGRESQL_URL --organization=$ORGANIZATION_ID
Found 2 results:
Human John Doe <john.doe@example.com>

  User 02e0486752d34d6ab3bf8e0befef1935 (REVOKED)
    2000-01-01T00:00:00Z: Created with profile STANDARD
    2000-01-02T00:00:00Z: Updated to profile CONTRIBUTOR
    2000-12-31T00:00:00Z: Revoked

  User 9e082a43b51e44ab9858628bac4a61d9 (ADMIN)
    2001-01-01T00:00:00Z: Created with profile ADMIN

    Realm 8006a491f0704040ae9a197ca7501f71
      2001-02-01T00:00:00Z: Access OWNER granted
      2001-02-02T00:00:00Z: Access removed
      2001-02-03T00:00:00Z: Access READER granted

    Realm 109c48b7c931435c913945f08d23432d
      2001-02-01T00:00:00Z: Access OWNER granted

Human Jane Doe <jane.doe@example.com>

  User baf59386baf740bba93151cdde1beac8 (OUTSIDER)
    2000-01-01T00:00:00Z: Created with profile OUTSIDER

    Realm 8006a491f0704040ae9a197ca7501f71
      2001-02-01T00:00:00Z: Access READER granted
```

## 3 - Do a realm export

The realm export is done with the server CLI:

```shell
parsec export_realm --db=$POSTGRESQL_URL --blockstore=$S3_URL --organization=$ORGANIZATION_ID --realm=$REALM_ID
```

This creates a file `parsec-export-$ORG-realm-$REALM-$TIMESTAMP.sqlite`.

> Notes:
>
> - The realm export can be limited up to a specific point in time using the
>   `--snapshot-timestamp` option.
> - A stopped export operation (using ^C) can be restarted by passing the output
>   file as argument along with the time of the initial export with `--snapshot-timestamp`.

## 4 - Read the realm export

The realm export file should then be transfered to a secured machine containing the
decryption keys.

The decryption keys can be of two types:

- Sequestered service key in case of a sequestered organization.
- Device key otherwise (i.e. decrypt the realm export from a machine normally
  used to run the Parsec client).

It is recommended to use a dedicated machine for this operation with no access to the
internet.

And from there the client CLI should be used to decrypt the workspace
from realm export file and mount it as a local directory.

Also note a `--timestamp=<TIMESTAMP_AS_RFC3339>` option can be provided to specify
the exact point in time at which the realm should be mounted.

### Decrypt with a sequestered service

```shell
$ parsec-cli mount-realm-export --decryptor sequester:ef9adae7ee9f44cc9f974fdcaaff8839:./sequester_service_1_private_decrypted.pem  ./parsec-export-TestOrg-realm-9275cdee7a1942cfae357ebb945f71a6-20250223T214759Z.sqlite
Organization: TestOrg
Realm ID: 9275cdee-7a19-42cf-ae35-7ebb945f71a6
Export temporal bounds: 2025-02-23T20:29:07.603857Z to 2025-02-23T21:47:59.873115Z
polling for the start...
mountpoint is ready !
Mounted at "./parsec-export-TestOrg-realm-9275cdee7a1942cfae357ebb945f71a6-20250223T214759Z"
```

> Note:
>
> The sequester service key is expected to be provided as a PEM file containing
> the key decrypted (i.e. file must start with `-----BEGIN PRIVATE KEY-----`).

### Decrypt with a device

```shell
$ parsec-cli mount-realm-export --decryptor device:$DEVICE_ID ./parsec-export-TestOrg-realm-9275cdee7a1942cfae357ebb945f71a6-20250223T214759Z.sqlite
Enter password for the device:
Organization: TestOrg
Realm ID: 9275cdee-7a19-42cf-ae35-7ebb945f71a6
Export temporal bounds: 2025-02-23T20:29:07.603857Z to 2025-02-23T21:47:59.873115Z
polling for the start...
mountpoint is ready !
Mounted at "./parsec-export-TestOrg-realm-9275cdee7a1942cfae357ebb945f71a6-20250223T214759Z"
```

> Note:
>
> Multiple keys can be provided (by passing multiple `--decryptor` options). This is not
> useful under normal conditions since any sequester service / user with access to the
> workspace is able to decrypt all it data.
> However it may come handy in case of a partially corrupted workspace (e.g. in case a
> malicious/buggy actor has provided corrupted keys during a realm key rotation).
