# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import argparse
import subprocess
from collections import OrderedDict, namedtuple
from dataclasses import dataclass
from importlib import import_module
from inspect import isclass, iscoroutinefunction, isfunction, signature
from pathlib import Path
from types import ModuleType
from typing import Any, List, Tuple, Union

from jinja2 import Environment, FileSystemLoader, StrictUndefined

BASEDIR = Path(__file__).parent


# Meta-types are defined in the api module for simplicity, `generate_api_specs` will take care of retrieving them
META_TYPES = [
    "Result",
    "Ref",
    "StrBasedType",
    "BytesBasedType",
    "I32BasedType",
    "U32BasedType",
    "Variant",
    "VariantItemTuple",
    "VariantItemUnit",
    "ErrorVariant",
    "Structure",
    "OnClientEventCallback",
]


class Result:
    ...


class Ref:
    ...


class StrBasedType(str):
    ...


class BytesBasedType(bytes):
    ...


class I32BasedType(int):
    ...


class U32BasedType(int):
    ...


class Variant:
    ...


class VariantItemTuple:
    ...


class VariantItemUnit:
    ...


class ErrorVariant:
    ...


class Structure:
    ...


class OnClientEventCallback:
    ...


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


def _raise_helper(msg: Any) -> None:
    raise RuntimeError(msg)


env.globals["raise"] = _raise_helper


class BaseTypeInUse:
    # Name of the type family (e.g. `bytes`, `struct`, `variant`), to be defined in subclass
    kind: str

    @staticmethod
    def parse(param: str) -> "BaseTypeInUse":
        assert not isinstance(
            param, str
        ), f"Bad param `{param!r}`, passing type as string is not supported"
        origin = getattr(param, "__origin__", None)
        if origin is Union:  # Python resolves `Optional[Foo]` as `Union[Foo, None]`
            assert len(param.__args__) == 2 and type(None) in param.__args__
            elem_param = next(x for x in param.__args__ if x is not type(None))
            return OptionalTypeInUse(BaseTypeInUse.parse(elem_param))

        elif origin in (list, List):
            assert len(param.__args__) == 1
            return ListTypeInUse(BaseTypeInUse.parse(param.__args__[0]))

        elif origin in (tuple, Tuple):
            return TupleTypeInUse([BaseTypeInUse.parse(x) for x in param.__args__])

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

        elif param is type(None):
            return NoneTypeInUse()

        elif isinstance(param, type) and issubclass(param, StrBasedType):
            return StrBasedTypeInUse(
                name=param.__name__,
                custom_from_rs_string=getattr(param, "custom_from_rs_string", None),
                custom_to_rs_string=getattr(param, "custom_to_rs_string", None),
            )

        elif isinstance(param, type) and issubclass(param, BytesBasedType):
            return BytesBasedTypeInUse(
                name=param.__name__,
                custom_from_rs_bytes=getattr(param, "custom_from_rs_bytes", None),
                custom_to_rs_bytes=getattr(param, "custom_to_rs_bytes", None),
            )

        elif isinstance(param, type) and issubclass(param, I32BasedType):
            return I32BasedTypeInUse(name=param.__name__)

        elif isinstance(param, type) and issubclass(param, U32BasedType):
            return U32BasedTypeInUse(name=param.__name__)

        else:
            typespec = TYPES_DB.get(param)
            assert typespec is not None, f"Bad param `{param!r}`, not a scalar/variant/struct"
            return typespec


@dataclass
class NoneTypeInUse(BaseTypeInUse):
    kind = "none"


@dataclass
class OptionalTypeInUse(BaseTypeInUse):
    kind = "optional"
    elem: BaseTypeInUse


@dataclass
class ListTypeInUse(BaseTypeInUse):
    kind = "list"
    elem: BaseTypeInUse


@dataclass
class TupleTypeInUse(BaseTypeInUse):
    kind = "tuple"
    values: List[BaseTypeInUse]


@dataclass
class StructSpec(BaseTypeInUse):
    kind = "struct"
    name: str
    attributes: OrderedDict[str, BaseTypeInUse]
    custom_getters: dict[str, str]
    custom_init: dict[str, str] | None

    def get_value(self, obj_name: str, attr_name: str) -> str | None:
        value = self.custom_getters.get(attr_name, None)
        if value is not None:
            return value.format(input=obj_name)
        else:
            return f"{obj_name}.{attr_name}"

    def list_attributes(self) -> str:
        if len(self.attributes) == 0:
            return ""
        return ",".join(self.attributes.keys()) + ","


@dataclass
class VariantItemSpec(BaseTypeInUse):
    name: str
    is_tuple: bool = False
    is_unit: bool = False
    is_struct: bool = False
    tuple: List[BaseTypeInUse] | None = None
    struct: StructSpec | None = None


