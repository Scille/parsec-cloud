# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import attr
from typing import Sequence, Optional, Callable
from uuid import UUID
import platform
from parsec._parsec import DateTime
import click

from parsec.api.protocol import DeviceLabel
from parsec.cli_utils import aconfirm, cli_exception_handler, spinner, aprompt
from parsec.core.backend_connection import backend_authenticated_cmds_factory
from parsec.core.cli.invitation import ask_info_new_user
from parsec.core.config import CoreConfig
from parsec.core.local_device import save_device_with_smartcard_in_config
from parsec.core.types import BackendPkiEnrollmentAddr
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_device_options,
    core_config_options,
    save_device_options,
)
from parsec.core.pki import (
    is_pki_enrollment_available,
    PkiEnrollmentSubmitterInitialCtx,
    PkiEnrollmentSubmitterSubmittedCtx,
    PkiEnrollmentSubmitterSubmittedStatusCtx,
    PkiEnrollmentSubmitterCancelledStatusCtx,
    PkiEnrollmentSubmitterRejectedStatusCtx,
    PkiEnrollmentSubmitterAcceptedStatusButBadSignatureCtx,
    PkiEnrollmentSubmitterAcceptedStatusCtx,
    accepter_list_submitted_from_backend,
    PkiEnrollementAccepterValidSubmittedCtx,
    PkiEnrollementAccepterInvalidSubmittedCtx,
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
    ctx = await PkiEnrollmentSubmitterInitialCtx.new(addr)

    x509_display = f"Certificate SHA1 Fingerprint: " + click.style(
        ctx.x509_certificate.certificate_sha1.hex(), fg="yellow"
    )
    x509_display += "\nCertificate Issuer Common Name: " + click.style(
        ctx.x509_certificate.issuer_common_name, fg="yellow"
    )
    x509_display += "\nCertificate Subject Common Name: " + click.style(
        ctx.x509_certificate.subject_common_name, fg="yellow"
    )
    x509_display += "\nCertificate Subject Email Address: " + click.style(
        ctx.x509_certificate.subject_email_address, fg="yellow"
    )

    click.echo(x509_display)

    async with spinner("Sending PKI enrollment to the backend"):
        ctx = await ctx.submit(
            config_dir=config.config_dir, requested_device_label=requested_device_label, force=force
        )

    enrollment_id_display = click.style(ctx.enrollment_id.hex, fg="green")
    click.echo(f"PKI enrollment {enrollment_id_display} submitted")


@click.command(short_help="submit a new PKI enrollment")
@click.argument("enrollment-address", type=BackendPkiEnrollmentAddr.from_url)
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
    enrollment_address: BackendPkiEnrollmentAddr,
    device_label: DeviceLabel,
    force: bool,
    **kwargs,
):
    """Submit a new PKI enrollment"""
    with cli_exception_handler(config.debug):
        _ensure_pki_enrollment_available()
        trio_run(_pki_enrollment_submit, config, enrollment_address, device_label, force)


