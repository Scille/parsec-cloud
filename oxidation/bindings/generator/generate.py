# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from __future__ import annotations

from types import ModuleType
from typing import Union, Dict, List, OrderedDict, Type, Optional
from importlib import import_module
from collections import namedtuple
import argparse
from dataclasses import dataclass
from jinja2 import Environment, FileSystemLoader, StrictUndefined
from pathlib import Path
from inspect import isfunction, isclass, iscoroutinefunction, signature


BASEDIR = Path(__file__).parent


# Meta-types are defined in the api module for simplicity, `generate_api_specs` will take care of retreiving them
META_TYPES = [
    "Result",
    "Ref",
    "StrBasedType",
    "I32BasedType",
    "Variant",
    "Structure",
    "OnClientEventCallback",
]
Result: Type = None
Ref: Type = None
StrBasedType: Type = None
I32BasedType: Type = None
Variant: Type = None
Structure: Type = None
OnClientEventCallback: Type = None


env = Environment(
    loader=FileSystemLoader(BASEDIR / "templates"),
    autoescape=False,
    trim_blocks=True,
    lstrip_blocks=False,
    extensions=["jinja2.ext.loopcontrols"],
    undefined=StrictUndefined,
    keep_trailing_newline=True,
)


def snake_case_to_camel_case(s: str) -> str:
    camel = ""
    next_is_uppercase = False
    for c in s:
        if c == "_":
            next_is_uppercase = True
            continue
        if next_is_uppercase:
            camel += c.upper()
            next_is_uppercase = False
        else:
            camel += c
    return camel


env.filters["snake2camel"] = snake_case_to_camel_case


def _raise_helper(msg):
    raise RuntimeError(msg)


env.globals["raise"] = _raise_helper


class BaseTypeInUse:
    # Name of the type familly (e.g. `bytes`, `struct`, `variant`), to be defined in subclass
    kind: str

    @staticmethod
    def parse(param: str) -> "BaseTypeInUse":
        assert not isinstance(
            param, str
        ), f"Bad param `{param!r}`, passing type as string is not supported"
        origin = getattr(param, "__origin__", None)
        if origin is Union:  # Python resolves `Optional[Foo]` as `Union[Foo, None]`
            assert len(param.__args__) == 2 and type(None) in param.__args__
            elem_param = next(x for x in param.__args__ if x is not type(None))  # noqa
            return OptionalTypeInUse(BaseTypeInUse.parse(elem_param))

        elif origin in (list, List):
            assert len(param.__args__) == 1
            return ListTypeInUse(BaseTypeInUse.parse(param.__args__[0]))

        elif origin is Result:
            assert len(param.__args__) == 2
            ok_param, err_param = param.__args__
            return ResultTypeInUse(
                ok=BaseTypeInUse.parse(ok_param),
                err=BaseTypeInUse.parse(err_param),
            )

        elif origin is Ref:
            assert len(param.__args__) == 1
            return RefTypeInUse(elem=BaseTypeInUse.parse(param.__args__[0]))

        elif param is OnClientEventCallback:
            spec = OpaqueSpec(kind="OnClientEventCallback")
            # Ugly hack to make template rendering happy
            spec.event_type = namedtuple("Type", "name")("ClientEvent")

            # spec.event_type = param.__orig_bases__[0].__args__[0]
            return spec

        elif isinstance(param, type) and issubclass(param, StrBasedType):
            return StrBasedTypeInUse(
                name=param.__name__,
                custom_from_rs_string=getattr(param, "custom_from_rs_string", None),
                custom_to_rs_string=getattr(param, "custom_to_rs_string", None),
            )

        elif isinstance(param, type) and issubclass(param, I32BasedType):
            return I32BasedTypeInUse(name=param.__name__)

        else:
            typespec = TYPESDB.get(param)
            assert typespec is not None, f"Bad param `{param!r}`, not a scalar/variant/struct"
            return typespec


@dataclass
class OptionalTypeInUse(BaseTypeInUse):
    kind = "optional"
    elem: BaseTypeInUse


