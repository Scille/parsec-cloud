# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

import argparse
import sys
import tempfile
from functools import partial

import psutil
import trio
from quart import make_response

try:
    from parsec._parsec import test_get_testbed_templates
except ImportError as exc:
    raise RuntimeError("Test features are disabled !") from exc
from parsec.api.protocol import OrganizationID
from parsec.backend import backend_app_factory
from parsec.backend.asgi import app_factory as asgi_app_factory
from parsec.backend.asgi import serve_backend_with_asgi
from parsec.backend.config import BackendConfig, MockedBlockStoreConfig, MockedEmailConfig
from parsec.backend.user import Device, User
from parsec.logging import configure_logging

DEFAULT_ORGANIZATION_LIFE_LIMIT = 10 * 60  # 10mn


async def _run_server(
    host: str,
    port: int | None,
    backend_addr: str,
    orga_life_limit: float,
    stop_after_process: int | None,
):
    # TODO: avoid tempdir for email ?
    tmpdir = tempfile.mkdtemp(prefix="tmp-email-folder-")
    config = BackendConfig(
        debug=True,
        db_url="MOCKED",
        db_min_connections=1,
        db_max_connections=1,
        sse_keepalive=30,
        forward_proto_enforce_https=None,
        backend_addr=None,
        email_config=MockedEmailConfig("no-reply@parsec.com", tmpdir),
        blockstore_config=MockedBlockStoreConfig(),
        administration_token="s3cr3t",
        organization_spontaneous_bootstrap=True,
    )
    async with trio.open_nursery() as nursery:
        if stop_after_process:

            async def _watch_and_stop_after_process(pid: int, cancel_scope: trio.CancelScope):
                while True:
                    await trio.sleep(1)
                    if not psutil.pid_exists(pid):
                        print(f"PID `{pid}` has left, closing server.")
                        cancel_scope.cancel()
                        break

            nursery.start_soon(
                _watch_and_stop_after_process, stop_after_process, nursery.cancel_scope
            )

        async with backend_app_factory(config=config) as backend:
            org_count = 0

            # Populate the server with the testbed templates
            templates = test_get_testbed_templates()
            template_id_to_org_id_and_crc: dict[str, tuple[OrganizationID, int]] = {}
            for template in templates:
                org_id = OrganizationID(f"{template.id.capitalize()}OrgTemplate")
                await backend.organization.create(id=org_id, bootstrap_token="")

                devices = template.devices

                first_devices = set()
                for user in template.users:
                    user_id = user.user_id
                    first_device = next(
                        d
                        for d in devices
                        if d.device_id.user_id == user_id
                        and d.certif.timestamp == user.certif.timestamp
                        and d.certif.author == user.certif.author
                    )
                    await backend.user.create_user(
                        organization_id=org_id,
                        user=User(
                            user_id=user_id,
                            human_handle=user.human_handle,
                            user_certificate=user.raw_certif,
                            redacted_user_certificate=user.raw_redacted_certif,
                            user_certifier=user.certif.author,
                            profile=user.profile,
                            created_on=user.certif.timestamp,
                        ),
                        first_device=Device(
                            device_id=first_device.device_id,
                            device_label=first_device.device_label,
                            device_certificate=first_device.raw_certif,
                            redacted_device_certificate=first_device.raw_redacted_certif,
                            device_certifier=first_device.certif.author,
                            created_on=first_device.certif.timestamp,
                        ),
                    )
                    first_devices.add(first_device.device_id)

                for device in devices:
                    if device.device_id in first_devices:
                        continue
                    await backend.user.create_device(
                        organization_id=org_id,
                        device=Device(
                            device_id=device.device_id,
                            device_label=device.device_label,
                            device_certificate=device.raw_certif,
                            redacted_device_certificate=device.raw_redacted_certif,
                            device_certifier=device.certif.author,
                            created_on=device.certif.timestamp,
                        ),
                    )

                template_id_to_org_id_and_crc[template.id] = (org_id, template.crc)

            # All set ! Now we can start the server

            asgi = asgi_app_factory(backend)

            # Add CORS handling
            @asgi.after_request
            def _add_cors(response):
                response.headers["ACCESS-CONTROL-ALLOW-ORIGIN"] = "*"
                response.headers["ACCESS-CONTROL-ALLOW-METHODS"] = "*"
                response.headers["ACCESS-CONTROL-ALLOW-HEADERS"] = "*"
                return response

            # We don't use json in the /testbed/... routes, this is to simplify
            # as much as possible implementation on the client side

            @asgi.route("/testbed/new/<template>", methods=["POST"])
            async def test_new(template: str):  # type: ignore[misc]
                nonlocal org_count
                try:
                    template_org_id, template_crc = template_id_to_org_id_and_crc[template]
                except KeyError:
                    return await make_response(b"unknown template", 404)

                org_count += 1
                new_org_id = OrganizationID(f"Org{org_count}")
                backend.test_duplicate_organization(template_org_id, new_org_id)

                async def _organization_garbage_collector(organization_id: OrganizationID):
                    await trio.sleep(orga_life_limit)
                    print(f"drop {organization_id}")
                    # Dropping is idempotent, so no need for error handling
                    backend.test_drop_organization(organization_id)

                nursery.start_soon(_organization_garbage_collector, new_org_id)

                return await make_response(f"{new_org_id.str}\n{template_crc}".encode("utf8"), 200)

            @asgi.route("/testbed/drop/<organization_id>", methods=["POST"])
            async def test_drop(organization_id: str):  # type: ignore[misc]
                try:
                    organization_id = OrganizationID(organization_id)
                except ValueError:
                    return await make_response(b"", 400)
                print(f"drop {organization_id}")
                # Dropping is idempotent, so no need for error handling
                backend.test_drop_organization(organization_id)
                return await make_response(b"", 200)

            binds = await nursery.start(
                partial(
                    serve_backend_with_asgi,
                    backend=backend,
                    host=host,
                    port=port,
                    app=asgi,
                )
            )
            port = int(binds[0].rsplit(":", 1)[-1])

            PINK = "\x1b[35m"
            BOLD_YELLOW = "\x1b[1;33m"
            NO_COLOR = "\x1b[0;0m"
            print(
                f"{BOLD_YELLOW}All set !"
                f"{NO_COLOR} Don't forget to export `TESTBED_SERVER_URL` environ variable:\n"
                f"{PINK}export TESTBED_SERVER_URL='parsec://127.0.0.1:{port}?no_ssl=true'{NO_COLOR}"
            )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--host", "-H", default="127.0.0.1")
    parser.add_argument("--port", "-P", type=int, default=6770)
    parser.add_argument("--backend-addr")
    parser.add_argument("--orga-life-limit", type=float, default=DEFAULT_ORGANIZATION_LIFE_LIMIT)
    parser.add_argument("--stop-after-process", type=int, default=None)

    args = parser.parse_args()
    backend_addr = args.backend_addr or "127.0.0.1"
    configure_logging(log_level="INFO", log_format="CONSOLE", log_stream=sys.stderr)

    trio.run(
        _run_server,
        args.host,
        args.port,
        backend_addr,
        args.orga_life_limit,
        args.stop_after_process,
    )
