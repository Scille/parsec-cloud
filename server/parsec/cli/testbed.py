# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import asyncio
import hashlib
import uuid
from base64 import b64decode, b64encode
from collections import defaultdict
from collections.abc import AsyncIterator, Callable
from contextlib import asynccontextmanager
from copy import deepcopy
from typing import TYPE_CHECKING, Any
from urllib.parse import quote_plus

import anyio
import click
import structlog
from fastapi import APIRouter, BackgroundTasks, Request, Response
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse, RedirectResponse

from parsec._parsec import (
    AccountAuthMethodID,
    CryptoError,
    DateTime,
    EmailAddress,
    HumanHandle,
    OrganizationID,
    ParsecAddr,
    SecretKey,
    SigningKey,
    ValidationCode,
    authenticated_cmds,
)

if TYPE_CHECKING:
    from parsec.api import ApiFn
from parsec.client_context import AuthenticatedClientContext
from parsec.templates import get_environment

try:
    from parsec._parsec import testbed

    TESTBED_AVAILABLE = True

    type TestbedTemplateContent = testbed.TestbedTemplateContent  # pyright: ignore[reportRedeclaration]
    type TestbedTemplate = tuple[OrganizationID, int, testbed.TestbedTemplateContent]  # pyright: ignore[reportRedeclaration]
except ImportError:
    TESTBED_AVAILABLE = False
    type TestbedTemplate = tuple[OrganizationID, int, Any]
    type TestbedTemplateContent = Any

from parsec.asgi import AsgiApp, app_factory, serve_parsec_asgi_app
from parsec.backend import Backend, backend_factory
from parsec.cli.options import debug_config_options, logging_config_options
from parsec.cli.utils import cli_exception_handler
from parsec.components.memory.user import MemoryDatamodel, MemoryUserComponent
from parsec.config import (
    BackendConfig,
    LogLevel,
    MockedBlockStoreConfig,
    MockedDatabaseConfig,
    MockedEmailConfig,
    MockedSentEmail,
    OpenBaoAuthConfig,
    OpenBaoAuthType,
    OpenBaoConfig,
    PostgreSQLBlockStoreConfig,
    PostgreSQLDatabaseConfig,
)

logger: structlog.stdlib.BoundLogger = structlog.get_logger()


# Helper for other CLI commands to add dev-related options
def if_testbed_available[FC](decorator: Callable[[FC], FC]) -> Callable[[FC], FC]:
    if TESTBED_AVAILABLE:
        return decorator
    else:
        return lambda f: f


DEFAULT_PORT = 6770


class TestbedNotAvailable(Exception):
    pass


class UnknownTemplateError(Exception):
    pass


_ORG_ID_COUNT = 0


def next_organization_id(prefix: str) -> OrganizationID:
    """
    Convenient helper to generate unique organization IDs for the tests.

    Note the returned IDs are only unique across the lifetime of the process,
    but this is enough since the PostgreSQL database gets reset before each run.
    """
    global _ORG_ID_COUNT
    _ORG_ID_COUNT += 1
    return OrganizationID(f"{prefix}{_ORG_ID_COUNT}")


