# Snapcraft 101

Welcome to snapcraft 101

## Basic Setup

1. Install `snapcraft`:

   ```shell
   snap install snapcraft --classic`
   ```

2. Install `multipass`:

   ```shell
   snap install multipass
   ```

3. Export some env variable:

   By default `snapcraft` will use a VM with 2 VCpu and 2Go.
   To make the building faster we could increase the allocated resource by exporting some variable.

   ```shell
   export SNAPCRAFT_BUILD_ENVIRONMENT_CPU=4
   export SNAPCRAFT_BUILD_ENVIRONMENT_MEMORY=4G
   ```

## Ensure required services are started

```shell
systemctl start snapd snap.multipass.multipassd
```

## Debug a build

```shell
snapcraft build --debug --verbose --shell-after parsec-core
```

> About the provided options:
>
> - `--debug`: will make you enter in a shell if a step fail.
> - `--verbose` : More verbose :shrug:.
> - `--shell-after <step>`: Enter in a shell after the step `<step>` (For the example: after `parsec-core`).

## Create a snap

```shell
snapcraft snap
```

## Cleanup build env

```shell
snapcraft clean
```

### To install the generated snap

```shell
snap install --dangerous --classic file.snap
```

## Stop multipass VM

To stop a multipass VM, you need first to know it's name.
The following command will list the VM managed by multipass.

```shell
multipass list
```

Then to stop a vm use the sub-command `stop`.

```shell
multipass stop snapcraft-parsec
```

## Test a generate snap

```shell
snapcraft try
```

```shell
snap try --classic prime
```