@dataclass
class VariantSpec(BaseTypeInUse):
    kind = "variant"
    name: str
    values: List[VariantItemSpec]
    is_error_variant: bool


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
class BytesBasedTypeInUse(BaseTypeInUse):
    kind = "bytes_based"
    name: str

    # If set, custom_from_rs_bytes/custom_to_rs_bytes should inlined rust functions
    # `fn (&[u8]) -> Result<X, AsRef<str>>`
    custom_from_rs_bytes: str | None = None
    # `fn (&X) -> Result<Vec<u8>, AsRef<str>>`
    custom_to_rs_bytes: str | None = None


@dataclass
class I32BasedTypeInUse(BaseTypeInUse):
    kind = "i32_based"
    name: str


@dataclass
class U32BasedTypeInUse(BaseTypeInUse):
    kind = "u32_based"
    name: str


@dataclass
class OpaqueSpec(BaseTypeInUse):
    kind: str


@dataclass
class MethSpec:
    name: str
    params: OrderedDict[str, BaseTypeInUse]
    return_type: BaseTypeInUse | None
    is_async: bool


@dataclass
class ApiSpecs:
    str_based_types: List[str]
    bytes_based_types: List[str]
    i32_based_types: List[str]
    u32_based_types: List[str]
    meths: List[MethSpec]
    structs: List[StructSpec]
    variants: List[VariantSpec]
    rust_code_to_inject: str | None  # Hack for the dummy test api


