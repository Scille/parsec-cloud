# Testbed server in a box

Parsec provides a Docker image for the testbed server that can be used for testing.

- [Testbed server in a box](#testbed-server-in-a-box)
  - [Build and publish a new testbed server docker image](#build-and-publish-a-new-testbed-server-docker-image)
    - [Generate a github token](#generate-a-github-token)
      - [How I regenerate my token](#how-i-regenerate-my-token)
    - [Setup docker](#setup-docker)
      - [Authenticate to the github container registry](#authenticate-to-the-github-container-registry)
        - [Setup `pass` credentials store](#setup-pass-credentials-store)
    - [Build the docker image](#build-the-docker-image)
    - [Publish the image](#publish-the-image)

## Build and publish a new testbed server docker image

Here we will detail how to build and publish a new Docker image for the testbed server.

### Generate a github token

To publish a package to GitHub, you need to have a [personnel access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token) (PAT) with the correct permissions (scopes).

Currently GitHub only allow to use PAT in classic mode (not fine-grained).

So you need to go to <https://github.com/settings/tokens/new> and:

- Set a correct name for the to be generated token (Mine is called `read-write-del-packages`).
- Set the expiration time (I let the default to `30 days`).
- Select `write:packages` (will include `read:packages`).
- Select `delete:packages`.

After that click on `Generate token`, it will display the generated token. **Make sure to keep it somewhere safe** (like you would do for a password).

#### How I regenerate my token

My token is about to expire how do I regenerate it ?

To regenerate your token (you will be notified by github when one is about to expire), go to <https://github.com/settings/tokens> click on the token to regenerate (the name of the token).

This will open a page with the details of the selected token and at the top of it, we have the button `Regenerate Token` (like when generating a token, keep the new one safe).

### Setup docker

For this step, we will consider you already have a functioning docker installation (the command `docker run hello-world` work as expected).

Here, we will setup docker to be able to authenticate to the [github docker registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-docker-registry).

#### Authenticate to the github container registry

You need to setup a credential store with docker to safely store the credentials (otherwise they are saved in plaintext).
Depending on your platform, you have the following credentials store:

| Name                                 | Platform |
| ------------------------------------ | -------- |
| D-Bus Secret Service                 | 🐧 Linux  |
| Pass                                 | 🐧 Linux  |
| Microsoft Windows Credential Manager | Windows  |
| Apple macOS keychain                 | 🍎 macOS  |

You can download them from <https://github.com/docker/docker-credential-helpers/releases>.

For the setup instructions, take a look at <https://github.com/docker/docker-credential-helpers/blob/master/README.md>.

I will detail the setup for `pass` since it's the one I'm using.

##### Setup `pass` credentials store

We consider you already installed [`pass`](https://www.passwordstore.org/) and initialized `pass` with a gpg key (you can take a look at <https://wiki.archlinux.org/title/Pass>).

Download the artifact `docker-credential-pass-v{{ version }}.linux-{{ arch }}` at <https://github.com/docker/docker-credential-helpers/releases/latest>

After you complete the download, you need to install the binary somewhere accessible in your path.
I put mine in `~/.local/bin`, so I have the following output with the command `whereis docker-credential-pass`

```shell
$ whereis docker-credential-pass
docker-credential-pass: /home/.../.local/bin/docker-credential-pass
```

> If you have `docker-credential-pass:` with nothing after, it can mean:
>
> - You put the binary in a folder that isn't already in the env var `PATH` (note `~/.local/bin` isn't always present by default in `PATH`).
> - You copied the binary in a correct folder but with the wrong name.

And the command `docker-credential-pass version` work (you may need to give it the execution permission with `chmod +x docker-credential-pass`):

```shell
$ docker-credential-pass version
docker-credential-pass (github.com/docker/docker-credential-helpers) v0.7.0
```

Modify the docker config file at `~/.docker/config.json`, you need to set the key `credsStore` to `pass`.

```json
{
  "credsStore": "pass"
}
```

Then excute `docker login`

```shell
$ docker login ghcr.io
Username: <USERNAME>
Password: <PAT>
Login Succeeded
```

You can then verify, that you correctly configured docker with

```shell
$ docker-credential-pass list
{"ghcr.io": "<USERNAME>"}
$ pass list docker-credential-helpers
docker-credential-helpers
└── <ghcr.io in base64>
    └── <USERNAME>
```

### Build the docker image

To build the docker image, simply execute

```shell
bash packaging/testbed-server/build.sh
```

> If you are adventurous, you can replace the `bash` by `source` if you're already in a bash shell (will close if something fail, use `set +x` after).
> That will provide access to the env var `PREFIX` & `UNIQ_TAG`.

The script will give you the instruction to run the generated docker image (look for the command after `You can now test/use the docker image with:`).

```shell
docker run --publish 6777:6777 --rm --name=parsec-testbed-server $PREFIX/parsec-testbed-server:$UNIQ_TAG
```

> The env var `PREFIX` & `UNIQ_TAG` are only available if you used `source` instead of `bash`, otherwise use the command displayed by the script.

### Publish the image

Once you have tested that the image works, upload it to the GitHub container registry with the command provided by the script (look for the command after `Once You have tested that the image is working you can push it with`)

```shell
for tag in $UNIQ_TAG; do
  docker push $PREFIX/parsec-testbed-server:\$tag;
done
```

> The env var `PREFIX` & `UNIQ_TAG` are only available if you used `source` instead of `bash`, otherwise use the command displayed by the script.
