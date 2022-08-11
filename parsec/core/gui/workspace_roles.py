# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec.core.types import WorkspaceRole
from parsec.core.gui.lang import translate as _


NOT_SHARED_KEY = "NOT_SHARED"


def get_role_translation(user_role):
    ROLES_TRANSLATIONS = {
        WorkspaceRole.READER: _("TEXT_WORKSPACE_ROLE_READER"),
        WorkspaceRole.CONTRIBUTOR: _("TEXT_WORKSPACE_ROLE_CONTRIBUTOR"),
        WorkspaceRole.MANAGER: _("TEXT_WORKSPACE_ROLE_MANAGER"),
        WorkspaceRole.OWNER: _("TEXT_WORKSPACE_ROLE_OWNER"),
        NOT_SHARED_KEY: _("TEXT_WORKSPACE_ROLE_NOT_SHARED"),
    }
    return ROLES_TRANSLATIONS[user_role]
