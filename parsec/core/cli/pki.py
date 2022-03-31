# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import attr
from typing import Sequence, Optional, Callable
from uuid import UUID
from pathlib import Path
import platform
from pendulum import DateTime
import click

from parsec.api.protocol import HumanHandle, DeviceLabel, PkiEnrollmentStatus
from parsec.cli_utils import cli_exception_handler, spinner, aprompt
from parsec.core.backend_connection import backend_authenticated_cmds_factory
from parsec.core.cli.invitation import ask_info_new_user
from parsec.core.config import CoreConfig
from parsec.core.types import BackendPkiEnrollmentAddr
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_device_options,
    core_config_options,
    save_device_options,
)
from parsec.core.pki2 import (
    pki_enrollment_remote_pendings_list,
    pki_enrollment_accept_remote_pending,
    ValidRemotePendingEnrollment,
    ClaqueAuSolRemotePendingEnrollment,
)
from parsec.core.pki import (
    is_pki_enrollment_available,
    PkiEnrollementSubmiterInitalCtx,
    PkiEnrollmentSubmiterSubmittedCtx,
)
from parsec.core.types import LocalDevice
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
    ctx = PkiEnrollementSubmiterInitalCtx.new(addr)
    try:
        requested_human_handle = HumanHandle(
            email=ctx.x509_certificate.issuer_email, label=ctx.x509_certificate.issuer_label
        )
    except ValueError:
        raise RuntimeError("bah !")  # TODO

    async with spinner("Sending PKI enrollment to the backend"):
        ctx = await ctx.submit(
            config_dir=config.config_dir,
            requested_device_label=requested_device_label,
            requested_human_handle=requested_human_handle,
            force=force,
        )

    enrollment_id_display = click.style(ctx.enrollment_id.hex, fg="green")
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
    pendings = PkiEnrollmentSubmiterSubmittedCtx.list_from_disk(config_dir=config.config_dir)

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

    def _display_pending_enrollment(pending: PkiEnrollmentSubmiterSubmittedCtx):
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

    for ctx in pendings:
        click.echo(_display_pending_enrollment(ctx))

        async with spinner("Fetching PKI enrollment status from the backend"):
            try:
                enrollment_status, occured_on, maybe_local_device = await ctx.poll(
                    clean_disk=True, extra_trust_roots=extra_trust_roots
                )

            except Exception:
                # TODO: exception handling !
                raise

        if enrollment_status == PkiEnrollmentStatus.SUBMITTED:
            # Nothing to do
            click.echo("Enrollment is still pending")

        elif enrollment_status == PkiEnrollmentStatus.CANCELLED:
            click.echo("Enrollment has been cancelled on " + click.style(occured_on, fg="yellow"))

        elif enrollment_status == PkiEnrollmentStatus.REJECTED:
            click.echo("Enrollment has been rejected on " + click.style(occured_on, fg="yellow"))

        else:
            assert enrollment_status == PkiEnrollmentStatus.ACCEPTED
            assert isinstance(maybe_local_device, LocalDevice)
            click.echo("Enrollment has been accepted on " + click.style(occured_on, fg="yellow"))


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