# TYPES_DB uses as key the type used in the api definition and as value the corresponding spec object that will
# be used in template generation.
# For instance, if we have `def foo() -> Bar: ...` in the api:
# The `Bar` object is going to be a key in TYPES_DB, and the value will be the `StructSpec` built by introspecting `Bar`
TYPES_DB: dict[type, OpaqueSpec | StructSpec | VariantSpec] = {
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
        # Special case to retrieve the meta types
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
    # - First we populate `TYPES_DB` by collecting the variant/struct types and storing them as placeholders
    # - Then we do the actual type parsing and replace the placeholders

    # First pass
    class ParsingPlaceholder:
        pass

    for item_name, item in api_items.items():
        if isclass(item) and issubclass(item, (Variant, Structure)):
            TYPES_DB[item] = ParsingPlaceholder()  # type: ignore[assignment]

    # Second pass
    variants: list[VariantSpec] = []
    structs: list[StructSpec] = []
    for item_name, item in api_items.items():
        if isclass(item) and issubclass(item, Variant):
            placeholder = TYPES_DB[item]
            # Variant values types are special kind of structures: the are never referenced directly (the variant type
            # must be referenced instead) so we don't need to add it to TYPES_DB (hence we wait for 2nd pass to treat them)
            variant_values = []
            for variant_val_name in dir(item):
                if variant_val_name.startswith("_"):
                    continue
                variant_val_type = getattr(item, variant_val_name)

                if isinstance(variant_val_type, VariantItemUnit):
                    value_spec = VariantItemSpec(name=variant_val_name, is_unit=True)

                elif isinstance(variant_val_type, VariantItemTuple):
                    items = getattr(variant_val_type, "items", [])
                    value_spec = VariantItemSpec(
                        name=variant_val_name,
                        is_tuple=True,
                        tuple=[BaseTypeInUse.parse(v) for v in items],
                    )

                else:
                    value_spec = VariantItemSpec(
                        name=variant_val_name,
                        is_struct=True,
                        struct=StructSpec(
                            name=variant_val_name,
                            attributes=OrderedDict(
                                (k, BaseTypeInUse.parse(v))
                                for k, v in getattr(variant_val_type, "__annotations__", {}).items()
                            ),
                            custom_getters={},
                            custom_init=None,
                        ),
                    )

                variant_values.append(value_spec)

            variant = VariantSpec(
                name=item_name,
                values=variant_values,
                is_error_variant=issubclass(item, ErrorVariant),
            )
            # Modify placeholder instead of replacing it given it is referenced in the nested specs
            placeholder.__dict__ = variant.__dict__
            placeholder.__class__ = variant.__class__
            variants.append(placeholder)  # type: ignore[arg-type]

        elif isclass(item) and issubclass(item, Structure):
            placeholder = TYPES_DB[item]
            annotations = getattr(item, "__annotations__", {})
            struct = StructSpec(
                name=item_name,
                attributes=OrderedDict(
                    (k, BaseTypeInUse.parse(v))
                    for k, v in annotations.items()
                    if k not in ("custom_getters", "custom_init")
                ),
                custom_getters=getattr(item, "custom_getters", {}),
                custom_init=getattr(item, "custom_init", None),
            )
            # Modify placeholder instead of replacing it given it is referenced in the nested specs
            placeholder.__dict__ = struct.__dict__
            placeholder.__class__ = struct.__class__
            structs.append(placeholder)  # type: ignore[arg-type]

    # Make sure all types have a unique name, this is not strictly required but it is very convenient when testing type in the template
    reserved: dict[Any, OpaqueSpec | StructSpec | VariantSpec] = {}
    for k, v in TYPES_DB.items():
        name = getattr(v, "name", v.kind)
        assert name not in reserved
        setted = reserved.setdefault(name, v)
        if setted is not v:
            raise RuntimeError(f"Multiple types with same name `{name}`: `{setted}` vs `{v}`")
    del reserved

    # Finally retrieve the api functions, nothing fancy here
    meths = []
    for item_name, item in api_items.items():
        if isfunction(item):
            s = signature(item)
            assert all(x.default is x.empty for x in s.parameters.values())  # Sanity checks
            meths.append(
                MethSpec(
                    name=item_name,
                    params=OrderedDict(
                        (k, BaseTypeInUse.parse(v.annotation)) for k, v in s.parameters.items()
                    ),
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
        bytes_based_types=[
            item.__name__
            for item in api_items.values()
            if isinstance(item, type) and issubclass(item, BytesBasedType)
        ],
        i32_based_types=[
            item.__name__
            for item in api_items.values()
            if isinstance(item, type) and issubclass(item, I32BasedType)
        ],
        u32_based_types=[
            item.__name__
            for item in api_items.values()
            if isinstance(item, type) and issubclass(item, U32BasedType)
        ],
        variants=variants,
        structs=structs,
        meths=meths,
        rust_code_to_inject=getattr(api_module, "BINDING_ELECTRON_METHS_INJECT_RUST_CODE", None),
    )


def generate_client(api_specs: ApiSpecs) -> list[str]:
    template = env.get_template("client_plugin_definitions.d.ts.j2")
    output = (BASEDIR / "../../client/src/plugins/libparsec/definitions.d.ts").resolve()
    print(f"Generating {output}")
    # Don't use `write_text` given it outputs \r\n for newlines on Windows
    output.write_bytes(template.render(api=api_specs).encode("utf8"))
    return []


def generate_electron(api_specs: ApiSpecs) -> list[str]:
    template = env.get_template("binding_electron_meths.rs.j2")
    output = (BASEDIR / "../electron/src/meths.rs").resolve()
    print(f"Generating {output}")
    output.write_bytes(template.render(api=api_specs).encode("utf8"))

    return ["libparsec_bindings_electron"]


def generate_electron_client(api_specs: ApiSpecs) -> list[str]:
    template = env.get_template("binding_electron_index.d.ts.j2")
    output = (BASEDIR / "../electron/src/index.d.ts").resolve()
    print(f"Generating {output}")
    output.write_bytes(template.render(api=api_specs).encode("utf8"))
    return []


def generate_web(api_specs: ApiSpecs) -> list[str]:
    template = env.get_template("binding_web_meths.rs.j2")
    output = (BASEDIR / "../web/src/meths.rs").resolve()
    print(f"Generating {output}")
    output.write_bytes(template.render(api=api_specs).encode("utf8"))

    return ["libparsec_bindings_web"]


def generate(what: str, api_specs: ApiSpecs) -> list[str]:
    if what == "client":
        return generate_client(api_specs)
    elif what == "electron":
        return generate_electron(api_specs)
    elif what == "electron_client":
        return generate_electron_client(api_specs)
    elif what == "web":
        return generate_web(api_specs)
    elif what == "android":
        # TODO !
        raise NotImplementedError("Android isn't ready yet")
    else:
        raise ValueError(f"Unknown generator `{what}`")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate bindings code")
    parser.add_argument(
        "what",
        choices=["all", "client", "electron", "electron_client", "web", "android"],
        nargs="+",
    )
    parser.add_argument("--test", action="store_true")
    args = parser.parse_args()

    if args.test:
        api_module = import_module("test_api")
    else:
        api_module = import_module("api")
    api_specs = generate_api_specs(api_module)

    rust_modified_packages = []

    if "all" in args.what:
        rust_modified_packages.extend(
            generate_client(api_specs) + generate_electron(api_specs) + generate_web(api_specs)
        )
    else:
        for what in args.what:
            rust_modified_packages.extend(generate(what, api_specs))

    subprocess.check_call(
        args=["cargo", "fmt"] + [f"--package={package}" for package in rust_modified_packages],
        cwd=BASEDIR / "../..",
    )
