# Parsec testbed server in a nutshell

Parsec testbed server is provided as a Docker image. The instructions below explain how to build and publish a new version of the Docker image.

- [Introduction](#introduction)
- [Testbed server in the CI](#testbed-server-in-the-ci)
- [Build and Publish a new testbed server Docker image](#build-and-publish-a-new-testbed-server-docker-image)
  - [Using GitHub Action](#using-github-action)
  - [Locally](#locally)
    - [Setup](#setup)
      - [Generate a GitHub Personal Access Token](#generate-a-github-personal-access-token)
        - [How to regenerate your token](#how-to-regenerate-your-token)
      - [Setup Docker](#setup-docker)
      - [Setup Docker credential store](#setup-docker-credential-store)
        - [Setup `pass` credential store](#setup-pass-credential-store)
      - [Authenticate to Github's Container registry](#authenticate-to-githubs-container-registry)
    - [Build the testbed server Docker image](#build-the-testbed-server-docker-image)
    - [Publish the testbed server Docker image](#publish-the-testbed-server-docker-image)

## Introduction

The Parsec testbed server is used to simplify the mockup of organizations for testing purposes.

Organization templates are defined in the [testbed](https://github.com/Scille/parsec-cloud/tree/master/libparsec/crates/testbed) crate. See the [templates](https://github.com/Scille/parsec-cloud/tree/master/libparsec/crates/testbed/src/templates) directory.

Releases of `parsec-testbed-server` can be found on <https://github.com/Scille/parsec-cloud/pkgs/container/parsec-cloud%2Fparsec-testbed-server>.

## Testbed server in the CI

The CI uses one of the published testbed server Docker images to run tests.

To see which image is used, look for the `parsec-testbed-server` service in the GitHub actions.

For example, `https://github.com/Scille/parsec-cloud/blob/master/.github/workflows/ci-rust.yml`:

```yaml
    services:
      parsec-testbed-server:
        image: ghcr.io/scille/parsec-cloud/parsec-testbed-server:3.5.3-a.0.dev.20396.d489997
```

## Build and Publish a new testbed server Docker image

### Using GitHub Action

The easiest way to build and publish a new testbed image is by using the dedicated [`docker-testbed`](https://github.com/Scille/parsec-cloud/actions/workflows/docker-testbed.yml) GitHub Action.

Click on `Run workflow`, select the branch to be used (i.e. `master`) and click `Run workflow`.

### Locally

This section describes how to locally build and publish a new testbed server Docker image.

#### Setup

##### Generate a GitHub Personal Access Token

To publish a package to GitHub, you need to have a [personnel access token (PAT)](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token) with the correct permissions (scopes). Currently GitHub only allows to use PAT in classic mode (not fine-grained).

So you need to go to <https://github.com/settings/tokens/new> and:

- Set a name for the to be generated token (for example: `read-write-del-packages`).
- Set the expiration time (you can leave the default to `30 days`).
- Select `write:packages` (will include `read:packages`).
- Select `delete:packages`.

After that click on `Generate token`, it will display the generated token. **Make sure to keep it somewhere safe** (like you would do for a password).

###### How to regenerate your token

GitHub will notify you if your token is about to expire. To regenerate it:

1. Go to <https://github.com/settings/tokens>.
2. Click on the name of the token, its details will be displayed.
3. Click `Regenerate Token` to regenerate it (as before, **keep the new token safe**).

##### Setup Docker

For this step, we will consider you already have a working Docker installation (the command `docker run hello-world` work as expected). If not, follow the instructions on <https://docs.docker.com/get-docker/>.

##### Setup Docker credential store

In order to safely store your credentials, you need to setup a credential store (otherwise they are saved in plaintext).

Depending on your platform, you have the following `docker-credential-helper` alternatives:

| Name                                 | Platform   | Package                        |
| ------------------------------------ | ---------- | ------------------------------ |
| D-Bus Secret Service                 | 🐧 Linux   | docker-credential-secretservice |
| Pass                                 | 🐧 Linux   | docker-credential-pass          |
| Microsoft Windows Credential Manager | 🪟 Windows | docker-credential-wincred       |
| Apple macOS keychain                 | 🍎 macOS   | docker-credential-osxkeychain   |

Download the binary that works better for you from the [releases page](https://github.com/docker/docker-credential-helpers/releases) and put that binary in your $PATH, so Docker can find it (take a look at <https://github.com/docker/docker-credential-helpers/blob/master/README.md#installation>).

The following steps describe how to setup and use the `pass` credential store.

###### Setup `pass` credential store

Install [`pass`](https://www.passwordstore.org/) and initialize it with a GPG key (take a look at <https://wiki.archlinux.org/title/Pass>).

As specified above, download the artifact `docker-credential-pass-v{{ version }}.linux-{{ arch }}` from the [releases page](https://github.com/docker/docker-credential-helpers/releases) and add it to your $PATH so Docker can find it (for example: `~/.local/bin`). It should be renamed `docker-credential-pass`.

Check that the command is found:

```shell
$ whereis docker-credential-pass
docker-credential-pass: /home/.../.local/bin/docker-credential-pass
```

> If you get `docker-credential-pass:` with nothing after, it can mean that:
>
> - You put the binary in a folder that isn't already in the env var `PATH` (note `~/.local/bin` isn't always present by default in `PATH`).
> - You copied the binary in a correct folder but with the wrong name.

Check that the command is working:

```shell
$ docker-credential-pass version
docker-credential-pass (github.com/docker/docker-credential-helpers) v0.7.0
```

> If you get permission denied, you may need to give it execution permission `chmod +x docker-credential-pass`

Edit the Docker configuration file at `~/.docker/config.json` (you can create it if it does not exists) and set the key `credsStore` to `pass`.

```json
{
  "credsStore": "pass"
}
```

##### Authenticate to Github's Container registry

Use Docker to login to [GitHub's Container registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry) using your GitHub's username and personal access token:

```shell
$ docker login ghcr.io
Username: <USERNAME>
Password: <PAT>
Login Succeeded
```

Check that it is correctly configured:

```shell
$ docker-credential-pass list
{"ghcr.io": "<USERNAME>"}
$ pass list docker-credential-helpers
docker-credential-helpers
└── <ghcr.io in base64>
    └── <USERNAME>
```

#### Build the testbed server Docker image

To build the testbed server Docker image, run:

```shell
bash packaging/testbed-server/build.sh
```

If build succeeds, it will display the command to run the testbed docker image (look for the command after `You can now test/use the docker image with:`).

> Alternative, if you are in a bash shell, you can `source` the build script (will close if something fail, use `set +x` after).
> This will provide access to `PREFIX` and `UNIQ_TAG` environment variables so you can run the testbed Docker image as:
>
> ```shell
> docker run --publish 6777:6777 --rm --name=parsec-testbed-server $PREFIX/parsec-testbed-server:$UNIQ_TAG
> ```

#### Publish the testbed server Docker image

Once you have tested that the image works, upload it to the GitHub Container registry with the command provided by the script (look for the command after `Once You have tested that the image is working you can push it with`)

> Alternative, if you have **sourced* the build script, you can run the testbed Docker image as:
>
> ```shell
> for tag in $UNIQ_TAG; do
>   docker push $PREFIX/parsec-testbed-server:\$tag;
> done
> ```
