Test scripts
============

This directory contains all the scripts used for the manual testing of the application.
It has a single entry point called `run_testenv` that prepares a test environment
in the current shell:

On linux:

```bash
source tests/scripts/run_testenv.sh
```

On windows:

```cmd
./tests/scripts/run_testenv.bat
```

The requirements for running this script is a python virtualenv with a full installation
of parsec:

```bash
pip install -e .[all]
```

Default behavior for `run_testenv`
-------------------------------------------

The `run_testenv` script starts by creating a temporary directory dedicated
to this test session. The environment variables are then set accordingly.

- On linux: `XDG_CACHE_HOME`, `XDG_DATA_HOME` and `XDG_CONFIG_HOME`
- On windows: `APPDATA`

Check that the script has been properly sourced using:

On linux:

```bash
echo $XDG_CONFIG_HOME
```

On windows:

```cmd
echo %APPDATA%
```

It will then proceed to configure the MIME types of this environment in order to
support the `parsec://` schema properly (linux only).

Check that the schema is properly registered using `xdg-open`:

```bash
xdg-open parsec://this-url-is-not-valid
```

It will then proceed to start a new backend server as a background task. If another
parsec backend is already running on the same port, the existing process will first
be killed.

Check that the backend is running using `netstat`:

```bash
netstat -tlp | grep 6888
```

It will then proceed to the initialization of a test organization, using the
`parsec.test_utils.initialize_test_organization` function. More precisely, the
test organization has two registered users, alice and bob who both own two devices,
laptop and pc. They each have their workspace, respectively `alice_workspace` and
`bob_workspace`, that their sharing with each other.

Check that the test organization has been properly created using parsec GUI:

```bash
parsec core gui
```

Parameterize `run_testenv`
------------------------------------

The script can be customized with many options. The full list is available through
the `--help` option:

```bash
$ source tests/scripts/run_testenv.sh --help
  [...]
  Options
  -B, --backend-address FROM_URL
  -p, --backend-port INTEGER      [default: 6888]
  -O, --organization-id ORGANIZATIONID
                                [default: corp]
  -a, --alice-device-id DEVICEID  [default: alice@laptop]
  -b, --bob-device-id DEVICEID    [default: bob@laptop]
  -o, --other-device-name TEXT    [default: pc]
  -x, --alice-workspace TEXT      [default: alice_workspace]
  -y, --bob-workspace TEXT        [default: bob_workspace]
  -P, --password TEXT             [default: test]
  -T, --administration-token TEXT
                                      [default: V8VjaXrOz6gUC6ZEHPab0DSsjfq6DmcJ]
    --force / --no-force            [default: False]
    -e, --empty
```

In particular:

- `--backend-address` can be used to connect to an existing backend instead of
    starting a new one
- `--backend-port` can be used to specify a backend port and prevent the script from
    killing the previously started backend
- `--empty` that can be used to initialize and empty environment. This is especially
  useful for testing the invitation procedure.

Example: testing the invitation procedure
-----------------------------------------

The following scenario shows how the parsec invatation procedure can easily be tested
using the `run_testenv` script and two terminals.

In a first terminal, run the following commands:

```bash
$ source tests/scripts/run_testenv.sh
$ parsec core gui
  # Connect as bob@laptop and register a new device called pc
  # Copy the URL
```

Then, in a second terminal:

```bash
$ source tests/scripts/run_testenv.sh --empty
$ xdg-open "<paste the URL here>"  # Or
$ firefox --no-remote "<paste the URL here>"
  # A second instance of parsec pops-up
  # Enter the token to complete the registration
```

Note that the two GUI application do not conflict with one another as they're
running in different environments. It works exactly as if they were being run
on two different computers.
