<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Sequester Test Guide

Sequester feature requires 2 pairs of asymmetric RSA keys:

- **Service signature key**: used at organization bootstrap to certify all services.
- **Sequester encryption key**: used by a sequester service to encrypt the manifests.

## Summary

- [Summary](#summary)
- [Prerequisite](#prerequisite)
- [Generate the RSA keys](#generate-the-rsa-keys)
- [Create a fresh organization](#create-a-fresh-organization)
- [Create the sequester service](#create-the-sequester-service)
  - [List service used in an organization](#list-service-used-in-an-organization)
  - [Enable or disable a service](#enable-or-disable-a-service)
- [Dump the sequester data](#dump-the-sequester-data)

## Prerequisite

The following instruction will consider that you have exported the following variables:

| Name                          | Description                                                                                                                                                        |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `PARSEC_SERVER_ADDR`         | The URL to access the parsec server (using the `parsec:` scheme).                                                                                                  |
| `PARSEC_ADMINISTRATION_TOKEN` | The administration token to perform admin task on the server.                                                                                                      |
| `PARSEC_DB`                   | The URL to the database used by the parsec server.                                                                                                                 |
| `PARSEC_BLOCKSTORE`           | The URL to access the block storage service used by the server.                                                                                                   |
| `PARSEC_ORGANIZATION_ID`      | The organization ID that will be created in this tutorial, you could set it to something like `TestSequester-$(parsec.cli --version \| cut -f3 -d' ' \| tr . - )`. |

## Generate the RSA keys

```shell
for key in {sequester-service-key,sequester-verify-key}; do
  openssl genrsa -out $key 4096
  openssl rsa -in $key -pubout -out $key.pub
done
```

## Create a fresh organization

```shell
parsec.cli core create_organization "$PARSEC_ORGANIZATION_ID"
```

This command return a bootstrap link, save that value into the variable `BOOTSTRAP_ADDR` (that value while be used for the next command).

Now we bootstrap the organization using the `sequester-verify-key`:

```shell
parsec.cli core bootstrap_organization "$BOOTSTRAP_ADDR" --sequester-verify-key sequester-verify-key.pub
```

The organization is now ready.

## Create the sequester service

We are now using the server CLI to interact directly with the server database.

```shell
parsec.cli server sequester create-service \
  --organization "$PARSEC_ORGANIZATION_ID" \
  --service-label "TestSequesterService" \
  --service-public-key sequester-service-key.pub \
  --authority-private-key sequester-verify-key \
  --service-type storage
```

That command requires the service public key (used for manifest encryption) and the service signing private key to sign the encryption key.

### List service used in an organization

The new service could be listed with the following command:

```shell
parsec.cli server sequester list_services --organization "$PARSEC_ORGANIZATION_ID"
```

> The `list_services` also provide the service-id that could be used to enable/disable a specific service.

### Enable or disable a service

```shell
parsec.cli server sequester update_service \
  --organization "$PARSEC_ORGANIZATION_ID" \
  --service "$SERVICE_ID"
  --disable # or --enable
```

## Dump the sequester data

> Create some data into the realm to get exportable data.

To dump the data, you will need the realm-id of the organization:

```shell
parsec.cli server human_accesses --organization "$PARSEC_ORGANIZATION_ID"
```

> Save the realm-id into the variable `REALM_ID`.

Export the sequester data:

```shell
parsec.cli server sequester export_realm \
  --organization "$PARSEC_ORGANIZATION_ID" \
  --realm "$REALM_ID" \
  --output sequester-export
```

The workspaces can be retrieved with the encryption private key:

```shell
parsec.cli server sequester extract_realm_export \
  --service-decryption-key sequester-service-key \
  --input "sequester-export/parsec-sequester-export-realm-$(echo $REALM_ID | xxd -p -u).sqlite"
  --output sequester-export
```
