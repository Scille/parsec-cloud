# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import attr
from typing import Sequence, Optional, Callable
from uuid import UUID, uuid4
from pathlib import Path
import platform
from pendulum import DateTime
import click

from parsec.api.data import PkiEnrollmentSubmitPayload
from parsec.api.protocol import HumanHandle, DeviceLabel, PkiEnrollmentStatus
from parsec.cli_utils import cli_exception_handler, operation, spinner, aprompt
from parsec.core.backend_connection import (
    backend_authenticated_cmds_factory,
    pki_enrollment_submit as cmd_pki_enrollment_submit,
    pki_enrollment_info as cmd_pki_enrollment_info,
)
from parsec.core.cli.invitation import ask_info_new_user
from parsec.core.config import CoreConfig
from parsec.core.types import BackendPkiEnrollmentAddr, BackendOrganizationAddr
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_device_options,
    core_config_options,
    save_device_options,
)
from parsec.core.pki import (
    LocalPendingEnrollment,
    is_pki_enrollment_available,
    pki_enrollment_select_certificate,
    pki_enrollment_sign_payload,
    pki_enrollment_save_local_pending,
    pki_enrollment_remove_local_pending,
    pki_enrollment_list_local_pendings,
    pki_enrollment_load_accept_payload,
    pki_enrollment_remote_pendings_list,
    pki_enrollment_accept_remote_pending,
    ValidRemotePendingEnrollment,
    ClaqueAuSolRemotePendingEnrollment,
)
from parsec.core.types import LocalDevice
from parsec.core.local_device import LocalDeviceError, generate_new_device
from parsec.crypto import PrivateKey, SigningKey
from parsec.utils import trio_run


def _ensure_pki_enrollment_available():
    if not is_pki_enrollment_available():
        raise RuntimeError("Parsec smartcard extension not available")


async def _pki_enrollment_submit(
    config: CoreConfig,
    addr: BackendPkiEnrollmentAddr,
    requested_device_label: DeviceLabel,
    force: bool,
):
    # Get smartcard module
    enrollment_id = uuid4()
    signing_key = SigningKey.generate()
    private_key = PrivateKey.generate()

    x509_certificate = pki_enrollment_select_certificate()
    requested_human_handle = HumanHandle(
        email=x509_certificate.issuer_email, label=x509_certificate.issuer_label
    )

    # Build submit payload
    cooked_submit_payload = PkiEnrollmentSubmitPayload(
        verify_key=signing_key.verify_key,
        public_key=private_key.public_key,
        requested_human_handle=requested_human_handle,
        requested_device_label=requested_device_label,
    )
    raw_submit_payload = cooked_submit_payload.dump()
    payload_signature: bytes
    payload_signature = pki_enrollment_sign_payload(
        payload=raw_submit_payload, x509_certificate=x509_certificate
    )

    async with spinner("Sending PKI enrollment to the backend"):

        rep = await cmd_pki_enrollment_submit(
            addr=addr,
            enrollment_id=enrollment_id,
            force=force,
            submitter_der_x509_certificate=x509_certificate.der_x509_certificate,
            submit_payload_signature=payload_signature,
            submit_payload=raw_submit_payload,
        )

        if rep["status"] != "ok":
            raise RuntimeError(f"Backend refused to create enrollment: {rep}")

    # Save the enrollment request on disk.
    # Note there is not atomicity with the request to the backend, but it's
    # considered fine:
    # - if the pending enrollment is not saved, CLI will display an error message (unless
    #   the whole machine has crashed ^^) so user is expected to retry the submit command
    # - in case the enrollment is accepted by a ninja-fast admin before the submit can be
    #   retried, it's no big deal to revoke the newly enrolled user and restart from scratch
    pki_enrollment_save_local_pending(
        config_dir=config.config_dir,
        x509_certificate=x509_certificate,
        addr=addr,
        enrollment_id=enrollment_id,
        submitted_on=rep["submitted_on"],
        submit_payload=cooked_submit_payload,
        signing_key=signing_key,
        private_key=private_key,
    )

    enrollment_id_display = click.style(enrollment_id.hex, fg="green")
    click.echo(f"PKI enrollment {enrollment_id_display} submitted")


