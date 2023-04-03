# 🚧 Rust protocol schema testing 🚧

From [ISSUE-2834](https://github.com/Scille/parsec-cloud/issues/2834)

- [🚧 Rust protocol schema testing 🚧](#-rust-protocol-schema-testing-)
  - [Rust protocol schema testing](#rust-protocol-schema-testing)
    - [TODOs](#todos)
    - [Assumption](#assumption)
    - [Change in the json schemes](#change-in-the-json-schemes)
      - [For protocol scheme](#for-protocol-scheme)
        - [Example protocol scheme](#example-protocol-scheme)
      - [For type scheme](#for-type-scheme)
        - [Example type scheme](#example-type-scheme)
  - [Testing the scheme](#testing-the-scheme)
    - [Testing retro-compatibility](#testing-retro-compatibility)
      - [Testing retro-compatibility on major versions (`MAJOR-ENEMIES`)](#testing-retro-compatibility-on-major-versions-major-enemies)
      - [Testing retro-compatibility on minor versions (`MINOR-ALLIES`)](#testing-retro-compatibility-on-minor-versions-minor-allies)
    - [Testing multiple unreleased version (`SQUASH-UNRELEASED`)](#testing-multiple-unreleased-version-squash-unreleased)
    - [Testing readonly older released version (`STABLE-RELEASE`)](#testing-readonly-older-released-version-stable-release)
    - [Testing `introduced_in`](#testing-introduced_in)

## Rust protocol schema testing

> How to test if the change made to the protocol schemes (`oxidation/libparsec/crates/protocol/schema/**.json`) is valid ?

- Testing retro-compatibility.
  - Ensure that major versions are indeed not compatible together (i.e.: `v1` against `v2`).
  - Ensure that minor versions are compatible to each other (i.e.: `v1.1` with `v1.3`).
- Testing if we aren't creating multiple unreleased version.
  > Multiple unreleased version **MUST** be squashed together
- Testing if we aren't editing an already released protocol version.

### TODOs

- [x] [ISSUE-3074](https://github.com/Scille/parsec-cloud/issues/3074)
- [ ] [ISSUE-3075](https://github.com/Scille/parsec-cloud/issues/3075)
- [ ] Add checker for previous python scheme against rust one
- [ ] Add checker for `MAJOR-ENEMIES`
- [ ] Add checker for `MINOR-ALLIES`
- [ ] Add checker for `SQUASH-UNRELEASED`
- [ ] Add checker for `STABLE-RELEASE`
- [ ] Add checker for `introduced_in`

### Assumption

1. Major/Minor versions are incremental (i.e.: we can't have a version `3` if the version `2` don't exist. Same if version `1` don't exist)

### Change in the json schemes

Some change are required in the json schemes:

- Major revisions of the same command will be contained in the same file in an array.
  > To simplify the future change, all commands will be put in the an array.
- Introduce field `majors_versions`
- New field `introduced_in` that replace the field `introduced_in_revision`

#### For protocol scheme

```typescript
type ProtocolScheme = {
    label: string,
    major_versions: number[],
    introduced_in?: MajorMinorString,
    req: Request,
    reps: Responses,
    nested_types?: NestedTypes,
}[]

type Request = RequestWithFields | RequestUnit;

interface RequestWithFields {
    cmd: string,
    other_fields: Fields,
}

interface RequestUnit {
    cmd: string,
    other_fields: Fields,
}

interface Fields {
    [name: string]: {
        type: string,
        introduced_in?: MajorMinorString,
    }
}

interface Responses {
    [status: string]: |
    {
        other_fields: Fields,
    } |
    {
        unit: string
    }
}

interface NestedTypes {
    [label: string]: |
    // either a enum
    {
        discriminant_field: string,
        variants: Variants,
    } |
    // or a struct
    {
        fields: Fields
    }
}

interface Variants {
    [name: string]: {
        discriminant_value: string,
        fields: Fields,
    }
}

type MajorMinorString = `${Major}.${Minor}`
type Major = number
type Minor = number
```

##### Example protocol scheme

```json
[
    {
        "label": "FooBar",
        "major_versions": [
            1,
            2
        ],
        "req": {
            "cmd": "foo_bar",
            "other_fields": {
                "human_handle": {
                    "type": "Option<HumanHandle>",
                    "introduced_in": "1.1"
                }
            }
        },
        "reps": {
            "ok": {
                "other_fields": {}
            }
        }
    },
    {
        "label": "FooBar",
        "major_versions": [
            3
        ],
        "req": {
            "cmd": "foo_bar",
            "other_fields": {
                "human_handle": {
                    "type": "HumanHandle"
                }
            }
        },
        "reps": {
            "ok": {
                "other_fields": {}
            },
            "error": {
                "other_fields": {
                    "reason": {
                        "type": "String"
                    }
                }
            }
        }
    }
]
```

From the example above, we have the field `human_handle` that is present on versions `>=1.1` (including `2.*`)

#### For type scheme

```typescript
type TypeScheme = {
    label: string,
    major_versions: number[],
    introduced_in?: MajorMinorString,
    other_fields: Fields
}[]
```

##### Example type scheme

```jsonc
[
    {
        "label": "FooBarType",
        "major_versions": [
            1,
            2,
        ],
        "other_fields": {
            "bar": {
                "type": "Int64"
            },
            "foo": {
                "type": "FooType",
                "introduced_in": "1.1"
            }
        }
    }
]
```

From the example above, we have the field `foo` that's is present on versions `>=1.1` (including `2.*`)

## Testing the scheme

### Testing retro-compatibility

We want to verify how a new version is compared to the previous versions.

#### Testing retro-compatibility on major versions (`MAJOR-ENEMIES`)

When creating a new major version e.g. `2.0`, we **MUST** check if this version is not compatible with the previous version `1.*`

#### Testing retro-compatibility on minor versions (`MINOR-ALLIES`)

When creating a new minor version e.g. `1.3`, we **MUST** check if this version is compatible with the previous version `1.2`.

### Testing multiple unreleased version (`SQUASH-UNRELEASED`)

During the development process, we may need to edit the api protocol but we don't want to have multiple *unreleased version*.

If we haven't release the version `2.1`, we don't need to create the version `2.2`.

### Testing readonly older released version (`STABLE-RELEASE`)

We want to check if we aren't editing a previously released api version.

For that we will need to have a list of *released versions* and check if those weren't edited

### Testing `introduced_in`

We **MUST** check the value in `introduced_in`, the value must respond to the following criteria:

1. The value **MUST** be valid for the type `MajorMinorString`
2. The `major` part **MUST** be listed in `major_versions`
3. The `minor` part **MUST** be `>0`
