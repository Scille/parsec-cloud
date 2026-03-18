# Libparsec types

This crate provides all the bases types used by the rest of libparsec crates.

> [!NOTE]
> Given how common they are, the types from this crate should always be imported as `use libparsec::prelude::*`.

There is two kind of types defined here:

- Common types: Those are very common types used across libparsec (e.g. `DeviceID`, `FsPath`, `DateTime`).
  While most of them implement (de)serialization, there are not supposed to be serialized alone but instead
  be part of a data type (see bellow).
- Data type: Those types are meant to be serialized (e.g. `UserCertificate`, `LocalDevice`, `FileManifest`).
  As such they are defined as JSON schemas so that backward compatibility can be easily enforced.
  Those types are low-level and only used within `libparsec_types` crates, see bellow.
- Data type wrapper: Those types abstract away the data types' backward compatibility by providing
  clean types that can be used by higher level crates (typically handling the renaming of a field,
  or adding a default value for newly added field when deserializing an older data).

The JSON schemas defining the data types can be found in the [`schema`](./schema)
directory. The data types implementation are generated with procedural macros
from the [`libparsec serialization format`](../serialization_format) crate
based on these schemas.

## How to add/update data type schemas?

1. Add/update .json5 schemas in `libparsec/crates/types/schema/`.
   The new schema/field must have a `introduced_in` attribute to document in which
   Parsec version they have been created (e.g. `350` if introduced in Parsec 3.5.0).
2. Add/update tests in `libparsec/crates/types/tests/`. Don't remove previous tests
   if you modify an existing schema but instead add a new testcase (the old test
   ensures our updated schema is still backward compatible).

> [!NOTE]
> Tests should contain serialized data, generating them is done as follow:
>
> 1. Write the test normally, but fill the content of the serialized data with
>    an empty placeholder (e.g. `let raw: &[u8] = hex!("");`).
> 2. Ensure the test prints the expected structure (e.g. `println!("***expected: {:?}", expected.dump());`).
>    Note the `***expected:` prefix which is going to be used in the next step.
> 3. Run the tests and pipe its result to the `misc/test_expected_payload_cooker.py` script
>    (i.e. `cargo nextest r -p libparsec_types 2>&1 | python misc/test_expected_payload_cooker.py`).
>    Since each failing test gets its output printed to stderr, the script is able
>    to retrieve the `***expected:` pattern for our currently invalid tests and
>    display a nicely formatted representation of the serialized expected structure.
> 4. Update the `raw` field in the test with the result obtained from last step.
>    Now our test should no longer be failing.
> 5. Don't remove the `println!("***expected: {:?}", ...)` from the test, as they will
>    be handy to update the test whenever the command gets modified.
