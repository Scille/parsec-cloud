# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .common import ErrorVariant, Result


class OpenBaoListSelfEmailsError(ErrorVariant):
    class BadURL:
        pass

    class NoServerResponse:
        pass

    class BadServerResponse:
        pass

    class Internal:
        pass


async def openbao_list_self_emails(
    openbao_server_url: str,
    openbao_secret_mount_path: str,
    openbao_transit_mount_path: str,
    openbao_entity_id: str,
    openbao_auth_token: str,
) -> Result[list[str], OpenBaoListSelfEmailsError]:
    raise NotImplementedError