@dataclass
class ListTypeInUse(BaseTypeInUse):
    kind = "list"
    elem: BaseTypeInUse


@dataclass
class StructSpec(BaseTypeInUse):
    kind = "struct"
    name: str
    attributes: OrderedDict[str, BaseTypeInUse]


@dataclass
class VariantSpec(BaseTypeInUse):
    kind = "variant"
    name: str
    values: List["StructSpec"]


@dataclass
class ResultTypeInUse(BaseTypeInUse):
    kind = "result"
    ok: BaseTypeInUse
    err: BaseTypeInUse


@dataclass
class RefTypeInUse(BaseTypeInUse):
    kind = "ref"
    elem: BaseTypeInUse


@dataclass
class StrBasedTypeInUse(BaseTypeInUse):
    kind = "str_based"
    name: str

    # If set, custom_from_rs_string/custom_to_rs_string should inlined rust functions
    # `fn (String) -> Result<X, AsRef<str>>`
    custom_from_rs_string: str | None = None
    # `fn (&X) -> Result<String, AsRef<str>>`
    custom_to_rs_string: str | None = None


@dataclass
class I32BasedTypeInUse(BaseTypeInUse):
    kind = "i32_based"
    name: str


@dataclass
class OpaqueSpec(BaseTypeInUse):
    kind: str


@dataclass
class MethSpec:
    name: str
    params: OrderedDict[str, BaseTypeInUse]
    return_type: Optional[BaseTypeInUse]
    is_async: bool


@dataclass
class ApiSpecs:
    str_based_types: List[StrBasedType]
    i32_based_types: List[I32BasedType]
    meths: List[MethSpec]
    structs: List[StructSpec]
    variants: List[VariantSpec]
    rust_code_to_inject: Optional[str]  # Hack for the dummy test api


# TYPESDB uses as key the type used in the api definition and as value the corresponding spec object that will
# be used in template generation.
# For instance, if we have `def foo() -> Bar: ...` in the api:
# The `Bar` object is going to be a key in TYPESDB, and the value will be the `StructSpec` built by introspecting `Bar`
TYPESDB: Dict[type, Union[OpaqueSpec, StructSpec, VariantSpec]] = {
    bool: OpaqueSpec(kind="bool"),
    float: OpaqueSpec(kind="float"),
    str: OpaqueSpec(kind="str"),
    bytes: OpaqueSpec(kind="bytes"),
    # TODO: support datetime !
    # This will be completed by the variants/structures during api parsing
}


