# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import uuid
import platform
from urllib.request import urlopen, Request

import trio
import click


from parsec.api.protocol import HumanHandle
from parsec.api.protocol.types import DeviceLabel, OrganizationID
from parsec.cli_utils import cli_exception_handler, aconfirm

from parsec.core.types.backend_address import BackendAddr

# from parsec.core.cli.invitation import ask_info_new_user
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_device_options,
    core_config_options,
    save_device_options,
)
from parsec.core.local_device import generate_new_device, save_device_with_smartcard_in_config

from parsec.core.types import BackendOrganizationAddr
from parsec.crypto import PrivateKey, SigningKey
from parsec.utils import trio_run
from parsec.core.local_device import LocalDeviceError

from parsec.api.protocol.pki import (
    pki_enrollment_request_serializer,
    pki_enrollment_get_reply_serializer,
)


def get_smartcard_module():
    try:
        from parsec_ext import smartcard
    except ImportError as exc:
        raise RuntimeError("The parsec-extensions module is not installed.") from exc
    return smartcard


async def _http_request(url, method="GET", data=None):
    def _do_req():
        req = Request(url=url, method=method, data=data)
        with urlopen(req) as rep:
            return rep.read()

    return await trio.to_thread.run_sync(_do_req)


async def _pki_enrollment_request(
    config, backend_address, organization_id, device_label, human_label, human_email, force
):
    # Get smartcard module
    smartcard = get_smartcard_module()

    # Get HTTP route for POST request and make sure it's available
    url = backend_address.to_http_domain_url(f"/anonymous/pki/{organization_id}/enrollment_request")
    await _http_request(url)

    # Generate information for the new device
    request_id = uuid.uuid4()
    private_key = PrivateKey.generate()
    signing_key = SigningKey.generate()

    # Get the X.509 certificate and sign the keys
    human_handle = HumanHandle(human_email, human_label)
    enrollment_request, local_request, certificate_id, certificate_sha1 = await trio.to_thread.run_sync(
        smartcard.prepare_enrollment_request,
        request_id,
        private_key,
        signing_key,
        human_handle,
        device_label,
        backend_address,
        organization_id,
    )

    # Serialize the arguments
    data = pki_enrollment_request_serializer.req_dumps(
        {
            "request": enrollment_request,
            "certificate_id": certificate_sha1,  # We use sha1 fingerprint as certficate ID
            "request_id": request_id,
            "force_flag": force,
        }
    )

    # Make sure the backend is available before saving to the disk
    await _http_request(url)

    # Save enrollment request to the disk
    await local_request.save(config.config_dir)

    # Perform the request
    rep_data = await _http_request(url, method="POST", data=data)

    # Deserialize the response
    rep = pki_enrollment_request_serializer.rep_loads(rep_data)

    # Echo the status
    click.echo(f"Status: {click.style(rep['status'],fg='yellow')}")


@click.command(short_help="Perform a PKI-based enrolement request")
@click.argument("backend_address", type=BackendAddr.from_url)
@click.argument("organization_id", required=True, type=OrganizationID)
@click.option("--device-label", prompt="Device label", default=platform.node, type=DeviceLabel)
@click.option("--human-label", prompt="Human label")
@click.option("--human-email", prompt="Human email")
@click.option("--force/--no-force")
@core_config_options
@cli_command_base_options
def pki_enrollment_request(
    config,
    backend_address,
    organization_id,
    device_label,
    human_label,
    human_email,
    force,
    **kwargs,
):
    """Perform a PKI-based enrolement request"""
    with cli_exception_handler(config.debug):
        trio_run(
            _pki_enrollment_request,
            config,
            backend_address,
            organization_id,
            device_label,
            human_label,
            human_email,
            force,
        )


async def _pki_enrollment_get_replies(config, request):
    smartcard = get_smartcard_module()

    async for path in smartcard.LocalEnrollmentRequest.iter_path(config.config_dir):
        if path.name.startswith(request):
            request = await smartcard.LocalEnrollmentRequest.load_from_path(path)

            # Build URL and check that it is available
            url = request.backend_address.to_http_domain_url(
                f"/anonymous/pki/{request.organization_id}/enrollment_get_reply"
            )
            await _http_request(url)

            # Build the "get_reply" request
            req_data = pki_enrollment_get_reply_serializer.dumps(
                {"certificate_id": request.certificate_sha1, "request_id": request.request_id}
            )

            # Perform the "get_reply" request
            rep_data = await _http_request(url, "POST", req_data)
            rep = pki_enrollment_get_reply.loads(rep_data)

            # Show status if reply is not OK
            if rep["status"] != "ok":
                click.echo(f"[{path.name} status] {click.style(rep['status'],fg='yellow')}")
                continue

            # Reply is OK, check the content
            reply = rep["reply"]
            try:
                subject, reply_info = smartcard.verify_enrollment_reply(reply)
            except LocalDeviceError as exc:
                click.echo(
                    f"[{path.name} status] {click.style('Invalid reply',fg='yellow')} ({exc})"
                )
                continue

            # Prompt the user for confirmation
            click.echo(f"[{path.name} status] {click.style('Valid reply',fg='yellow')}")
            click.echo(f"Issuer: {subject}")
            click.echo(f"Name: {reply_info.human_handle.label}")
            click.echo(f"Email: {reply_info.human_handle.email}")
            click.echo(f"Device name: {reply_info.device_label}")
            if not await aconfirm("Continue", prompt_suffix="? "):
                continue

            # Recover private keys
            private_key, signing_key = smartcard.recover_private_keys(request)

            # Create the local device
            organization_address = BackendOrganizationAddr.build(
                request.backend_address, request.organization_id, reply_info.root_verify_key
            )
            local_device = generate_new_device(
                organization_addr=organization_address,
                device_id=reply_info.device_id,
                profile=reply_info.profile,
                human_handle=reply_info.human_handle,
                device_label=reply_info.device_label,
                signing_key=signing_key,
                private_key=private_key,
            )

            # Save the device with the corresponding smartcard
            save_device_with_smartcard_in_config(
                config.config_dir,
                local_device,
                certificate_id=request.certificate_id,
                certificate_sha1=request.certificate_sha1,
            )

            # Remove the corresponding request file
            await request.remove_file()


@click.command(short_help="Get replies for PKI enrollment request")
@click.option("--request", "-R", help="Request to use designed by the begining its ID.", default="")
@core_config_options
@cli_command_base_options
def pki_enrollment_get_reply(config, request):
    """Get replies for PKI enrollment request"""
    with cli_exception_handler(config.debug):
        trio_run(_pki_enrollment_get_replies, config, request)


@click.command(short_help="show the list of enrollment requests")
@core_config_and_device_options
@cli_command_base_options
def pki_enrollment_get_requests(config, device, **kwargs):
    """
    Show the list of enrollment requests
    """
    with cli_exception_handler(config.debug):
        # Disable task monitoring given user prompt will block the coroutine
        # trio_run(_pki_enrollment_get_requests, config, device)
        pass


@click.command(short_help="accept an enrollment request")
@click.argument("certificate")
@click.option("--requested_label")
@save_device_options
@core_config_and_device_options
@cli_command_base_options
def pki_enrollment_reply(config, device, certificate, save_device_with_selected_auth, **kwargs):
    """
    Accept an enrollment request
    """
    with cli_exception_handler(config.debug):
        # Disable task monitoring given user prompt will block the coroutine
        # trio_run(_pki_enrollment_reply, config, device, save_device_with_selected_auth, certificate)
        pass
