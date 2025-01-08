# Libparsec Testbed

The `libparsec_testbed` crate defines common organization templates to prevent code
duplication in tests.

## Organization templates

A template defines the structure of an organization such as:

- the users & their devices
- the workspaces & their sharing rules
- the files and directories inside each workspace

Within a test, this is usually a good starting point (hence the name) and the
organization can be further customized in a way that is relevant to the test
(e.g. revoke an existing user).

Templates also allow to create deterministic tests based on the events that
should be triggered, the order in which they should be triggered or the
specific timestamp at which they should be triggered.

## Use in tests

The templates defined in this crate are used to test Libparsec, Parsec CLI and
the Parsec Server.

### Libparsec testing

The templates can be enabled via the `#[parsec_test]` macro defined in the
`tests_macros` crate.

```rust
#[parsec_test(testbed = "coolorg")]
async fn my_test_function(
    // ...
    env: &TestbedEnv,
){
  // access and customize the organization via the TestbedEnv
}
```

### Parsec CLI testing

Currently, the testbed is only partially used in Parsec CLI tests to provide a
basic Parsec Server and setup/create organization from there (so templates are
not used). See https://github.com/Scille/parsec-cloud/issues/9144.

### Parsec Server testing

The templates are exposed as pytest fixtures in `server/tests/common/client.py`.

## Internals

Organization templates are created with a `TestbedTemplateBuilder` and exposed
via the `TESTBED_TEMPLATES` array.

Example:

```rust
let mut builder = TestbedTemplate::from_builder("my_organization");
builder.bootstrap_organization("alice"); // alice@dev1
builder
    .new_user("bob")
    .with_initial_profile(UserProfile::Standard); // bob@dev1
builder.new_device("alice"); // alice@dev2
builder.new_device("bob"); // bob@dev2
```

The easiest way to add a new template is to take a look at the existing ones in
the the `templates/` directory.
