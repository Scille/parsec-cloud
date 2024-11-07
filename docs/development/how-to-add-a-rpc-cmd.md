<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# How to add an API RPC command to Parsec

The goal of this document is to cover all the necessary steps to add an API RPC command to Parsec.

> Client-to-server RPC API routes are also called "commands" in Parsec. There are three command families: `anonymous_cmds` (authentication not required), `authenticated_cmds` (authentication required) and `invited_cmds` (invitation required).
>
> Note there is another unrelated API for administration purpose that uses REST instead of RPC (as it is expected to be used by admin using curl), this documentation doesn't cover it.

In summary:

1. Add the schema describing the new API RPC command
2. Generate binding code and serialization tests
3. Implement server-side code (In-memory & PostgreSQL) and tests
4. Implement client-side code and tests

> Note that you will need a working development environment. See (./README.md).

## Schema generation

Let's add a new `dummy` route to `invited_cmds`.

In [this folder](../../misc/libparsec/crates/protocol/schema/invited_cmds/) add a `dummy.json5` schema describing the API RPC command.

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "dummy",
            "fields": [
                {
                    "name": "dummy_field",
                    "type": "String"
                }
            ]
        },
        "reps": [
            {
                "status": "dummy_rep_with_fields",
                "fields": [
                    {
                        "name": "dummy_rep_field",
                        "type": "String"
                    }
                ]
            },
            {
                "status": "dummy_rep_without_fields",
            }
        ]
    }
]

```

For a full description of the accepted format see [the parsing/generating script](../../misc/gen_protocol_typings.py).

> Note that the files are .json5. This is to allow for comments inside these files. But the parser we are using only supports .json files and we're removing the comments ourselves. So don't use any other specific feature of the json5 format (trailing commas, I'm looking at you 👀).

Then from the root directory, run the following scripts.

```shell
python make.py python-dev-rebuild # short option `r`
python misc/gen_protocol_typings.py
```

The `gen_protocol_typing` will only generate typing and modify the following files, that must be committed:
    - `server/parsec/_parsec_pyi/protocol/__init__.pyi`
    - `server/parsec/_parsec_pyi/protocol/xxxxxx_cmds/v4/__init__.pyi`
    - `server/tests/common/rpc.py`


`python-dev-rebuild` will rebuild the bindings in `server/parsec/_parsec.cpython-312-x86_64-linux-gnu.so`. Use can use `python-dev-install` to install before building the bindings (short option `i`).

## Serialization tests

After bindings generation, the serialization tests become required.

In `libparsec/crates/protocol/tests/xxxxx_cmds/v4/dummy.rs` for each req and rep variant, there must be a public function named `req` or `rep_{variant name}`. Otherwise, you'll have this kind of error:

```shell
error[E0425]: cannot find function `rep_dummy_rep_without_fields` in module `dummy`
  --> libparsec/crates/protocol/tests/serialization.rs:15:1
   |
15 | protocol_cmds_tests!("schema/xxxxx_cmds");
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `dummy`
```

As for the content, the goal of these serialization tests is to go from bytes to the tested type, assert it's OK and back to bytes and assert it's adding up.

On how to generate the bytes you can use [this script](../../misc/test_expected_payload_cooker.py)

> TLDR :
> - add `println!("***expected: {:?}", expected.dump().unwrap());` in each new test
> - execute `$ cargo nextest run -p libparsec_protocol 2>&1 | python ./misc/test_expected_payload_cooker.py`
> - copy paste the output

## Server-side implementation

### Optional: the route introduces a new component

In `server/parsec/components` you define a base component. It will need to have a corresponding memory and postgresql components in their respective folder.

Add your component to `class Backend` in `server/parsec/backend.py`

Add it to the `components_factory` both in `server/parsec/components/memory/factory.py` and `server/parsec/components/postgresql/factory.py`

### Defining the route

In a component (maybe the one defined just before), you'll need to define this kind of function.

```python
    @api
    async def api_dummy(
        self,
        client_ctx: xxxxxClientContext,
        req: xxxxx_cmds.latest.dummy.Req,
    ) -> xxxxx.latest.dummy.Rep:
        match await self.do_dummy():
            case None:
                return invited_cmds.latest.dummy.Rep()
            case DummyBadOutcome.DUMMY_SPECIFIC_ERROR:
                return invited_cmds.latest.dummy.RepDummyRepWithoutFields()
            case DummyBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
```

This function will mostly contain error management logic. The component will also contain a generic method, that will be overridden in the memory and postgresql implementations.

> Note that some common errors, like organization not found, have helpers that return the appropriate error.

```python
    async def do_dummy(
        self,
        # parameters found in client_ctx and req
    ) -> None | DummyBadOutcome:
        raise NotImplementedError
```

Your errors must inherit from `BadOutcomeEnum`.

The actual implementation will happen in the memory and postgresql components.

```python
class MemoryDummyComponent(BaseDummyComponent):
    def __init__(self, data: MemoryDatamodel, event_bus: EventBus) -> None:
        super().__init__()
        self._data = data
        self._event_bus = event_bus

    @override
    async def do_dummy(
        self,
        # parameters found in client_ctx and req
    ) -> None | DummyBadOutcome:
        print("There are ",len(self._data.organizations), " organizations here.")
```

### Server tests

Add the new test file `server/tests/api_v4/xxxxx/dummy.py`. It should contain a test function for each response status (see other command's tests as a guide on how to write them).

Import the new test file in `server/tests/api_v4/xxxxx/__init__.py`. It must be named `test_{new command name}` (e.g. `test_dummy`).

Start a new testbed :

```shell
python make.py run-testbed-server # short option `rts`
```

For more information on se testbed see [here](README.md/#starting-the-testbed-server).

`test_each_cmd_req_rep_has_dedicated_test` will fail if a test function is missing

## Client-side implementation

TODO
