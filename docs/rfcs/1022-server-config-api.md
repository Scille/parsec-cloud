<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Server configuration API

## 1 - Goals

Currently the client is able to obtain some configuration from the server, but only on a per-organization base.

Per-organization configuration is convenient since it allows widely different organizations
to be hosted on the same server, however:

- It increases server complexity: configuration has to be stored in the database,
  dedicated administration APIs are required to update the configuration.
- It increase client complexity: configuration cannot be obtained before device login
  (as querying the organization config without authentication would be a form of oracle
  allowing an attacker to know what organization exist).
- In practice widely different organizations are most likely to be hosted on different
  server (typically an organization with very high security requirements may consider
  hosting on a dedicated server as part of its security requirements).

For this reason, this RFC describe a new sever-wide configuration that can be queried
without any authentication.

> Note:
>
> The server-wide configuration doesn't fully replace per-organization configuration
> (typically the user or data limit are still pertinent to be per-organization).

## 2 - Protocol changes

### 2.1 - Replace anonymous account API commands family by anonymous server

Querying the server configuration is done without authentication and without the need
to specify an organization ID.

Currently the anonymous account API family already correspond to this, so we choose to
re-use this API family (and rename it so reflect the fact it is just a route that allow
sending command without authentication that are global to the server).

Changes:

- Rename API commands family "anonymous account" -> "anonymous server"
- Rename server API route `/anonymous_account` -> `/anonymous_server`

> Note:
>
> Since the Parsec account features are not in production yet, we consider acceptable
> to break compatibility by renaming this route.

### 2.2 - Server configuration API

