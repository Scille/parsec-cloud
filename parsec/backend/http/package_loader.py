# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from jinja2 import BaseLoader, TemplateNotFound
import importlib_resources
from importlib import import_module


class PackageLoader(BaseLoader):
    def __init__(self, path):
        self.path = path
        self.package = import_module(".templates", "parsec.backend.http")

    def get_source(self, environment, template):
        if not importlib_resources.is_resource(self.package, template):
            raise TemplateNotFound(template)
        source = importlib_resources.read_text(self.package, template)
        return source, self.path, lambda: True
