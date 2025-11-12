# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import OpenBaoAuthType, anonymous_server_cmds
from parsec.config import AccountConfig, AllowedClientAgent, OpenBaoAuthConfig, OpenBaoConfig
from tests.common import AnonymousServerRpcClient, Backend, HttpCommonErrorsTester


@pytest.mark.parametrize("kind", ("default", "custom"))
async def test_anonymous_server_server_config_ok(
    backend: Backend,
    anonymous_server: AnonymousServerRpcClient,
    kind: str,
) -> None:
    match kind:
        case "default":
            expected_rep = anonymous_server_cmds.latest.server_config.RepOk(
                account=anonymous_server_cmds.latest.server_config.AccountConfig.DISABLED,
                organization_bootstrap=anonymous_server_cmds.latest.server_config.OrganizationBootstrapConfig.WITH_BOOTSTRAP_TOKEN,
                openbao=anonymous_server_cmds.latest.server_config.OpenBaoConfigDisabled(),
                client_agent=anonymous_server_cmds.latest.server_config.ClientAgentConfig.NATIVE_OR_WEB,
            )

        case "custom":
            backend.config.organization_spontaneous_bootstrap = True
            backend.config.account_config = AccountConfig.ENABLED_WITH_VAULT
            backend.config.allowed_client_agent = AllowedClientAgent.NATIVE_ONLY
            backend.config.openbao_config = OpenBaoConfig(
                server_url="https://openbao.parsec.invalid",
                secret_mount_path="secrets/parsec-keys",
                auths=[
                    OpenBaoAuthConfig(
                        id=OpenBaoAuthType.HEXAGONE,
                        mount_path="auth/hexagone",
                    ),
                    OpenBaoAuthConfig(
                        id=OpenBaoAuthType.PRO_CONNECT,
                        mount_path="auth/pro_connect",
                    ),
                ],
            )
            anonymous_server.headers["User-Agent"] = "Parsec-Client/3.7 Linux"

            expected_rep = anonymous_server_cmds.latest.server_config.RepOk(
                account=anonymous_server_cmds.latest.server_config.AccountConfig.ENABLED_WITH_VAULT,
                organization_bootstrap=anonymous_server_cmds.latest.server_config.OrganizationBootstrapConfig.SPONTANEOUS,
                openbao=anonymous_server_cmds.latest.server_config.OpenBaoConfigEnabled(
                    server_url="https://openbao.parsec.invalid",
                    secret=anonymous_server_cmds.latest.server_config.OpenBaoSecretConfigKV2(
                        "secrets/parsec-keys"
                    ),
                    auths=[
                        anonymous_server_cmds.latest.server_config.OpenBaoAuthConfig(
                            id="HEXAGONE", mount_path="auth/hexagone"
                        ),
                        anonymous_server_cmds.latest.server_config.OpenBaoAuthConfig(
                            id="PRO_CONNECT", mount_path="auth/pro_connect"
                        ),
                    ],
                ),
                client_agent=anonymous_server_cmds.latest.server_config.ClientAgentConfig.NATIVE_ONLY,
            )

        case unknown:
            assert False, unknown

    rep = await anonymous_server.server_config()
    assert rep == expected_rep


async def test_anonymous_server_server_config_http_common_errors(
    anonymous_server: AnonymousServerRpcClient,
    anonymous_server_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await anonymous_server.server_config()

    await anonymous_server_http_common_errors_tester(do)
