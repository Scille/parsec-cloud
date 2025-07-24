# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import functools
from pathlib import Path

from jinja2 import (
    Environment,
    FileSystemLoader,
    StrictUndefined,
)
from jinja2 import PackageLoader as JinjaPackageLoader


def get_environment(template_dir: Path | None) -> Environment:
    environment_builder = functools.partial(Environment, undefined=StrictUndefined)
    if template_dir is None:
        return environment_builder(loader=JinjaPackageLoader("parsec", "templates"))
    else:
        return environment_builder(loader=FileSystemLoader(template_dir))
