# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import tempfile
from base64 import urlsafe_b64decode
from collections.abc import Callable
from email.message import Message
from email.mime.text import MIMEText
from pathlib import Path
from typing import ParamSpec, TypeVar

import click

from parsec._parsec import EmailAddress, InvitationType, OrganizationID
from parsec.components.account import generate_email_deletion_email, generate_email_validation_email
from parsec.components.invite import generate_invite_email

PARSEC_EMAIL_ADDR = EmailAddress("parsec@example.com")
ALICE_EMAIL_ADDR = EmailAddress("alice@example.com")
DEFAULT_INVITATION_URL = "https://invitation.parsec.example.com"
DEFAULT_ACCOUNT_VALIDATION_URL = "https://validate.parsec.example.com"
DEFAULT_ACCOUNT_DELETION_URL = "https://del.parsec.example.com"
DEFAULT_BASE_SERVER_URL = "https://parsec.example.com"

P = ParamSpec("P")
T = TypeVar("T")


def write_mail_file_to_filesystem(message: Message, output_dir: Path, file_prefix: str):
    # Ensure output dir exist
    output_dir.mkdir(parents=True, exist_ok=True)

    for file in message.get_payload():
        assert isinstance(file, MIMEText)

        match file.get_content_subtype():
            case "plain":
                ext = "txt"
            case "html":
                ext = "html"
            case other:
                assert False, other

        data = file.get_payload()
        assert isinstance(data, str)
        match file.get_content_charset():
            case "us-ascii":
                pass
            case "utf-8":
                # UTF-8 content is encoded in base64
                data = urlsafe_b64decode(data).decode()
            case other:
                assert False, other

        output_file = output_dir / f"{file_prefix}.{ext}"
        print(f"Writing file to {output_file}")
        output_file.write_text(data)


def mail_templates_shared_options(fn: Callable[P, T]) -> Callable[P, T]:
    decorators = [
        click.option(
            "--output-dir",
            show_default=True,
            callback=lambda ctx, param, value: value or Path(tempfile.gettempdir()),
            type=click.Path(dir_okay=True, file_okay=False, exists=True, path_type=Path),
            help="The output directory to save the email file",
        ),
        click.option(
            "--server-url",
            default=DEFAULT_BASE_SERVER_URL,
            show_default=True,
            help="The base server url used to access static resources",
        ),
    ]
    for decorator in decorators:
        fn = decorator(fn)
    return fn


@click.group(short_help="Export server email to a file")
def export_email():
    pass


@export_email.command(short_help="Export invitation email")
@mail_templates_shared_options
@click.option(
    "--invitation-type",
    type=str,
    default=InvitationType.USER,
    show_default=True,
    callback=lambda ctx, param, value: InvitationType.from_str(value.upper()),
)
@click.option("--organization-id", type=OrganizationID, required=True)
@click.option("--invitation-url", default=DEFAULT_INVITATION_URL, show_default=True)
@click.option("--greeter-name", default=None, type=str, show_default=True, help="The greeter name")
def invite_mail(
    invitation_type: InvitationType,
    organization_id: OrganizationID,
    invitation_url: str,
    greeter_name: str | None,
    server_url: str,
    output_dir: Path,
):
    message = generate_invite_email(
        invitation_type=invitation_type,
        organization_id=organization_id,
        from_addr=PARSEC_EMAIL_ADDR,
        to_addr=ALICE_EMAIL_ADDR,
        invitation_url=invitation_url,
        server_url=server_url,
        greeter_name=greeter_name,
    )

    write_mail_file_to_filesystem(message, output_dir, "invitation_mail")


@export_email.command(short_help="Export account validation email")
@mail_templates_shared_options
@click.option(
    "--validation-url", type=str, default=DEFAULT_ACCOUNT_VALIDATION_URL, show_default=True
)
def account_validation(validation_url: str, server_url: str, output_dir: Path):
    message = generate_email_validation_email(
        from_addr=PARSEC_EMAIL_ADDR,
        to_addr=ALICE_EMAIL_ADDR,
        validation_url=validation_url,
        server_url=server_url,
    )

    write_mail_file_to_filesystem(message, output_dir, "account_validation_email")


@export_email.command(short_help="Export account deletion email")
@mail_templates_shared_options
@click.option("--deletion-url", type=str, default=DEFAULT_ACCOUNT_DELETION_URL, show_default=True)
def account_deletion(deletion_url: str, server_url: str, output_dir: Path):
    message = generate_email_deletion_email(
        from_addr=PARSEC_EMAIL_ADDR,
        to_addr=ALICE_EMAIL_ADDR,
        deletion_url=deletion_url,
        server_url=server_url,
    )

    write_mail_file_to_filesystem(message, output_dir, "account_deletion_email")