class TestbedBackend:
    __test__ = False  # Prevents Pytest from thinking this is a test class

    def __init__(
        self,
        backend: Backend,
        loaded_templates: dict[str, TestbedTemplate] | None = None,
    ):
        self.backend = backend
        self._account_count = 0
        self._load_template_lock = anyio.Lock()
        # keys: template ID, values: (to duplicate organization ID, CRC, template content)
        self._loaded_templates = {} if loaded_templates is None else loaded_templates
        self.template_per_org: dict[OrganizationID, TestbedTemplateContent] = {}
        # We expose some routes to simulate a OpenBao server, hence this field
        self.openbao_secrets: defaultdict[tuple[str, str], list[Any]] = defaultdict(list)
        self.openbao_signing_keys: dict[str, SigningKey] = {}
        # If the testbed server run with the memory backend, we can copy the datamodel
        # object to create snapshots and overwrite the initial one to rollback.
        self._data_snapshots: list[tuple[DateTime, MemoryDatamodel]] = []
        self._data: MemoryDatamodel | None
        if isinstance(backend.user, MemoryUserComponent):
            self._data = backend.user._data
        else:
            self._data = None  # Snapshot support is disabled

    async def get_template(self, template: str) -> TestbedTemplate:
        try:
            return self._loaded_templates[template]
        except KeyError:
            async with self._load_template_lock:
                # Ensure the template hasn't been loaded while we were waiting for the lock
                try:
                    return self._loaded_templates[template]

                except KeyError:
                    if not TESTBED_AVAILABLE:
                        raise TestbedNotAvailable()

                    # If it exists, template has not been loaded yet
                    maybe_template_content = testbed.test_get_testbed_template(  # pyright: ignore [reportPossiblyUnboundVariable]
                        template
                    )

                    if not maybe_template_content:
                        # No template with the given id
                        raise UnknownTemplateError(template)
                    template_content = maybe_template_content

                    template_crc = template_content.compute_crc()
                    template_org_id = await self.backend.test_load_template(template_content)
                    ret = (
                        template_org_id,
                        template_crc,
                        template_content,
                    )
                    self._loaded_templates[template] = ret
                    return ret

    async def new_organization(self, template: str) -> TestbedTemplate:
        template_org_id, template_crc, template_content = await self.get_template(template)
        new_org_id = next_organization_id(prefix="TestbedOrg")
        await self.backend.test_duplicate_organization(template_org_id, new_org_id)
        self.template_per_org[new_org_id] = template_content

        return (new_org_id, template_crc, template_content)

    async def customize_organization(self, id: OrganizationID, customization: bytes) -> None:
        template = self.template_per_org[id]
        cooked_customization = testbed.test_load_testbed_customization(template, customization)  # pyright: ignore [reportPossiblyUnboundVariable]
        await self.backend.test_customize_organization(id, template, cooked_customization)

    async def drop_organization(self, id: OrganizationID) -> None:
        await self.backend.test_drop_organization(id)
        del self.template_per_org[id]


testbed_router = APIRouter(tags=["testbed"])


def testbed_app_factory(testbed: TestbedBackend) -> AsgiApp:
    app = app_factory(testbed.backend)

    # Testbed server often runs in background, so it output on crash is often
    # not visible (e.g. on the CI). Hence it's convenient to have the client
    # print the stacktrace on our behalf.
    # Note the testbed server is only meant to be run for tests and on a local
    # local machine so this has no security implication.
    app.debug = True

    app.state.testbed = testbed

    app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],
        allow_methods=["*"],
        allow_headers=["*"],
    )

    app.include_router(testbed_router)

    return app


# We don't use json in the /testbed/... routes, this is to simplify
# as much as possible implementation on the client side


@testbed_router.post("/testbed/new/{template}")
async def test_new(template: str, request: Request, background_tasks: BackgroundTasks) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    try:
        (new_org_id, template_crc, _) = await testbed.new_organization(template)
    except UnknownTemplateError:
        return Response(
            status_code=404,
            content=b"unknown template",
        )

    match request.query_params.get("ttl"):
        case None as orga_life_limit:
            pass
        case str() as raw:
            try:
                orga_life_limit = float(raw)
            except ValueError:
                return Response(
                    status_code=400,
                    content=b"invalid ttl query param",
                )

    if orga_life_limit:

        async def _organization_garbage_collector():
            await asyncio.sleep(orga_life_limit)
            logger.info("Dropping testbed org due to time limit", organization=new_org_id.str)
            # Dropping is idempotent, so no need for error handling
            await testbed.backend.test_drop_organization(new_org_id)

        background_tasks.add_task(_organization_garbage_collector)

    return Response(
        status_code=200,
        content=f"{new_org_id.str}\n{template_crc}".encode(),
    )