async def _pki_enrollment_poll(
    config: CoreConfig,
    enrollment_id_filter: Optional[str],
    dry_run: bool,
    save_device_with_selected_auth: Callable,
    finalize: Sequence[str],
):
    pendings = PkiEnrollmentSubmitterSubmittedCtx.list_from_disk(config_dir=config.config_dir)

    # Try to shorten the UUIDs to make it easier to work with
    enrollment_ids = [e.enrollment_id for e in pendings]
    for enrollment_id_len in range(3, 64):
        if len({h.hex[:enrollment_id_len] for h in enrollment_ids}) == len(enrollment_ids):
            break

    # Manage pre-selected actions
    preselected_actions = {x: "finalize" for x in finalize}

    def _preselected_actions_lookup(enrollment_id: UUID) -> Optional[str]:
        for preselected in preselected_actions:
            if len(preselected) < enrollment_id_len:
                continue
            if enrollment_id.hex.startswith(preselected):
                return preselected_actions.pop(preselected)

    # Filter if needed
    if enrollment_id_filter:
        if len(enrollment_id_filter) < enrollment_id_len:
            raise RuntimeError()
        pendings = [e for e in pendings if e.enrollment_id.hex.starswith(enrollment_id_filter)]
        if not pendings:
            raise RuntimeError(f"No enrollment with id {enrollment_id_filter} locally available")

    def _display_pending_enrollment(pending: PkiEnrollmentSubmitterSubmittedCtx):
        enrollment_id_display = click.style(
            pending.enrollment_id.hex[:enrollment_id_len], fg="green"
        )
        display = f"Pending enrollment {enrollment_id_display}"
        display += f"\n  Submitted on: " + click.style(pending.submitted_on, fg="yellow")
        display += f"\n  Organization URL: " + click.style(pending.addr.to_url(), fg="yellow")
        display += f"\n  Certificate SHA1 Fingerprint: " + click.style(
            pending.x509_certificate.certificate_sha1.hex(), fg="yellow"
        )
        display += "\n  Certificate Issuer Common Name: " + click.style(
            pending.x509_certificate.issuer_common_name, fg="yellow"
        )
        display += "\n  Certificate Subject Common Name: " + click.style(
            pending.x509_certificate.subject_common_name, fg="yellow"
        )
        display += "\n  Certificate Subject Email Address: " + click.style(
            pending.x509_certificate.subject_email_address, fg="yellow"
        )
        display += "\n  Requested Device Label: " + click.style(
            pending.submit_payload.requested_device_label.str, fg="yellow"
        )
        return display

    def _display_accepted_enrollment(accepted: PkiEnrollmentSubmitterAcceptedStatusCtx):
        display = (
            f"Enrollment has been accepted on " + click.style(ctx.accepted_on, fg="yellow") + " by:"
        )
        display += f"\n  Certificate SHA1 Fingerprint: " + click.style(
            accepted.accepter_x509_certificate.certificate_sha1.hex(), fg="yellow"
        )
        display += "\n  Certificate Issuer Common Name: " + click.style(
            accepted.accepter_x509_certificate.issuer_common_name, fg="yellow"
        )
        display += "\n  Certificate Subject Common Name: " + click.style(
            accepted.accepter_x509_certificate.subject_common_name, fg="yellow"
        )
        display += "\n  Certificate Subject Email Address: " + click.style(
            accepted.accepter_x509_certificate.subject_email_address, fg="yellow"
        )
        return display

    num_pendings_display = click.style(str(len(pendings)), fg="green")
    click.echo(f"Found {num_pendings_display} pending enrollment(s):")

    for ctx in pendings:
        click.echo(_display_pending_enrollment(ctx))

        async with spinner("Fetching PKI enrollment status from the backend"):
            try:
                ctx = await ctx.poll(extra_trust_roots=config.pki_extra_trust_roots)

            except Exception:
                # TODO: exception handling !
                raise

        if isinstance(ctx, PkiEnrollmentSubmitterSubmittedStatusCtx):
            # Nothing to do
            click.echo("Enrollment is still pending")

        elif isinstance(ctx, PkiEnrollmentSubmitterCancelledStatusCtx):
            click.echo(
                "Enrollment has been cancelled on " + click.style(ctx.submitted_on, fg="yellow")
            )
            if not dry_run:
                await ctx.remove_from_disk()

        elif isinstance(ctx, PkiEnrollmentSubmitterRejectedStatusCtx):
            click.echo(
                "Enrollment has been rejected on " + click.style(ctx.rejected_on, fg="yellow")
            )
            if not dry_run:
                await ctx.remove_from_disk()

        elif isinstance(ctx, PkiEnrollmentSubmitterAcceptedStatusButBadSignatureCtx):
            click.echo(
                "Enrollment has been accepted on " + click.style(ctx.accepted_on, fg="yellow")
            )
            raise RuntimeError(
                f"Cannot validate accept information with selected X509 certificate: {ctx.error}"
            )

        else:
            assert isinstance(ctx, PkiEnrollmentSubmitterAcceptedStatusCtx)
            click.echo(_display_accepted_enrollment(ctx))
            if not dry_run:
                preselected_finalize = _preselected_actions_lookup(ctx.enrollment_id) == "finalize"
                if not preselected_finalize and not await aconfirm("Finalize device creation"):
                    return
                ctx = await ctx.finalize()
                await save_device_with_smartcard_in_config(
                    config.config_dir,
                    ctx.new_device,
                    certificate_id=ctx.x509_certificate.certificate_id,
                    certificate_sha1=ctx.x509_certificate.certificate_sha1,
                )
                await ctx.remove_from_disk()


