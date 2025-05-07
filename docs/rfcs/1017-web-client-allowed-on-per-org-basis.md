<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Web client allowed on a per-organization basis

## 1 - Goals

While convenient, Parsec web client is provides a lower level of security than the native client.

Its most obvious security flaw is the fact the web client is hosted by the server, allowing
a malicious server to insert backdoors in the web client in order to obtain the user secrets.

For this reason, the choice of using the web client is a matter of convenience vs security trade-off
and hence should be decided on a per-organization basis.

This RFC proposes to add a new configuration to the organization called the "client source strategy".
It can take two values:

- Only native clients are allowed
- Web clients are allowed alongside native clients

## 3 - Changes

## 3.1 - Server configuration

Client source strategy is represented in the organization config as an enum:

```json5
{
    "name": "ClientSourceStrategy",
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
}
```

- Add a `client_source_strategy` column to the  `organization` table in the datamodel.
- Add `organization_initial_client_source_strategy` to `BackendConfig`
- Add `--organization-initial-client-source-strategy` option to run CLI

### 3.2 - Administration API related to organization configuration

Add `client_source_strategy` fields to:

- `GET /administration/organizations/{raw_organization_id}`
- `PATCH /administration/organizations/{raw_organization_id}`

### 3.3 - Additional HTTP error code (464: web client not allowed by organization)

For each anonymous/authenticated/invited API query, the Parsec server should check the `User-Agent`
header to ensure respect the organization configuration:

- If `ClientSourceStrategy` is `NativeOrWeb`, any user-agent is allowed.
- If `ClientSourceStrategy` is `NativeOnly`, only user-agent starting with `Parsec-Client/` is allowed
  (e.g. `Parsec-Client/3.3.2 Windows`). Otherwise the request is rejected with HTTP 464.

> Note: The HTTP code is specific to the fact the web client is not allowed, not to the fact
> the client source strategy is not respected. The reason for this is twofold:
>
> - It makes the HTTP code easier to understand (as the concept of "client source strategy" is not obvious)
> - It simplifies client-side error handling (the client doesn't have to take into account the platform it runs on)

### 3.4 - Authenticated API `event_listen`

Add `client_source_strategy` fields to the `OrganizationConfig` event.

```json5
{
    "cmd": "events_listen",

    // Omitted items
    // […]

    "nested_types": [
        {
            "name": "APIEvent",
            "discriminant_field": "event",
            "variants": [
                {
                    // This event is always fired first upon SSE connection
                    "name": "OrganizationConfig",
                    "discriminant_value": "ORGANIZATION_CONFIG",
                    "fields": [

                        // Omitted items
                        // […]

                        {
                            "name": "client_source_strategy",
                            "type": "ClientSourceStrategy"
                        }
                    ]
                },

                // Omitted items
                // […]
            ]
        },
        {
            "name": "ClientSourceStrategy",
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
        }
    ]
}
```

### 3.5 - Client side

- New variant `WebClientNotAllowedByOrganization` to error `libparsec_client_connection::ConnectionError`
- GUI handling of such error (typically showing a message in a modal and going back to the authentication page).