@testbed_router.post("/testbed/customize/{raw_organization_id}")
async def test_customize(raw_organization_id: str, request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        return Response(status_code=400, content=b"")

    customization = await request.body()
    await testbed.customize_organization(organization_id, customization)

    return Response(status_code=200, content=b"")


@testbed_router.post("/testbed/drop/{raw_organization_id}")
async def test_drop(raw_organization_id: str, request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        return Response(status_code=400, content=b"")

    # Dropping is idempotent, so no need for error handling
    await testbed.drop_organization(organization_id)

    return Response(status_code=200, content=b"")


class CorruptedCertificateGet:
    """
    Patch `certificate_get` API command to return an invalid certificate.
    This is useful to test the client behavior when the server is buggy/malicious.
    """

    def __init__(
        self,
        testbed: TestbedBackend,
        vanilla_api_certificate_get: ApiFn,  # pyright: ignore[reportMissingTypeArgument] Req/Rep are currently untyped
    ):
        self.testbed = testbed
        self.vanilla_api_certificate_get = vanilla_api_certificate_get
        self.organizations: set[OrganizationID] = set()

    async def __call__(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.certificate_get.Req,
    ):
        rep = await self.vanilla_api_certificate_get(client_ctx, req)

        if not isinstance(rep, authenticated_cmds.latest.certificate_get.RepOk):
            return rep

        return authenticated_cmds.latest.certificate_get.RepOk(
            common_certificates=[*rep.common_certificates, b"<dummy>"],
            sequester_certificates=rep.sequester_certificates,
            shamir_recovery_certificates=rep.shamir_recovery_certificates,
            realm_certificates=rep.realm_certificates,
        )

    @classmethod
    def get_or_install(cls, testbed: TestbedBackend) -> CorruptedCertificateGet:
        target = testbed.backend.apis[authenticated_cmds.latest.certificate_get.Req]
        if isinstance(target, cls):
            return target

        vanilla_api_certificate_get = target
        corrupted = cls(testbed, vanilla_api_certificate_get)
        testbed.backend.apis[authenticated_cmds.latest.certificate_get.Req] = corrupted
        return corrupted

    @staticmethod
    def is_installed(testbed: TestbedBackend) -> bool:
        target = testbed.backend.apis[authenticated_cmds.latest.certificate_get.Req]
        return isinstance(target, CorruptedCertificateGet)

    @staticmethod
    def uninstall(testbed: TestbedBackend):
        target = testbed.backend.apis[authenticated_cmds.latest.certificate_get.Req]
        assert isinstance(target, CorruptedCertificateGet)
        testbed.backend.apis[authenticated_cmds.latest.certificate_get.Req] = (
            target.vanilla_api_certificate_get
        )


@testbed_router.post("/testbed/corrupt/{raw_organization_id}")
async def test_corruption(raw_organization_id: str, request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        return Response(status_code=400, content=b"")

    req_body = await request.body()
    for corruption in req_body.split(b"\n"):
        match corruption:
            case b"certificates:true":
                corruption = CorruptedCertificateGet.get_or_install(testbed)
                corruption.organizations.add(organization_id)

            case b"certificates:false":
                if CorruptedCertificateGet.is_installed(testbed):
                    corruption = CorruptedCertificateGet.get_or_install(testbed)
                    try:
                        corruption.organizations.remove(organization_id)
                    except KeyError:
                        pass

            case unknown:
                return Response(
                    status_code=400,
                    content=(
                        b"Unknown corruption `"
                        + unknown
                        + b"`, only `certificates:[true|false]` is allowed !"
                    ),
                )

    return Response(status_code=200, content=b"")


def _openbao_entity_id_from_vault_token_header(request: Request) -> str | None:
    vault_token = request.headers.get("x-vault-token", "")
    return _openbao_entity_id_from_vault_token_value(vault_token)


def _openbao_entity_id_from_vault_token_value(vault_token: str) -> str | None:
    if not vault_token.startswith("s."):
        return None
    return str(uuid.UUID(bytes=hashlib.sha256(vault_token.encode()).digest()[:16]))


@testbed_router.get("/testbed/mock/openbao/v1/auth/token/lookup-self")
async def test_openbao_auth_token_lookup_self(request: Request):
    entity_id = _openbao_entity_id_from_vault_token_header(request)
    if not entity_id:
        return Response(status_code=403)

    # See https://openbao.org/api-docs/auth/token/#sample-response-2

    return {
        "data": {
            "accessor": "8609694a-cdbc-db9b-d345-e782dbb562ed",
            "creation_time": 1523979354,
            "creation_ttl": 2764800,
            "display_name": "oidc-tesla",
            "entity_id": entity_id,
            "expire_time": "2018-05-19T11:35:54.466476215-04:00",
            "explicit_max_ttl": 0,
            "id": "cf64a70f-3a12-3f6c-791d-6cef6d390eed",
            "identity_policies": ["dev-group-policy"],
            "issue_time": "2018-04-17T11:35:54.466476078-04:00",
            "meta": {"username": "tesla"},
            "num_uses": 0,
            "orphan": True,
            "path": "auth/oidc/login/tesla",
            "policies": ["default", "testgroup2-policy"],
            "renewable": True,
            "ttl": 2764790,
        }
    }


@testbed_router.get("/testbed/mock/openbao/v1/secret/data/{secret_path:path}")
async def test_openbao_read_secret(secret_path: str, request: Request):
    testbed: TestbedBackend = request.app.state.testbed

    entity_id = _openbao_entity_id_from_vault_token_header(request)
    if not entity_id:
        return Response(status_code=403)

    secret_versions = testbed.openbao_secrets[(entity_id, secret_path)]
    if not secret_versions:
        return Response(status_code=404)

    # See https://openbao.org/api-docs/secret/kv/kv-v2/#sample-response-1

    return {
        "data": {
            "data": secret_versions[-1],
            "metadata": {
                "created_time": "2018-03-22T02:24:06.945319214Z",
                "custom_metadata": {"owner": "alice", "mission_critical": "false"},
                "deletion_time": "",
                "destroyed": False,
                "version": len(secret_versions),
            },
        }
    }


@testbed_router.post("/testbed/mock/openbao/v1/secret/data/{secret_path:path}")
async def test_openbao_create_secret(secret_path: str, request: Request):
    testbed: TestbedBackend = request.app.state.testbed

    entity_id = _openbao_entity_id_from_vault_token_header(request)
    if not entity_id:
        return Response(status_code=403)

    # See https://openbao.org/api-docs/secret/kv/kv-v2/#sample-payload-1

    try:
        req = await request.json()
    except ValueError:
        return Response("Bad JSON body", status_code=400)
    cas = req.get("option", {}).get("cas")
    if not (cas is None or isinstance(cas, int)):
        return Response(status_code=400)
    secret_data = req.get("data")

    secret_current_version = len(testbed.openbao_secrets.get((entity_id, secret_path), []))
    if cas is not None and secret_current_version != cas:
        return Response(status_code=409)

    testbed.openbao_secrets[(entity_id, secret_path)].append(secret_data)

    # See https://openbao.org/api-docs/secret/kv/kv-v2/#sample-payload-1

    return {
        "data": {
            "created_time": "2018-03-22T02:36:43.986212308Z",
            "custom_metadata": {"owner": "alice", "mission_critical": "false"},
            "deletion_time": "",
            "destroyed": False,
            "version": 1,
        }
    }


@testbed_router.post("/testbed/mock/openbao/v1/auth/{auth_provider}/oidc/auth_url")
async def test_openbao_hexagone_oidc_auth_url(request: Request, auth_provider: str):
    testbed: TestbedBackend = request.app.state.testbed

    # See https://openbao.org/api-docs/secret/kv/kv-v2/#sample-payload-1

    try:
        req = await request.json()
    except ValueError:
        return Response("Bad JSON body", status_code=400)
    role = req.get("role")
    if not isinstance(role, str):
        return Response("Bad/missing field `role`", status_code=400)

    redirect_uri = req.get("redirect_uri")
    if not isinstance(redirect_uri, str):
        return Response("Bad/missing field `redirect_uri`", status_code=400)

    sso_base_url = testbed.backend.config.server_addr.to_http_url("/testbed/mock/sso/authorize")
    auth_url = f"{sso_base_url}?redirect_uri={quote_plus(redirect_uri)}"

    return {
        "request_id": str(uuid.uuid4()),
        "lease_id": "",
        "renewable": False,
        "lease_duration": 0,
        "data": {
            "auth_url": auth_url,
        },
        "wrap_info": None,
        "warnings": None,
        "auth": None,
    }


@testbed_router.post("/testbed/mock/openbao/v1/transit/keys/{key_name}")
async def test_openbao_create_key(request: Request, key_name: str):
    testbed: TestbedBackend = request.app.state.testbed

    entity_id = _openbao_entity_id_from_vault_token_header(request)
    if not entity_id:
        return Response(status_code=403)

    if key_name != f"user-{entity_id}":
        return Response(status_code=403)

    # See https://openbao.org/api-docs/secret/transit/#create-key

    try:
        await request.json()
    except ValueError:
        return Response("Bad JSON body", status_code=400)

    try:
        key = testbed.openbao_signing_keys[entity_id]
    except KeyError:
        key = SigningKey.generate()
        testbed.openbao_signing_keys[entity_id] = key

    return {
        "request_id": str(uuid.uuid4()),
        "lease_id": "",
        "renewable": False,
        "lease_duration": 0,
        "data": {
            "allow_plaintext_backup": False,
            "auto_rotate_period": 0,
            "deletion_allowed": False,
            "derived": False,
            "exportable": False,
            "imported_key": False,
            "keys": {
                "1": {
                    "certificate_chain": "",
                    "creation_time": "2025-12-01T15:50:33.606080602Z",
                    "name": "ed25519",
                    "public_key": b64encode(key.verify_key.encode()).decode(),
                }
            },
            "latest_version": 1,
            "min_available_version": 0,
            "min_decryption_version": 1,
            "min_encryption_version": 0,
            "name": key_name,
            "soft_deleted": False,
            "supports_decryption": False,
            "supports_derivation": True,
            "supports_encryption": False,
            "supports_signing": True,
            "type": "ed25519",
        },
        "wrap_info": None,
        "warnings": None,
        "auth": None,
    }


@testbed_router.post("/testbed/mock/openbao/v1/transit/sign/{key_name}")
async def test_openbao_sign(request: Request, key_name: str):
    testbed: TestbedBackend = request.app.state.testbed

    entity_id = _openbao_entity_id_from_vault_token_header(request)
    if not entity_id:
        return Response(status_code=403)

    if key_name != f"user-{entity_id}":
        return Response(status_code=403)

    # See https://openbao.org/api-docs/secret/transit/#sign-data

    try:
        req = await request.json()
    except ValueError:
        return Response("Bad JSON body", status_code=400)

    input_b64 = req.get("input")
    if not isinstance(input_b64, str):
        return Response("Bad/missing field `input`", status_code=400)
    try:
        input_raw = b64decode(input_b64)
    except ValueError:
        return Response("Bad/missing field `input`", status_code=400)

    try:
        key = testbed.openbao_signing_keys[entity_id]
    except KeyError:
        return JSONResponse(status_code=400, content={"errors": ["signing key not found"]})

    signature_raw = key.sign_only_signature(input_raw)
    signature = f"vault:v1:{b64encode(signature_raw).decode()}"
    return {
        "request_id": str(uuid.uuid4()),
        "lease_id": "",
        "renewable": False,
        "lease_duration": 0,
        "data": {
            "key_version": 1,
            "signature": signature,
        },
        "wrap_info": None,
        "warnings": None,
        "auth": None,
    }


@testbed_router.post("/testbed/mock/openbao/v1/transit/verify/{key_name}")
async def test_openbao_verify(request: Request, key_name: str):
    testbed: TestbedBackend = request.app.state.testbed

    entity_id = _openbao_entity_id_from_vault_token_header(request)
    if not entity_id:
        return Response(status_code=403)

    author_entity_id = key_name.removeprefix("user-")

    # See https://openbao.org/api-docs/secret/transit/#verify-signed-data

    try:
        req = await request.json()
    except ValueError:
        return Response("Bad JSON body", status_code=400)

    input_b64 = req.get("input")
    if not isinstance(input_b64, str):
        return Response("Bad/missing field `input`", status_code=400)
    try:
        input_raw = b64decode(input_b64)
    except ValueError:
        return Response("Bad/missing field `input`", status_code=400)

    signature = req.get("signature")
    if not isinstance(signature, str) or not signature.startswith("vault:v1:"):
        return Response("Bad/missing field `signature`", status_code=400)
    try:
        signature_raw = b64decode(signature.removeprefix("vault:v1:"))
    except ValueError:
        return Response("Bad/missing field `signature`", status_code=400)

    try:
        key = testbed.openbao_signing_keys[author_entity_id]
    except KeyError:
        return JSONResponse(
            status_code=400, content={"errors": ["signature verification key not found"]}
        )

    try:
        key.verify_key.verify_with_signature(signature_raw, input_raw)
        is_valid = True
    except CryptoError:
        is_valid = False

    return {
        "request_id": str(uuid.uuid4()),
        "lease_id": "",
        "renewable": False,
        "lease_duration": 0,
        "data": {"valid": is_valid},
        "wrap_info": None,
        "warnings": None,
        "auth": None,
    }


@testbed_router.get("/testbed/mock/openbao/v1/identity/entity/id/{other_entity_id}")
async def test_openbao_get_entity_info(request: Request, other_entity_id: str):
    self_entity_id = _openbao_entity_id_from_vault_token_header(request)
    if not self_entity_id:
        return Response(status_code=403)

    # See https://openbao.org/api-docs/secret/transit/#verify-signed-data

    email = f"{other_entity_id}@example.invalid"

    return {
        "request_id": str(uuid.uuid4()),
        "lease_id": "",
        "renewable": False,
        "lease_duration": 0,
        "data": {
            "aliases": [
                {
                    "canonical_id": "217b7a03-b4d0-aff6-eaa8-4e1aa0573193",
                    "creation_time": "2025-12-01T15:09:53Z",
                    "custom_metadata": None,
                    "id": "d42286c0-a41d-bf0f-7dab-c9c27a0f0a58",
                    "last_update_time": "2025-12-01T15:09:53Z",
                    "local": False,
                    "merged_from_canonical_ids": None,
                    "metadata": {"role": "default"},
                    "mount_accessor": "auth_oidc_e99f8297",
                    "mount_path": "auth/oidc/",
                    "mount_type": "oidc",
                    "name": email,
                }
            ],
            "creation_time": "2025-12-01T15:09:53Z",
            "direct_group_ids": [],
            "disabled": False,
            "group_ids": [],
            "id": "217b7a03-b4d0-aff6-eaa8-4e1aa0573193",
            "inherited_group_ids": [],
            "last_update_time": "2025-12-01T15:09:53Z",
            "merged_entity_ids": None,
            "metadata": None,
            "name": "entity_46f449cb.root",
            "namespace_id": "root",
            "policies": [],
        },
        "wrap_info": None,
        "warnings": None,
        "auth": None,
    }


@testbed_router.get("/testbed/mock/sso/authorize")
async def test_openbao_hexagone_auth_url(request: Request):
    testbed: TestbedBackend = request.app.state.testbed

    match request.query_params.get("expect"):
        case "ok" | None:
            pass
        case "ko":
            return Response(status_code=403)
        case _:
            return Response(
                "Bad value for parameter `expect` (allowed: `ok` or `ko`)", status_code=400
            )

    # Since we don't actually do authentication, the discriminant field is used to be
    # able to isolate different authentication attempts.
    # The discriminant field is used to generate the `code` field that in turn
    # is used to compute the vault token... which itself it used to obtain the
    # entity ID!
    discriminant = request.query_params.get("discriminant", "alice@example.com")
    code = hashlib.sha256(discriminant.encode()).hexdigest()

    sso_base_url = testbed.backend.config.server_addr.to_http_url("/testbed/mock/sso/authorize")
    redirect_uri = request.query_params.get("redirect_uri")
    if redirect_uri is None:
        return Response("`redirect_uri` parameter required", status_code=400)
    redirect_uri = (
        # cspell: disable-next-line
        f"{redirect_uri}?code={code}&state=st_VXANtcosv3cCZThfaPLz&iss={quote_plus(sso_base_url)}"
    )

    return RedirectResponse(url=redirect_uri, status_code=302)


@testbed_router.get("/testbed/mock/openbao/v1/auth/{auth_provider}/oidc/callback")
async def test_openbao_hexagone_oidc_callback(request: Request, auth_provider: str):
    # See https://openbao.org/api-docs/auth/jwt/#sample-response-3

    state = request.query_params.get("state")
    if state is None:
        return Response("Missing parameter `state`", status_code=400)
    # cspell: disable-next-line
    if state != "st_VXANtcosv3cCZThfaPLz":
        return Response(
            # cspell: disable-next-line
            "Bad value for field `state` (always expects the dummy constant `st_VXANtcosv3cCZThfaPLz`)",
            status_code=400,
        )

    code = request.query_params.get("code")
    if code is None:
        return Response("Missing parameter `code`", status_code=400)

    vault_token = "s." + code
    entity_id = _openbao_entity_id_from_vault_token_value(vault_token)
    assert entity_id is not None

    return {
        "request_id": str(uuid.uuid4()),
        "lease_id": "",
        "renewable": False,
        "lease_duration": 0,
        "data": None,
        "wrap_info": None,
        "warnings": None,
        "auth": {
            "client_token": vault_token,
            "accessor": "9MOq1KMqjLpsEJ3baKfx1qh5",
            "policies": ["default"],
            "token_policies": ["default"],
            "metadata": {"role": "default"},
            "lease_duration": 60,
            "renewable": True,
            "entity_id": entity_id,
            "token_type": "service",
            "orphan": True,
            "mfa_requirement": None,
            "num_uses": 0,
        },
    }


@testbed_router.post("/testbed/snapshot/create")
async def test_snapshot(request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    if testbed._data is None:
        return Response(
            status_code=400, content="Snapshot&rollback is only supported with the memory backend"
        )

    now = DateTime.now()
    testbed._data_snapshots.append((now, deepcopy(testbed._data)))
    id = len(testbed._data_snapshots)
    return JSONResponse(status_code=200, content={"id": id, "timestamp": now.to_rfc3339()})


@testbed_router.post("/testbed/snapshot/rollback/{id}")
async def test_rollback(id: int, request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    if testbed._data is None:
        return Response(
            status_code=400, content="Snapshot&rollback is only supported with the memory backend"
        )

    if id <= 0 or id > len(testbed._data_snapshots):
        match len(testbed._data_snapshots):
            case 0:
                msg = "No snapshot created so far (use `POST /testbed/snapshot/create`)"
            case last_id:
                msg = f"Not found, last valid snapshot ID is {last_id}"
        return Response(status_code=404, content=msg)

    ts, save = deepcopy(testbed._data_snapshots[id - 1])

    # We must overwrite the content of the original data object (and not just
    # do `testbed._data = save`) since each component in the backend points to
    # this object.
    for field in testbed._data.__dataclass_fields__.keys():
        setattr(testbed._data, field, getattr(save, field))

    return JSONResponse(status_code=200, content={"timestamp": ts.to_rfc3339()})


@testbed_router.get("/testbed/mailbox/{raw_recipient}")
async def test_mailbox(raw_recipient: str, request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed
    try:
        recipient = EmailAddress(raw_recipient)
    except ValueError:
        return Response(status_code=400, content=b"invalid email address")

    match testbed.backend.config.email_config:
        case MockedEmailConfig() as email_config:
            sent_emails = (mail for mail in email_config.sent_emails if mail.recipient == recipient)
        case _:
            return Response(
                status_code=400, content=b"mailbox is only available with MockedEmailConfig !"
            )

    # Body should be something like `<sender_email>\t<timestamp>\t<base64(body)`,
    # also multiple mails can be send one after the other separated by `\n`

    def _encode_mail(mail: MockedSentEmail) -> bytes:
        return f"{mail.sender.str}\t{mail.timestamp.to_rfc3339()}\t".encode() + b64encode(
            mail.body.encode("utf8")
        )

    rep_body = b"\n".join(_encode_mail(mail) for mail in sent_emails)

    return Response(status_code=200, content=rep_body)


@testbed_router.post("/testbed/account/new")
async def test_new_account(request: Request) -> Response:
    testbed: TestbedBackend = request.app.state.testbed

    req_body = await request.body()
    try:
        (
            raw_auth_method_id,
            raw_mac_key,
            raw_vault_key_access,
        ) = req_body.split(b"\n")

        auth_method_id = AccountAuthMethodID.from_hex(raw_auth_method_id.decode("ascii"))
        auth_method_mac_key = SecretKey(b64decode(raw_mac_key))
        vault_key_access = b64decode(raw_vault_key_access)

    except ValueError:
        return Response(
            status_code=400,
            content=b"invalid body, expected `<auth_method_id as hex>\nb64(<mac_key>)\nb64(<vault_key_access>)`",
        )

    testbed._account_count += 1
    human_handle = HumanHandle(
        email=EmailAddress(f"agent{testbed._account_count}@example.com"),
        label=f"Agent{testbed._account_count:0>3}",
    )

    validation_code = await testbed.backend.account.create_send_validation_email(
        DateTime.now(), human_handle.email
    )
    assert isinstance(validation_code, ValidationCode)

    # Discard the mail that have been generated during the account creation
    match testbed.backend.config.email_config:
        case MockedEmailConfig() as email_config:
            email_config.sent_emails = [
                mail for mail in email_config.sent_emails if mail.recipient != human_handle.email
            ]
        case _:
            pass

    outcome = await testbed.backend.account.create_proceed(
        now=DateTime.now(),
        validation_code=validation_code,
        vault_key_access=vault_key_access,
        human_handle=human_handle,
        created_by_user_agent="TestbedAgent",
        created_by_ip="",
        auth_method_id=auth_method_id,
        auth_method_mac_key=auth_method_mac_key,
        auth_method_password_algorithm=None,
    )
    assert outcome is None

    rep_body = f"{human_handle.str}".encode()

    return Response(status_code=200, content=rep_body)


@asynccontextmanager
async def testbed_backend_factory(
    server_addr: ParsecAddr, with_postgresql: str | None
) -> AsyncIterator[TestbedBackend]:
    blockstore_config = (
        MockedBlockStoreConfig() if with_postgresql is None else PostgreSQLBlockStoreConfig()
    )
    # Same as the defaults in `db_server_options`
    if with_postgresql is None:
        db_config = MockedDatabaseConfig()
    else:
        db_config = PostgreSQLDatabaseConfig(
            url=with_postgresql, min_connections=1, max_connections=5
        )

    jinja_env = get_environment(None)
    config = BackendConfig(
        jinja_env=jinja_env,
        debug=True,
        db_config=db_config,
        sse_keepalive=30,
        proxy_trusted_addresses=None,
        server_addr=server_addr,
        email_config=MockedEmailConfig(EmailAddress("no-reply@parsec.com")),
        blockstore_config=blockstore_config,
        administration_token="s3cr3t",
        fake_account_password_algorithm_seed=SecretKey(b"F" * 32),
        organization_spontaneous_bootstrap=True,
        openbao_config=OpenBaoConfig(
            server_url=server_addr.to_http_url("/testbed/mock/openbao"),
            secret_mount_path="secret",
            transit_mount_path="transit",
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
        ),
        # Disable the rate limit
        email_rate_limit_cooldown_delay=0,
        email_rate_limit_max_per_hour=0,
    )
    async with backend_factory(config=config) as backend:
        yield TestbedBackend(backend=backend)


@click.command(
    context_settings={"max_content_width": 400},
    short_help="run the testbed server",
)
@click.option(
    "--host",
    "-H",
    default="127.0.0.1",
    show_default=True,
    envvar="PARSEC_HOST",
    help="Host to listen on",
)
@click.option(
    "--port",
    "-P",
    default=DEFAULT_PORT,
    type=int,
    show_default=True,
    envvar="PARSEC_PORT",
    help="Port to listen on",
)
@click.option(
    "--server-addr",
    envvar="PARSEC_SERVER_ADDR",
    metavar="URL",
    help="""URL to reach this server (typically used in invitation emails)
[default: parsec3://localhost:$PORT?no_ssl=True]
""",
)
@click.option(
    "--with-postgresql",
    envvar="PARSEC_WITH_POSTGRESQL",
    default=None,
    show_default=True,
    metavar="WITH_POSTGRESQL",
    help="Use a postgresql database instead of the mocked one",
)
@click.option(
    "--stop-after-process",
    type=int,
    default=None,
    show_default=True,
    envvar="PARSEC_STOP_AFTER_PROCESS",
    help="Stop the server once the given process has terminated",
)
# Add --log-level/--log-format/--log-file
@logging_config_options(default_log_level="INFO")
# Add --debug & --version
@debug_config_options
def testbed_cmd(
    host: str,
    port: int,
    server_addr: str | None,
    with_postgresql: str | None,
    stop_after_process: int | None,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
    debug: bool,
) -> None:
    if server_addr is None:
        server_addr = f"parsec3://localhost:{port}?no_ssl=True"
    cooked_server_addr = ParsecAddr.from_url(server_addr)

    async def _run_testbed():
        # Task group must be enclosed by backend (and not the other way around !)
        # given we will sleep forever in it __aexit__ part
        async with anyio.create_task_group() as tg:
            if stop_after_process:

                async def _watch_and_stop_after_process(pid: int, cancel_scope: anyio.CancelScope):
                    while True:
                        await anyio.sleep(1)
                        # psutil is a dev dependency, so we cannot import it globally
                        import psutil

                        if not psutil.pid_exists(pid):
                            print(f"PID `{pid}` has left, closing server.")
                            cancel_scope.cancel()
                            break

                tg.start_soon(_watch_and_stop_after_process, stop_after_process, tg.cancel_scope)

            async with testbed_backend_factory(
                server_addr=cooked_server_addr, with_postgresql=with_postgresql
            ) as testbed:
                click.secho("All set !", fg="yellow")
                click.echo("Don't forget to export `TESTBED_SERVER` environ variable:")
                click.secho(
                    f"export TESTBED_SERVER='parsec3://127.0.0.1:{port}?no_ssl=true'",
                    fg="magenta",
                )

                app = testbed_app_factory(testbed)
                await serve_parsec_asgi_app(
                    host=host,
                    port=port,
                    app=app,
                    proxy_trusted_addresses=None,
                    ssl_ciphers=["TLSv1"],
                )

                click.echo("bye ;-)")

    with cli_exception_handler(debug):
        asyncio.run(_run_testbed())
