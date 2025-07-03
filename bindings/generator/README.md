# Libparsec bindings generator

This directory contains code to generate libparsec bindings for different platforms.

To avoid errors and the tedious work of writing bindings code, a Python script is used.

## How does it work?

The `generate.py` script generates bindings code for all platforms based on:

- The types and functions described in the `api` directory
- The code templates (jinja2) from the `templates` directory

Simply run the `generate.py` script specifying the platform (or `all` for all platforms):

> [!NOTE]
> The dependencies needed to run `generate.py` are installed via Poetry together with Parsec server
> dependencies (see [/server/README.md](../../server/README.md)).

```shell
python generate.py electron web
```

## Updating the bindings

Update the files in the `api` folder according to your needs (adding, updating or removing classes
or functions) and then run the `generate.py` script for all platforms.

Generated bindings are updated in-place, so you can just commit the changed files from the bindings directory (as well as `client/src/plugins/libparsec/definitions.ts`).

> [!NOTE]
> Hopefully, you shouldn't need to update `templates` because once they are functional for
> a platform they should remain functional 🤞.