def generate_api_specs(api_module: ModuleType) -> ApiSpecs:
    api_items = {}

    # Use `module.__dict__` instead of `dir(module)` to preserve the definition order
    for item_name, item in api_module.__dict__.items():
        # Special case to retreive the meta types
        if item_name in META_TYPES:
            globals()[item_name] = getattr(api_module, item_name)
            continue
        # Ignore items that have been imported from another module
        if not getattr(item, "__module__", "").startswith(api_module.__name__):
            continue
        api_items[item_name] = item

    # The variant/struct types we define for the api can be recursive and/or
    # refere other types defined later on.
    # Hence we do the parsing in two passes:
    # - First we populate `TYPEDB` by collecting the variant/struct types and storing them as placeholders
    # - Then we do the actual type parsing and replace the placeholders

    # First pass
    class ParsingPlaceholder:
        pass

    for item_name, item in api_items.items():
        if isclass(item) and issubclass(item, (Variant, Structure)):
            TYPESDB[item] = ParsingPlaceholder()

    # Second pass
    variants = []
    structs = []
    for item_name, item in api_items.items():
        if isclass(item) and issubclass(item, Variant):
            placeholder = TYPESDB[item]
            # Variant values types are special kind of structures: the are never referenced directly (the variant type
            # must be referenced instead) so we don't need to add it to TYPESDB (hence we wait for 2nd pass to treat them)
            variant_values = []
            for variant_val_name in dir(item):
                if variant_val_name.startswith("_"):
                    continue
                variant_val_type = getattr(item, variant_val_name)
                value_spec = StructSpec(
                    name=variant_val_name,
                    attributes={
                        k: BaseTypeInUse.parse(v)
                        for k, v in getattr(variant_val_type, "__annotations__", {}).items()
                    },
                )
                variant_values.append(value_spec)

            variant = VariantSpec(
                name=item_name,
                values=variant_values,
            )
            # Modify placeholder instead of replacing it given it is referenced in the nested specs
            placeholder.__dict__ = variant.__dict__
            placeholder.__class__ = variant.__class__
            variants.append(placeholder)

        elif isclass(item) and issubclass(item, Structure):
            placeholder = TYPESDB[item]
            struct = StructSpec(
                name=item_name,
                attributes={
                    k: BaseTypeInUse.parse(v)
                    for k, v in getattr(item, "__annotations__", {}).items()
                },
            )
            # Modify placeholder instead of replacing it given it is referenced in the nested specs
            placeholder.__dict__ = struct.__dict__
            placeholder.__class__ = struct.__class__
            structs.append(placeholder)

    # Make sure all types have a unique name, this is not strictly required but it is very convenient when testing type in the template
    reserved = {}
    for k, v in TYPESDB.items():
        name = getattr(v, "name", v.kind)
        assert name not in reserved
        setted = reserved.setdefault(name, v)
        if setted is not v:
            raise RuntimeError(f"Multiple types with same name `{name}`: `{setted}` vs `{v}`")
    del reserved

    # Finally retreive the api functions, nothing fancy here
    meths = []
    for item_name, item in api_items.items():
        if isfunction(item):
            s = signature(item)
            assert all(x.default is x.empty for x in s.parameters.values())  # Sanity checks
            meths.append(
                MethSpec(
                    name=item_name,
                    params={k: BaseTypeInUse.parse(v.annotation) for k, v in s.parameters.items()},
                    return_type=None
                    if s.return_annotation in (None, s.empty)
                    else BaseTypeInUse.parse(s.return_annotation),
                    is_async=iscoroutinefunction(item),
                )
            )

    return ApiSpecs(
        str_based_types=[
            item.__name__
            for item in api_items.values()
            if isinstance(item, type) and issubclass(item, StrBasedType)
        ],
        i32_based_types=[
            item.__name__
            for item in api_items.values()
            if isinstance(item, type) and issubclass(item, I32BasedType)
        ],
        variants=variants,
        structs=structs,
        meths=meths,
        rust_code_to_inject=getattr(api_module, "BINDING_ELECTRON_METHS_INJECT_RUST_CODE", None),
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate bindings code")
    parser.add_argument(
        "what",
        choices=["all", "client", "electron", "web", "android"],
    )
    parser.add_argument("--test", action="store_true")
    args = parser.parse_args()

    if args.test:
        api_module = import_module("test_api")
    else:
        api_module = import_module("api")
    api_specs = generate_api_specs(api_module)

    if args.what in ("all", "client"):
        template = env.get_template("client_plugin_definitions.d.ts.j2")
        output = (BASEDIR / "../../client/src/plugins/libparsec/definitions.d.ts").resolve()
        print(f"Generating {output}")
        # Don't use `write_text` given it outputs \r\n for newlines on Windows
        output.write_bytes(template.render(api=api_specs).encode("utf8"))

    if args.what in ("all", "electron"):
        template = env.get_template("binding_electron_index.d.ts.j2")
        output = (BASEDIR / "../electron/src/index.d.ts").resolve()
        print(f"Generating {output}")
        output.write_bytes(template.render(api=api_specs).encode("utf8"))

        template = env.get_template("binding_electron_meths.rs.j2")
        output = (BASEDIR / "../electron/src/meths.rs").resolve()
        print(f"Generating {output}")
        output.write_bytes(template.render(api=api_specs).encode("utf8"))

    if args.what in ("all", "web"):
        template = env.get_template("binding_web_meths.rs.j2")
        output = (BASEDIR / "../web/src/meths.rs").resolve()
        print(f"Generating {output}")
        output.write_bytes(template.render(api=api_specs).encode("utf8"))

    if args.what in ("all", "android"):
        # TODO !
        pass
