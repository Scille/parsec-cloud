# Notary project

Notary project is a tool to simplify adding Github's [Issues][issue] & [Pull Requests][pull-request]
to a [Github Project][github-project]

[issue]: https://docs.github.com/en/issues/tracking-your-work-with-issues/about-issues
[pull-request]: https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests
[github-project]: https://docs.github.com/en/issues/trying-out-the-new-projects-experience/about-projects

## Installation

Before using `notary project`, you need to install [Github CLI][github-cli], you can found the instruction [here][github-cli-install]

[github-cli]: https://cli.github.com/
[github-cli-install]: https://github.com/cli/cli#installation

After the installation, you need to authenticate with the _Github CLI_ using the following command

```shell
gh auth login --scopes "project"
```

> We need to add the scopes `project` to generate a token that have
> the read/write permission on a _Github Project_

## Information required before running the script

Before to run `Notary Project`, you'll need the following information:

- the organization name in lowercase
- the project id/number

> When you access a _Github Project_
>
> The URL look like `https://github.com/orgs/<organization>/projects/<project_id>`
>
> - `<organization>` is the organization name (may not be in lowercase)
> - `<project_id>` is the project id

## Running `Notary Project`

To run the `notary project`, you execute the following command

```shell
bash .github/scripts/notary/script.sh <organization> <project_id>
```

> Replace `<organization>` by your organization name
> and `<project_id>` by your project ID

Example output:

```shell
$ bash .github/scripts/notary/script.sh <organization> <project_id>
[...]
{"id":"[redacted]","number":[redacted],"title":"Foo bar zoo","type":"pr"}
Adding pr "Foo bar zoo" to project Parsec-cloud/issues
{"data":{"addProjectV2ItemById":{"item":{"id":"[redacted]"}}}}
```