New anonymous server API:

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "server_config",
        "req": {},
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "client_agent",
                        "type": "ClientAgentConfig"
                    },
                    {
                        "name": "account",
                        "type": "AccountConfig"
                    },
                    {
                        "name": "organization_bootstrap",
                        "type": "OrganizationBootstrapConfig"
                    },
                    {
                        "name": "openbao",
                        "type": "OpenBaoConfig"
                    }
                ]
            }
        ],
        "nested_types": [
            {
                "name": "ClientAgentConfig",
                "variants": [
                    {
                        // The server will reject any connection to this organization
                        // from a web client.
                        "name": "NativeOnly",
                        "discriminant_value": "NATIVE_ONLY"
                    },
                    {
                        // The server allows connection to this organization from a web client.
                        "name": "NativeOrWeb",
                        "discriminant_value": "NATIVE_OR_WEB"
                    }
                ]
            },
            {
                "name": "AccountConfig",
                "variants": [
                    {
                        // Parsec account is not available
                        "name": "Disabled",
                        "discriminant_value": "DISABLED"
                    },
                    {
                        // Any anonymous client is allowed to create a Parsec account (as
                        // long as the user can validate the email address he provided).
                        // This Parsec account can then be used as a cross-organization
                        // hub to:
                        // - List the organization he (i.e. his email address) is part of
                        // - List the pending invitation related to his email address
                        // - Use the account vault to store a key itself protecting a
                        //   local device (used to protect local device on web with the
                        //   Parsec account authentication instead of a separate password).
                        // - Use the account vault to store registration device (used, to
                        //   skip the device-to-device enrollment process when connecting
                        //   to an organization from a machine with no local device).
                        "name": "EnabledWithVault",
                        "discriminant_value": "ENABLED_WITH_VAULT"
                    },
                    {
                        // The user should not store any sensitive data with a low entropy
                        // key server-side using his account vault (typically storing,
                        // encrypted with a password, a registration device or a key
                        // itself protecting a local device).
                        //
                        // Note this is a purely advisory configuration since only the
                        // client can decrypt the vault content, and hence it would be
                        // pointless to try to enforce it on the server side.
                        "name": "EnabledWithoutVault",
                        "discriminant_value": "ENABLED_WITHOUT_VAULT"
                    }
                ]
            },
            {
                "name": "OrganizationBootstrapConfig",
                "variants": [
                    {
                        // The organization must have been first created by a server
                        // administrator, leading to a bootstrap organization URL
                        // (containing a bootstrap token) that must be used to proceed
                        // with the bootstrap.
                        "name": "WithBootstrapToken",
                        "discriminant_value": "WITH_BOOTSTRAP_TOKEN"
                    },
                    {
                        // If the organization doesn't already exists on the server,
                        // it will be automatically created when a bootstrap is attempted
                        // (hence there is no bootstrap token involved in this case and
                        // any anonymous client is allowed to create an organization,
                        // use this carefully!).
                        "name": "Spontaneous",
                        "discriminant_value": "SPONTANEOUS"
                    }
                ]
            },
            {
                // An OpenBao server can be configured to allow SSO (using Open ID
                // Connect) authentication for users.
                // This is done by storing an opaque key on the OpenBao server that is
                // itself typically used to encrypt the local device keys.
                //
                // Obviously the level of security is lower than traditional approaches
                // (e.g. storing the opaque key on the OS Keyring or derive it from a
                // strong password).
                // However this is a trade-of to increase user-friendliness, the decision
                // whether or not to use this is to be made by the server administrator
                // according to its own threat model.
                "name": "OpenBaoConfig",
                "discriminant_field": "type",
                "variants": [
                    {
                        "name": "Disabled",
                        "discriminant_value": "DISABLED"
                    },
                    {
                        "name": "Enabled",
                        "discriminant_value": "ENABLED",
                        "fields": [
                            {
                                // Base URL to the OpenBao server
                                "name": "server_url",
                                "type": "String"
                            },
                            {
                                "name": "secret",
                                "type": "OpenBaoSecretConfig"
                            },
                            {
                                "name": "auths",
                                "type": "List<OpenBaoAuthConfig>"
                            }
                        ]
                    }
                ]
            },
            {
                "name": "OpenBaoSecretConfig",
                "discriminant_field": "type",
                "variants": [
                    {
                        "name": "KV2",
                        "discriminant_value": "KV2",
                        "fields": [
                            {
                                // Basically secret are going to be fetched from
                                // `<openbao_server_url>/<openbao_secret_mount_path>/data/<secret_path>`
                                // see https://openbao.org/api-docs/secret/kv/kv-v2/#read-secret-version
                                "name": "mount_path",
                                "type": "String"
                            }
                        ]
                    }
                ]
            },
            {
                "name": "OpenBaoAuthConfig",
                "discriminant_field": "type",
                "variants": [
                    {
                        "name": "OIDCHexagone",
                        "discriminant_value": "OIDC_HEXGONE",
                        "fields": [
                            {
                                // Basically OIDC authentication is going to be done with
                                // `<openbao_server_url>/<openbao_mount_path>/oidc/auth_url`
                                // see https://openbao.org/api-docs/auth/jwt/#oidc-authorization-url-request
                                "name": "mount_path",
                                "type": "String"
                            }
                        ]
                    },
                    {
                        "name": "OIDCProConnect",
                        "discriminant_value": "OIDC_PRO_CONNECT",
                        "fields": [
                            {
                                // Basically OIDC authentication is going to be done with
                                // `<openbao_server_url>/<openbao_mount_path>/oidc/auth_url`
                                // see https://openbao.org/api-docs/auth/jwt/#oidc-authorization-url-request
                                "name": "mount_path",
                                "type": "String"
                            }
                        ]
                    }
                ]
            }
        ]
    }
]
```

### 2.3 - Move `allowed_client_agent/account_vault_strategy` from organization-level to server-level config

Changes:

- Remove `allowed_client_agent/account_vault_strategy` from `list_organizations` (administration REST API)
- Remove `allowed_client_agent/account_vault_strategy` from `events_listen` (authenticated API)
- Remove `allowed_client_agent/account_vault_strategy` from `organization_self_list` (authenticated account API)
- Remove `--organization-initial-allowed-client-agent/--organization-initial-account-vault-strategy` from `parsec run` server CLI command and replace them by `--allowed-client-agent/--account-config`.

### 2.4 - Add OpenBao configuration

Add `--openbao-server-url/--openbao-secret-mount-path/--openbao-auth-pro-connect/--openbao-auth-hexagone` to `parsec run` server CLI command.
