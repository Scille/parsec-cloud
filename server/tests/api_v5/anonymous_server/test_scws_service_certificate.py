# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import anonymous_server_cmds
from tests.common import AnonymousServerRpcClient, Backend, HttpCommonErrorsTester


async def test_anonymous_server_scws_service_certificate_ok(
    backend: Backend, anonymous_server: AnonymousServerRpcClient
):
    fake_cert = "Not a real cert"
    backend.config.scws_service_certificate = fake_cert

    rep = await anonymous_server.scws_service_certificate()
    assert rep == anonymous_server_cmds.latest.scws_service_certificate.RepOk(certificate=fake_cert)


async def test_anonymous_server_scws_service_certificate_not_available(
    backend: Backend, anonymous_server: AnonymousServerRpcClient
):
    assert backend.config.scws_service_certificate is None

    rep = await anonymous_server.scws_service_certificate()
    assert rep == anonymous_server_cmds.latest.scws_service_certificate.RepNotAvailable()


async def test_anonymous_server_scws_service_certificate_http_common_errors(
    anonymous_server: AnonymousServerRpcClient,
    anonymous_server_http_common_errors_tester: HttpCommonErrorsTester,
):
    async def do():
        await anonymous_server.scws_service_certificate()

    await anonymous_server_http_common_errors_tester(do)
