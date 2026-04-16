<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# How to add (or update) an RPC command

The goal of this document is to cover the necessary steps to add (or update) an RPC command to Parsec.

## Quick introduction

Parsec server exposes the following APIs:

- Administration API (REST): for server administration, not covered here.
- Client-Server API (RPC): for business-logic, client-server communication.

See [`libparsec_protocol` crate's README](../../libparsec/crates/protocol/README.md) for more info on RPC command.

In summary, to add or update an existing RPC command you need to:

1. Add or update the schema describing the RPC command
2. Generate binding code
3. Generate serialization tests
4. Implement server-side code (memory & PostgreSQL) and tests
5. Implement client-side code and tests

> Note that you will need a working development environment. See ([README.md](./README.md)).

## 1. Schema generation

You will find command schemas in [./libparsec/crates/protocol/schema](../../libparsec/crates/protocol/schema) directory.

Let's add a new `dummy` route to the `invited_cmds` family.

Add a `dummy.json5` schema describing the RPC command in [./libparsec/crates/protocol/schema/invited_cmds/](../../libparsec/crates/protocol/schema/invited_cmds/).

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

For a full description of the accepted format see [./misc/gen_protocol_typings.py](../../misc/gen_protocol_typings.py).

Note that the files are `.json5`. This is to allow for comments inside these files, but currently the parser only
supports `.json` files (we're removing the comments ourselves). So don't use any other specific feature of the `json5`
format (trailing commas, I'm looking at you 👀).

## 2. (Re)generate bindings

From the root directory, run the following scripts:

```shell
python make.py python-dev-install  # or python make.py i, to install or upgrade requirements
python make.py python-dev-rebuild  # or python make.py r, to (re)build bindings
```

> Bindings are built in `server/parsec/_parsec.cpython-312-x86_64-linux-gnu.so`.

Now run the following command:

```shell
python misc/gen_protocol_typings.py
```

This will generate typing and modify some files in the [`./server`](../../server) directory that must be committed.

For example:

- `server/parsec/_parsec_pyi/protocol/__init__.pyi`
- `server/parsec/_parsec_pyi/protocol/invited_cmds/v5/__init__.pyi`
- `server/parsec/_parsec_pyi/protocol/invited_cmds/v5/dummy.pyi`
- `server/tests/common/rpc.py`

## 3. Serialization tests

Once bindings are generated, the serialization tests (from libparsec `protocol` crate) will become required.

You can run serialization tests with:

```shell
cargo nextest run -p libparsec_protocol
```

In the corresponding test file (e.g. `libparsec/crates/protocol/tests/invited_cmds/v5/dummy.rs`) there must be public
functions named `req` and `rep_{variant name}` for each `req` (request) and `rep` (response) variant declared in the
command schema. Otherwise, you'll have this kind of error:

```shell
error[E0425]: cannot find function `rep_dummy_rep_without_fields` in module `dummy`
  --> libparsec/crates/protocol/tests/serialization.rs:15:1
   |
15 | protocol_cmds_tests!("schema/invited_cmds");
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `dummy`
```

The test functions should test the serialization process, essentially:

- Convert from bytes to the tested type.
- Assert is OK.
- Convert tested type back to bytes.
- Assert is OK.

> You can use [this script](../../misc/test_expected_payload_cooker.py) to generate the bytes correspond to the command.
> TLDR:
>
> - add `println!("***expected: {:?}", expected.dump().unwrap());` in each new test
> - execute `$ cargo nextest run -p libparsec_protocol 2>&1 | python ./misc/test_expected_payload_cooker.py`
> - copy paste the output

Make sure all serialization tests from the protocol create pass before continuing.

## 4. Server-side implementation

In almost all cases, the RPC command will be implemented somewhere in [`./server/parsec/components`](../../server/parsec/components).

You will find a "base" components and two main implementations:

- [`./server/parsec/components/memory`](../../server/parsec/components/memory)
- [`./server/parsec/components/postgresql`](../../server/parsec/components/postgresql)

### If you updated an existing RPC command

Find where the command is implemented and update implementation accordingly.

### If you introduced a new RPC command related to an existing component

Add the command to the existing base component and then complete memory and postgresql implementations.

See "Command implementation" below.

### If you introduced a new RPC command unrelated to an existing component

1. In `server/parsec/components` add a new base component and the corresponding memory and postgresql implementations.

2. Add your component to `class Backend` in [`./server/parsec/backend.py`](../../server/parsec/backend.py)

3. Add your component to the `components_factory` both in [`server/parsec/components/memory/factory.py`](../../server/parsec/components/memory/factory.py) and [`server/parsec/components/postgresql/factory.py`](../../server/parsec/components/postgresql/factory.py).

See "Command implementation" below.

### Command implementation

In a component (like the one defined just before), you'll need to define this kind of function.

```python
    @api
    async def api_dummy(
        self,
        client_ctx: InvitedClientContext,
        req: invited_cmds.latest.dummy.Req,
    ) -> invited_cmds.latest.dummy.Rep:
        match await self.do_dummy():
            case None:
                return invited_cmds.latest.dummy.Rep()
            case DummyBadOutcome.DUMMY_SPECIFIC_ERROR:
                return invited_cmds.latest.dummy.RepDummyRepWithoutFields()
            case DummyBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
```

This function will mostly contain error management logic. The component will also contain a generic
method, that will be overridden in the memory and postgresql implementations.

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

In any case, tests should be updated to match the updated or added command.

If you added a new command:

1. Add a new test file `server/tests/api_v5/invited/test_dummy.py`.
   It must be named `test_{new command name}.py`.
   It must contain a test function for each response status
   (see other command's tests as a guide on how to write them).

2. Expose the tests from the new test file in `server/tests/api_v5/invited/__init__.py` (i.e. `from .test_dummy import *`).

3. Run tests (see [`./server/README.md`](../../server/README.md)).
   > The test `test_each_cmd_req_rep_has_dedicated_test` will fail if a test function is missing

4. If needed, you can start a new testbed:

```shell
python make.py run-testbed-server # short option `rts`
```

See the [README for more information on the testbed](./README.md/#starting-the-testbed-server).

> [!NOTE]
> If you add a field to an existing command, use the "introduced_in" field
> and don't forget to update the description of the changes [here](../../libparsec/crates/protocol/src/version.rs)

## 5. Client-side implementation

TODO
