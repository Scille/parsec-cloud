# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations
from typing import Any, Callable, Tuple

from jinja2 import Environment, BaseLoader, TemplateNotFound, StrictUndefined, Template
import importlib.resources


class PackageLoader(BaseLoader):
    def __init__(self, path: str) -> None:
        self.path = path

    def get_source(self, environment: Any, template: str) -> Tuple[str, str, Callable[[], bool]]:
        from parsec.backend import templates  # Self import \o/

        try:
            source = importlib.resources.files(templates).joinpath(template).read_text()
        except FileNotFoundError as exc:
            raise TemplateNotFound(template) from exc
        return source, self.path, lambda: True


# Env config is also needed to configure the ASGI app
JINJA_ENV_CONFIG = {
    "loader": PackageLoader("parsec.backend.http.templates"),
    "undefined": StrictUndefined,
}
JINJA_ENV = Environment(**JINJA_ENV_CONFIG)  # type: ignore[arg-type]


def get_template(name: str | Template) -> Template:
    return JINJA_ENV.get_template(name)
