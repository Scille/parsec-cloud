from datetime import datetime, timezone


class UserManifest:
    def __init__(self):
        pass


class FileManifest:
    def __init__(self):
        pass


class FileBlock:
    def __init__(self):
        pass


class Workspace:
    def __init__(self):
        pass


class File:
    def __init__(self, created=None, updated=None, data=None):
        self.data = data or b''
        now = datetime.now(timezone.utc)
        self.created = created or now
        self.updated = updated or now


class Directory:
    def __init__(self, created=None, updated=None, children=None):
        self.children = children or {}
        now = datetime.now(timezone.utc)
        self.created = created or now
        self.updated = updated or now


class BaseWorkspaceReader:
    pass


class BaseWorkspaceWriter:
    pass


class BaseWorkspaceSerializer:
    pass


def workspace_factory(user_manifest):
    pass