@click.command(short_help="submit a new PKI enrollment")
@click.argument("backend_invitation", type=BackendPkiEnrollmentAddr.from_url)
@click.option(
    "--device-label", prompt="Device label", default=lambda: platform.node(), type=DeviceLabel
)
@click.option(
    "--force/--no-force",
    help="replace any pending PKI enrollment done with the same X509 certificate",
)
@core_config_options
@cli_command_base_options
def pki_enrollment_submit(
    config: CoreConfig,
    backend_invitation: BackendPkiEnrollmentAddr,
    device_label: DeviceLabel,
    force: bool,
    **kwargs,
):
    """Submit a new PKI enrollment"""
    with cli_exception_handler(config.debug):
        _ensure_pki_enrollment_available()
        trio_run(_pki_enrollment_submit, config, backend_invitation, device_label, force)


async def _pki_enrollment_poll(
    config: CoreConfig,
    enrollment_id_filter: Optional[str],
    dry_run: bool,
    save_device_with_selected_auth: Callable,
    extra_trust_roots: Sequence[Path],
):
    pendings = pki_enrollment_list_local_pendings(config_dir=config.config_dir)

    # Try to shorten the UUIDs to make it easier to work with
    enrollment_ids = [e.enrollment_id for e in pendings]
    for enrollment_id_len in range(3, 64):
        if len({h.hex[:enrollment_id_len] for h in enrollment_ids}) == len(enrollment_ids):
            break

    # Filter if needed
    if enrollment_id_filter:
        if len(enrollment_id_filter) < enrollment_id_len:
            raise RuntimeError()
        pendings = [e for e in pendings if e.enrollment_id.hex.starswith(enrollment_id_filter)]
        if not pendings:
            raise RuntimeError(f"No enrollment with id {enrollment_id_filter} locally available")

    def _display_pending_enrollment(pending: LocalPendingEnrollment):
        enrollment_id_display = click.style(
            pending.enrollment_id.hex[:enrollment_id_len], fg="green"
        )
        display = f"Pending enrollment {enrollment_id_display}"
        display += f"\n  submitted on: " + click.style(pending.submitted_on, fg="yellow")
        display += f"\n  organization URL: " + click.style(pending.addr, fg="yellow")
        display += f"\n  X509 issuer SHA1 fingerprint: " + click.style(
            pending.x509_certificate.certificate_sha1.hex(), fg="yellow"
        )
        display += "\n  X509 issuer label: " + click.style(
            pending.x509_certificate.issuer_label, fg="yellow"
        )
        display += "\n  X509 issuer email: " + click.style(
            pending.x509_certificate.issuer_email, fg="yellow"
        )
        display += "\n  requested human handle: " + click.style(
            pending.submit_payload.requested_human_handle, fg="yellow"
        )
        display += "\n  requested device label: " + click.style(
            pending.submit_payload.requested_device_label, fg="yellow"
        )
        return display

    num_pendings_display = click.style(str(len(pendings)), fg="green")
    click.echo(f"Found {num_pendings_display} pending enrollment(s):")

    for pending in pendings:
        click.echo(_display_pending_enrollment(pending))

        async with spinner("Fetching PKI enrollment status from the backend"):
            try:
                rep = await cmd_pki_enrollment_info(
                    addr=pending.addr, enrollment_id=pending.enrollment_id
                )
            except Exception:
                # TODO: exception handling !
                raise
            if rep["status"] != "ok":
                # TODO: exception handling !
                raise RuntimeError()

        if rep["type"] == PkiEnrollmentStatus.SUBMITTED:
            # Nothing to do
            click.echo("Enrollment is still pending")

        elif rep["type"] == PkiEnrollmentStatus.CANCELLED:
            click.echo(
                "Enrollment has been cancelled on " + click.style(rep["cancelled_on"], fg="yellow")
            )
            if not dry_run:
                with operation("Removing enrollement info"):
                    pki_enrollment_remove_local_pending(
                        config_dir=config.config_dir, enrollment_id=pending.enrollment_id
                    )

        elif rep["type"] == PkiEnrollmentStatus.REJECTED:
            click.echo(
                "Enrollment has been rejected on " + click.style(rep["rejected_on"], fg="yellow")
            )
            if not dry_run:
                with operation("Removing enrollement info"):
                    pki_enrollment_remove_local_pending(
                        config_dir=config.config_dir, enrollment_id=pending.enrollment_id
                    )

        else:
            assert rep["type"] == PkiEnrollmentStatus.ACCEPTED
            click.echo(
                "Enrollment has been accepted on " + click.style(rep["accepted_on"], fg="yellow")
            )
            if not dry_run:
                with operation("Saving new device"):
                    try:
                        (accepter_x509_certif, accept_payload) = pki_enrollment_load_accept_payload(
                            config=config,
                            extra_trust_roots=extra_trust_roots,
                            der_x509_certificate=rep["accepter_der_x509_certificate"],
                            payload_signature=rep["accept_payload_signature"],
                            payload=rep["accept_payload"],
                        )

                        # Create the local device
                        organization_addr = BackendOrganizationAddr.build(
                            backend_addr=pending.addr,
                            organization_id=pending.addr.organization_id,
                            root_verify_key=accept_payload.root_verify_key,
                        )
                        new_device = generate_new_device(
                            organization_addr=organization_addr,
                            device_id=accept_payload.device_id,
                            profile=accept_payload.profile,
                            human_handle=accept_payload.human_handle,
                            device_label=accept_payload.device_label,
                            signing_key=pending.signing_key,
                            private_key=pending.private_key,
                        )

                    # Verification failed
                    except LocalDeviceError as exc:
                        click.echo(f"Invalid enrollement accep payload: {exc}")

                    await save_device_with_selected_auth(
                        config_dir=config.config_dir, device=new_device
                    )

                with operation("Removing enrollement info"):
                    pki_enrollment_remove_local_pending(
                        config_dir=config.config_dir, enrollment_id=pending.enrollment_id
                    )


