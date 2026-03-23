# Libparsec Protocol

This crate provides the schema definition, implementation and serialization
tests for Parsec protocol commands (i.e. client-server RPC API).

Those commands are grouped into "command families":

- `authenticated_cmds`: Used to access an organization with a enrolled device. This is the most common family.
- `anonymous_cmds`/`invited_cmds`: Used to access an organization when the client doesn't have a device yet
  (e.g. `organization_bootstrap`, `invite_claimer_step`).
- `tos_cmds`: Used to accept an organization's Term Of Services (TOS) with a device (since
  `authenticated_cmds` family rejects users that haven't accepted TOS).
- `anonymous_server_cmds`: Used to access server-global resources without authenticated
  (e.g. `server_config`, `account_create_proceed`).
- `authenticated_account_cmds`: Used to access a Parsec account once authenticated within it.

The JSON schemas defining the commands can be found in the [`schema`](./schema) directory.
The commands implementation and tests are generated with procedural macros from the
[`libparsec serialization format`](../serialization_format) crate based on these schemas.

## API schema syntax overview

Here is quick overview of what a schema looks like for the "ping" command.

```json5
[
    {
        "major_versions": [                   // Supported major versions.
            5
        ],
        "cmd": "ping",                        // The command name.
        "req": {                              // The Request structure.
            "fields": [                       // The Request fields.
                {
                    "name": "ping",           // The field name.
                    "type": "String"          // The field type.
                }
            ]
        },
        "reps": [                             // The different Response status.
            {
                "status": "ok",               // The response status name.
                "fields": [                   // The response status fields.
                    {
                        "name": "pong",
                        "type": "String"
                    }
                ]
            }
        ]
    }
]
```

### The json5 format

Schemas are defined in [`.json5` format](https://json5.org/) to allow for inline comments.
Note however that this is currently the only supported `json5` feature since comments are removed
by our parser.

### The `introduced_in` field

The `introduced_in` field is supported for commands and fields. It describes the specific
**API version** in which the command or field was added.

See [`src/version.rs`](src/version.rs).

### The `type` field

The supported types include **scalars**, **containers** and **custom types**. The list of supported types is defined in [`serialization_format/src/types.rs`](../serialization_format/src/types.rs) (see `generate_field_type_enum!` macro).

It worths mentioning the following containers:

- `RequiredOption` => The field must be present but its value can be null.
- `NonRequiredOption` => the field can be missing or its value to be null.

### The `nested_types` field

A commands may specify a `nested_types` field containing custom type definitions used in the request or the response.

This can be either "plain" types:

```json5
{
    "name": "OpenBaoAuthConfig",
    "fields": [
        {
            "name": "id",
            "type": "String"
        },
        {
            "name": "mount_path",
            "type": "String"
        }
    ]
},
```

Or "variant" types:

```json5
{
    "name": "CryptPadConfig",
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
                    "name": "server_url",
                    "type": "String"
                }
            ]
        }
    ]
}
```

## How to add/update protocol commands?

1. Bump API version in `libparsec/crates/protocol/src/version.rs`.
   You most likely want to bump the minor version of the latest API.
   Also document what has changed and why this change is a minor or major one.
2. Add/update .json5 schemas in `libparsec/crates/protocol/schema/`.
   The new schema/field must have a `introduced_in` attribute to document in which
   API version they have been created.
3. Add/update tests in `libparsec/crates/protocol/tests/`. A command must have have
   a dedicated test for it's request and every one of its response statuses (this
   is enforced during tests compilation).

> [!NOTE]
> Tests should contain serialized data, generating them is done as follow:
>
> 1. Write the test normally, but fill the content of the serialized data with
>    an empty placeholder (e.g. `let raw: &[u8] = hex!("");`).
> 2. Ensure the test prints the expected structure (e.g. `println!("***expected: {:?}", expected.dump().unwrap());`).
>    Note the `***expected:` prefix which is going to be used in the next step.
> 3. Run the tests and pipe its result to the `misc/test_expected_payload_cooker.py` script
>    (i.e. `cargo nextest r -p libparsec_protocol 2>&1 | python misc/test_expected_payload_cooker.py`).
>    Since each failing test gets its output printed to stderr, the script is able
>    to retrieve the `***expected:` pattern for our currently invalid tests and
>    display a nicely formatted representation of the serialized expected structure.
> 4. Update the `raw` field in the test with the result obtained from last step.
>    Now our test should no longer be failing.
> 5. Don't remove the `println!("***expected: {:?}", ...)` from the test, as they will
>    be handy to update the test whenever the command gets modified.
