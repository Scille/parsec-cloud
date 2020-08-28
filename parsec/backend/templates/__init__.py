# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from jinja2 import Environment, BaseLoader, TemplateNotFound, StrictUndefined
import importlib_resources


class PackageLoader(BaseLoader):
    def __init__(self, path):
        self.path = path

    def get_source(self, environment, template):
        from parsec.backend import templates  # Self import \o/

        try:
            source = importlib_resources.read_text(templates, template)
        except FileNotFoundError as exc:
            raise TemplateNotFound(template) from exc
        return source, self.path, lambda: True


JINJA_ENV = Environment(
    loader=PackageLoader("parsec.backend.http.templates"), undefined=StrictUndefined
)


def get_template(name):
    return JINJA_ENV.get_template(name)
