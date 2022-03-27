# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import uuid
import pathlib
import platform
from urllib.request import urlopen, Request

import trio
import click


from parsec.api.protocol import HumanHandle
from parsec.api.protocol.types import DeviceLabel
from parsec.cli_utils import cli_exception_handler, aconfirm, aprompt
from parsec.core.backend_connection.authenticated import backend_authenticated_cmds_factory
from parsec.core.cli.invitation import ask_info_new_user
from parsec.core.invite.greeter import _create_new_user_certificates

from parsec.core.types.backend_address import BackendInvitationAddr, InvitationType

from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_device_options,
    core_config_options,
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


async def _pki_enrollment_request(config, invitation_address, device_label, force):
    # Get smartcard module
    smartcard = get_smartcard_module()

    # Check invitation address
    if invitation_address.invitation_type != InvitationType.PKI:
        raise ValueError("The invitation address must correspond to a PKI invitation")

    # Get HTTP route for POST request and make sure it's available
    organization_id = invitation_address.organization_id
    url = invitation_address.to_http_domain_url(
        f"/anonymous/pki/{organization_id}/enrollment_request"
    )
    await _http_request(url)

    # Generate information for the new device
    request_id = uuid.uuid4()
    private_key = PrivateKey.generate()
    signing_key = SigningKey.generate()

    # Get the X.509 certificate and sign the keys
    enrollment_request, local_request, certificate_id, certificate_sha1 = await trio.to_thread.run_sync(
        smartcard.prepare_enrollment_request,
        request_id,
        private_key,
        signing_key,
        device_label,
        invitation_address,
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
@click.argument("backend-invitation", type=BackendInvitationAddr.from_url)
@click.option(
    "--device-label", prompt="Device label", default=lambda: platform.node(), type=DeviceLabel
)
@click.option("--force/--no-force")
@core_config_options
@cli_command_base_options
def pki_enrollment_request(config, backend_invitation, device_label, force, **kwargs):
    """Perform a PKI-based enrolement request"""
    with cli_exception_handler(config.debug):
        trio_run(_pki_enrollment_request, config, backend_invitation, device_label, force)


async def _pki_enrollment_get_replies(config, request_prefix, extra_trust_roots):
    smartcard = get_smartcard_module()

    async for path in smartcard.LocalEnrollmentRequest.iter_path(config.config_dir):
        if path.name.startswith(request_prefix):
            request = await smartcard.LocalEnrollmentRequest.load_from_path(path)

            # Build URL and check that it is available
            url = request.backend_addr.to_http_domain_url(
                f"/anonymous/pki/{request.organization_id}/enrollment_get_reply"
            )
            await _http_request(url)

            # Build the "get_reply" request
            req_data = pki_enrollment_get_reply_serializer.req_dumps(
                {"certificate_id": request.certificate_sha1, "request_id": request.request_id}
            )

            # Perform the "get_reply" request
            rep_data = await _http_request(url, "POST", req_data)
            rep = pki_enrollment_get_reply_serializer.rep_loads(rep_data)

            # Show status if reply is not OK
            if rep["status"] != "ok":
                click.echo(f"[{path.name} status] {click.style(rep['status'],fg='yellow')}")
                if rep["status"] != "pending":
                    if await aconfirm("Remove the local request"):
                        await request.get_path(config.config_dir).unlink()
                continue

            # Reply is OK, check the content
            reply = rep["reply"]
            try:
                subject, reply_info = smartcard.verify_enrollment_reply(
                    config, reply, extra_trust_roots
                )
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
            click.echo(f"Profile: {reply_info.profile.name}")
            if not await aconfirm("Continue", prompt_suffix="? "):
                continue

            # Recover private keys
            private_key, signing_key = await trio.to_thread.run_sync(
                smartcard.recover_private_keys, request
            )

            # Create the local device
            organization_address = BackendOrganizationAddr.build(
                request.backend_addr, request.organization_id, reply_info.root_verify_key
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
            await request.get_path(config.config_dir).unlink()


@click.command(short_help="Get replies for PKI enrollment request")
@click.option("--request", "-R", help="Request to use designed by the begining its ID.", default="")
@click.option("--extra-trust-root", multiple=True, default=(), type=pathlib.Path)
@core_config_options
@cli_command_base_options
def pki_enrollment_get_reply(config, request, extra_trust_root, **kwargs):
    """Get replies for PKI enrollment request"""
    with cli_exception_handler(config.debug):
        trio_run(_pki_enrollment_get_replies, config, request, extra_trust_root)


async def _pki_enrollment_get_requests(config, device, extra_trust_roots, list_only):
    smartcard = get_smartcard_module()

    # Connect to the backend
    async with backend_authenticated_cmds_factory(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        keepalive=config.backend_connection_keepalive,
    ) as cmds:

        # Get pending requests
        rep = await cmds.pki_enrollment_get_requests()
        if rep["status"] != "ok":
            raise RuntimeError(f"Could not get the enrollment requests: {rep}")

        # Loop over pending requests
        for certificate_id, request_id, request in rep["requests"]:

            # Verify the enrollment request
            try:
                subject, request_info = smartcard.verify_enrollment_request(
                    config, request, extra_trust_roots
                )

            # Verification failed
            except smartcard.LocalDeviceError as exc:
                click.echo(
                    f"[SHA-1 fingerprint: {certificate_id.hex()}] [Request ID: {request_id}] [User Information: {request.requested_human_handle}] Invalid request"
                )
                click.echo(f"-> [Error] {exc}")

                # No action to take
                if list_only:
                    continue

                # Let the user pick and action
                choice = await aprompt(
                    "-> Action",
                    default="Ignore",
                    type=click.Choice(["Ignore", "Reject"], case_sensitive=False),
                )
                if choice.lower() == "ignore":
                    continue

                # Reject the request
                assert choice.lower() == "reject"
                rep = await cmds.pki_enrollment_reply(
                    certificate_id, request_id, reply=None, user_id=None
                )
                if rep["status"] != "ok":
                    raise RuntimeError(f"Could not reject to the enrollment request: {rep}")
                click.echo(f"-> Successfully rejected")
                continue

            # The request is successfully verified
            click.echo(
                f"[SHA-1 fingerprint: {certificate_id.hex()}] [Request ID: {request_id}] [User Information: {request.requested_human_handle}] Valid request"
            )
            click.echo(f"-> [Subject] {subject}")

            # No action to take
            if list_only:
                continue

            # Let the user pick an action
            choice = await aprompt(
                "-> Action",
                default="Ignore",
                type=click.Choice(["Ignore", "Reject", "Accept"], case_sensitive=False),
            )
            if choice.lower() == "ignore":
                continue

            # Reject the request
            if choice.lower() == "reject":
                rep = await cmds.pki_enrollment_reply(
                    certificate_id, request_id, reply=None, user_id=None
                )
                if rep["status"] != "ok":
                    raise RuntimeError(f"Could not reject the enrollment request: {rep}")
                click.echo(f"-> Successfully rejected")
                continue

            # Accept the request
            assert choice.lower() == "accept"

            # Let the admin edit the user information
            human_label, human_email, device_label, profile = await ask_info_new_user(
                request_info.requested_device_label, request_info.requested_human_handle
            )
            device_label = DeviceLabel(device_label)
            human_handle = HumanHandle(human_email, human_label)

            # Create the certificate for the new user
            author = device
            user_certificate, redacted_user_certificate, device_certificate, redacted_device_certificate, user_confirmation = _create_new_user_certificates(
                author,
                device_label,
                human_handle,
                profile,
                request_info.public_key,
                request_info.verify_key,
            )

            # Prepare reply
            reply = await trio.to_thread.run_sync(
                smartcard.prepare_enrollment_reply, author, user_confirmation
            )

            # Perform the pki_enrollment_reply command
            rep = await cmds.pki_enrollment_reply(
                certificate_id,
                request_id,
                reply=reply,
                user_id=user_confirmation.device_id.user_id,
                user_certificate=user_certificate,
                device_certificate=device_certificate,
                redacted_user_certificate=redacted_user_certificate,
                redacted_device_certificate=redacted_device_certificate,
            )
            if rep["status"] != "ok":
                raise RuntimeError(f"Could not reject the enrollment request: {rep}")
            click.echo(f"-> Successfully accepted")


@click.command(short_help="show the list of enrollment requests")
@click.option("--extra-trust-root", multiple=True, default=(), type=pathlib.Path)
@click.option("--list-only/--reply", default=True)
@core_config_and_device_options
@cli_command_base_options
def pki_enrollment_get_requests(config, device, extra_trust_root, list_only, **kwargs):
    """
    Show the list of enrollment requests, and pick a reply if `--reply` is provided
    """
    with cli_exception_handler(config.debug):
        trio_run(_pki_enrollment_get_requests, config, device, extra_trust_root, list_only)