@click.command(short_help="check status of the pending PKI enrollments locally available")
@click.argument("enrollment_id", required=False)
@click.option("--dry-run", is_flag=True)
@click.option("--extra-trust-root", multiple=True, default=(), type=Path)
@save_device_options
@core_config_options
@cli_command_base_options
def pki_enrollment_poll(
    config: CoreConfig,
    enrollment_id: Optional[str],
    dry_run: bool,
    save_device_with_selected_auth: Callable,
    extra_trust_root: Sequence[Path],
    **kwargs,
):
    """Check status of the pending PKI enrollments locally available"""
    with cli_exception_handler(config.debug):
        _ensure_pki_enrollment_available()
        trio_run(
            _pki_enrollment_poll,
            config,
            enrollment_id,
            dry_run,
            save_device_with_selected_auth,
            extra_trust_root,
        )


@attr.s
class CookedPendingEnrollment:
    enrollment_id: UUID
    short_enrollment_id: str
    display: str
    fingerprint: str
    submitted_on: DateTime
    submitter_der_x509_certificate: bytes
    submit_payload_signature: bytes
    raw_submit_payload: bytes
    action: Optional[str]  # None/accept/reject


async def _pki_enrollment_review_pendings(
    config: CoreConfig,
    device: LocalDevice,
    extra_trust_roots: Sequence[Path],
    list_only: bool,
    accept: Sequence[str],
    reject: Sequence[str],
):
    # Connect to the backend
    async with backend_authenticated_cmds_factory(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        keepalive=config.backend_connection_keepalive,
    ) as cmds:
        pendings = await pki_enrollment_remote_pendings_list(
            config=config, cmds=cmds, extra_trust_roots=extra_trust_roots
        )

        num_pendings_display = click.style(str(len(pendings)), fg="green")
        click.echo(f"Found {num_pendings_display} pending enrollment(s):")

        # Try to shorten the UUIDs to make it easier to work with
        enrollment_ids = [e.enrollment_id for e in pendings]
        for enrollment_id_len in range(3, 64):
            if len({h.hex[:enrollment_id_len] for h in enrollment_ids}) == len(enrollment_ids):
                break

        preselected_actions = {**{x: "accept" for x in accept}, **{x: "reject" for x in reject}}

        def _preselected_actions_lookup(enrollment_id: UUID) -> Optional[str]:
            for preselected in preselected_actions:
                if len(preselected) < enrollment_id_len:
                    continue
                if enrollment_id.hex.startswith(preselected):
                    return preselected_actions.pop(preselected)

        def _display_pending_enrollment(pending):
            enrollment_id_display = click.style(
                pending.enrollment_id.hex[:enrollment_id_len], fg="green"
            )
            display = f"Pending enrollment {enrollment_id_display}"
            display += f"\n  submitted on: " + click.style(pending.submitted_on, fg="yellow")
            display += f"\n  X509 issuer SHA1 fingerprint: " + click.style(
                pending.certificate_sha1.hex(), fg="yellow"
            )
            if isinstance(pending, ClaqueAuSolRemotePendingEnrollment):
                display += click.style("Invalid enrollment", fg="red") + f": {pending.error}"
            else:
                assert isinstance(pending, ValidRemotePendingEnrollment)
                display += "\n  X509 issuer label: " + click.style(
                    pending.submitter_x509_certif.issuer_label, fg="yellow"
                )
                display += "\n  X509 issuer email: " + click.style(
                    pending.submitter_x509_certif.issuer_email, fg="yellow"
                )
                display += "\n  requested human handle: " + click.style(
                    pending.submit_payload.requested_human_handle, fg="yellow"
                )
                display += "\n  requested device label: " + click.style(
                    pending.submit_payload.requested_device_label, fg="yellow"
                )
            return display

        for pending in pendings:
            enrollment_id = pending.enrollment_id
            click.echo(_display_pending_enrollment(pending))
            if list_only:
                continue

            if isinstance(pending, ClaqueAuSolRemotePendingEnrollment):
                action = _preselected_actions_lookup(enrollment_id)
                if action == "accept":
                    raise RuntimeError(f"Could not accept enrollment {enrollment_id.hex}")

                elif action is None:
                    # Let the user pick and action
                    action = await aprompt(
                        "-> Action",
                        default="Ignore",
                        type=click.Choice(["Ignore", "Reject"], case_sensitive=False),
                    ).lower()

                if action == "ignore":
                    continue

                else:
                    # Reject the request
                    assert action == "reject"
                    async with spinner("Rejecting PKI enrollment from the backend"):
                        rep = await cmds.pki_enrollment_reject(enrollment_id)
                        if rep["status"] != "ok":
                            raise RuntimeError(
                                f"Could not reject enrollment {enrollment_id.hex()}: {rep}"
                            )
                    continue

            else:
                assert isinstance(pending, ValidRemotePendingEnrollment)
                action = _preselected_actions_lookup(enrollment_id)
                if action is None:
                    # Let the user pick and action
                    action = await aprompt(
                        "-> Action",
                        default="Ignore",
                        type=click.Choice(["Ignore", "Reject", "Accept"], case_sensitive=False),
                    ).lower()

                if action == "ignore":
                    continue

                # Reject the request
                if action == "reject":
                    rep = await cmds.pki_enrollment_reject(enrollment_id)
                    if rep["status"] != "ok":
                        raise RuntimeError(
                            f"Could not reject enrollment {enrollment_id.hex}: {rep}"
                        )
                    click.echo(f"-> Successfully rejected")
                    continue

                else:
                    # Accept the request
                    assert action == "accept"

                    # Let the admin edit the user information
                    human_label, human_email, device_label, profile = await ask_info_new_user(
                        requested_device_label=pending.submit_payload.requested_device_label,
                        requested_human_handle=pending.submit_payload.requested_human_handle,
                    )
                    device_label = DeviceLabel(device_label)
                    human_handle = HumanHandle(human_email, human_label)

                    async with spinner("Accepting PKI enrollment in the backend"):
                        await pki_enrollment_accept_remote_pending(
                            cmds=cmds,
                            enrollment_id=enrollment_id,
                            author=device,
                            device_label=device_label,
                            human_handle=human_handle,
                            profile=profile,
                            public_key=pending.submit_payload.public_key,
                            verify_key=pending.submit_payload.verify_key,
                        )

        if preselected_actions:
            raise RuntimeError(
                f"Additional --accept/--reject elements not used: {', '.join(preselected_actions.keys())}"
            )


@click.command(short_help="show the pending PKI enrollments")
# TODO: document options
@click.option("--extra-trust-root", multiple=True, default=(), type=Path)
@click.option("--list-only", is_flag=True)
@click.option(
    "--accept", multiple=True, default=()
)  # Not typed as UUID given it can be only a subset
@click.option(
    "--reject", multiple=True, default=()
)  # Not typed as UUID given it can be only a subset
@core_config_and_device_options
@cli_command_base_options
def pki_enrollment_review_pendings(
    config: CoreConfig,
    device: LocalDevice,
    extra_trust_root: Sequence[Path],
    list_only: bool,
    accept: Sequence[str],
    reject: Sequence[str],
    **kwargs,
):
    """
    Show the pending PKI enrollments and accept/reject them
    """
    if list_only and (accept or reject):
        raise RuntimeError("Cannot use --list-only together with --accept/--reject")

    if set(accept) & set(reject):
        raise RuntimeError("An enrollment ID cannot be both marked as --accept and --reject")

    with cli_exception_handler(config.debug):
        _ensure_pki_enrollment_available()
        trio_run(
            _pki_enrollment_review_pendings,
            config,
            device,
            extra_trust_root,
            list_only,
            accept,
            reject,
        )