@click.command(short_help="check status of the pending PKI enrollments locally available")
@click.argument("enrollment_id", required=False)
@click.option("--dry-run", is_flag=True)
@click.option("--finalize", multiple=True, default=())
@save_device_options
@core_config_options
@cli_command_base_options
def pki_enrollment_poll(
    config: CoreConfig,
    enrollment_id: Optional[str],
    dry_run: bool,
    save_device_with_selected_auth: Callable,
    finalize: Sequence[str],
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
            finalize,
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
        pendings = await accepter_list_submitted_from_backend(
            cmds=cmds, extra_trust_roots=config.pki_extra_trust_roots
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
            display += f"\n  Submitted on: " + click.style(pending.submitted_on, fg="yellow")
            display += f"\n  Certificate SHA1 fingerprint: " + click.style(
                pending.submitter_x509_certificate_sha1.hex(), fg="yellow"
            )
            if isinstance(pending, PkiEnrollementAccepterInvalidSubmittedCtx):
                display += click.style("\nInvalid enrollment", fg="red") + f": {pending.error}"
            else:
                assert isinstance(pending, PkiEnrollementAccepterValidSubmittedCtx)
                display += "\n  Certificate Issuer Common Name: " + click.style(
                    pending.submitter_x509_certificate.issuer_common_name, fg="yellow"
                )
                display += "\n  Certificate Subject Common Name: " + click.style(
                    pending.submitter_x509_certificate.subject_common_name, fg="yellow"
                )
                display += "\n  Certificate Subject Email Address: " + click.style(
                    pending.submitter_x509_certificate.subject_email_address, fg="yellow"
                )
                display += "\n  Requested Device Label: " + click.style(
                    pending.submit_payload.requested_device_label.str, fg="yellow"
                )
            return display

        for pending in pendings:
            enrollment_id = pending.enrollment_id
            click.echo(_display_pending_enrollment(pending))
            if list_only:
                continue

            if isinstance(pending, PkiEnrollementAccepterInvalidSubmittedCtx):
                action = _preselected_actions_lookup(enrollment_id)
                if action == "accept":
                    raise RuntimeError(f"Could not accept invalid enrollment {enrollment_id.hex}")

                elif action is None:
                    # Let the user pick and action
                    action = await aprompt(
                        "-> Action",
                        default="Ignore",
                        type=click.Choice(["Ignore", "Reject"], case_sensitive=False),
                    )
                    action = action.lower()

                if action == "ignore":
                    continue

                else:
                    # Reject the request
                    assert action == "reject"
                    async with spinner("Rejecting PKI enrollment from the backend"):
                        await pending.reject()
                    continue
            else:
                assert isinstance(pending, PkiEnrollementAccepterValidSubmittedCtx)

                action = _preselected_actions_lookup(enrollment_id)
                if action is None:
                    # Let the user pick and action
                    action = await aprompt(
                        "-> Action",
                        default="Ignore",
                        type=click.Choice(["Ignore", "Reject", "Accept"], case_sensitive=False),
                    )
                    action = action.lower()

                if action == "ignore":
                    continue

                # Reject the request
                if action == "reject":
                    async with spinner("Rejecting PKI enrollment from the backend"):
                        await pending.reject()
                    continue

                else:
                    # Accept the request
                    assert action == "accept"

                    # Let the admin edit the user information
                    (
                        granted_device_label,
                        granted_human_handle,
                        granted_profile,
                    ) = await ask_info_new_user(
                        default_device_label=pending.submit_payload.requested_device_label,
                        default_user_label=pending.submitter_x509_certificate.subject_common_name,
                        default_user_email=pending.submitter_x509_certificate.subject_email_address,
                    )

                    async with spinner("Accepting PKI enrollment in the backend"):
                        await pending.accept(
                            author=device,
                            device_label=granted_device_label,
                            human_handle=granted_human_handle,
                            profile=granted_profile,
                        )

        if preselected_actions:
            raise RuntimeError(
                f"Additional --accept/--reject elements not used: {', '.join(preselected_actions.keys())}"
            )


@click.command(short_help="show the pending PKI enrollments")
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
        trio_run(_pki_enrollment_review_pendings, config, device, list_only, accept, reject)
