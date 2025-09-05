# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from enum import auto

from parsec._parsec import (
    OrganizationID,
    VlobID,
    authenticated_cmds,
)
from parsec.api import api
from parsec.client_context import AuthenticatedClientContext
from parsec.config import BackendConfig
from parsec.types import BadOutcomeEnum


class EditicsCreateOrJoinBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    WORKSPACE_NOT_FOUND = auto()
    FILE_NOT_FOUND = auto()


class BaseEditicsComponent:
    def __init__(self, config: BackendConfig):
        self._config = config

    async def editics_create_or_join_session(
        self,
        encrypted_session_key: bytes,
        file_id: VlobID,
        workspace_id: VlobID,
        organization_id: OrganizationID,
    ) -> bytes | EditicsCreateOrJoinBadOutcome:
        raise NotImplementedError

    @api
    async def api_editics_join_session(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.editics_join_session.Req,
    ) -> authenticated_cmds.latest.editics_join_session.Rep:
        output = await self.editics_create_or_join_session(
            req.encrypted_session_key, req.file_id, req.workspace_id, client_ctx.organization_id
        )

        match output:
            case bytes() as session_key:
                return authenticated_cmds.latest.editics_join_session.RepOk(session_key)
            case EditicsCreateOrJoinBadOutcome():
                raise NotImplementedError
