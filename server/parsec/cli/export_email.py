# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from base64 import urlsafe_b64decode
from collections.abc import Callable
from email.message import Message
from email.mime.text import MIMEText
from pathlib import Path

import click

from parsec._parsec import EmailAddress, InvitationType, OrganizationID, ValidationCode
from parsec.components.account import (
    _generate_account_create_validation_email,
    _generate_account_delete_validation_email,
)
from parsec.components.invite import generate_invite_email
from parsec.templates import get_environment

DEFAULT_SENDER_EMAIL = EmailAddress("parsec@example.com")
DEFAULT_RECIPIENT_EMAIL = EmailAddress("alice@example.com")
DEFAULT_ORGANIZATION_ID = OrganizationID("CoolOrg")
DEFAULT_INVITATION_URL = "https://invitation.parsec.example.com"
DEFAULT_BASE_SERVER_URL = "https://parsec.example.com"
# cspell: words DELC8D
DEFAULT_VALIDATION_CODE = ValidationCode("DELC8D")


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


def mail_templates_shared_options[**P, T](fn: Callable[P, T]) -> Callable[P, T]:
    decorators = [
        click.option(
            "--sender",
            show_default=True,
            type=EmailAddress,
            default=DEFAULT_SENDER_EMAIL.str,
        ),
        click.option(
            "--recipient",
            show_default=True,
            type=EmailAddress,
            default=DEFAULT_RECIPIENT_EMAIL.str,
        ),
        click.option(
            "--output-dir",
            show_default=True,
            default=".",
            type=click.Path(dir_okay=True, file_okay=False, path_type=Path),
            help="The output directory to save the email file",
        ),
        click.option(
            "--server-url",
            default=DEFAULT_BASE_SERVER_URL,
            show_default=True,
            help="The base server url used to access static resources",
        ),
        click.option(
            "--template-dir",
            type=click.Path(dir_okay=True, file_okay=False, exists=True, path_type=Path),
            help="Load templates from the specified directory instead of using the default one",
        ),
    ]
    for decorator in decorators:
        fn = decorator(fn)
    return fn


@click.group(short_help="Export server email to a file for debugging purpose")
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
@click.option(
    "--organization-id", type=OrganizationID, default=DEFAULT_ORGANIZATION_ID, show_default=True
)
@click.option("--invitation-url", default=DEFAULT_INVITATION_URL, show_default=True)
@click.option(
    "--reply-to",
    show_default=True,
    type=EmailAddress,
    default=None,
)
@click.option("--greeter-name", default=None, type=str, show_default=True, help="The greeter name")
def invite(
    sender: EmailAddress,
    recipient: EmailAddress,
    invitation_type: InvitationType,
    organization_id: OrganizationID,
    invitation_url: str,
    greeter_name: str | None,
    reply_to: EmailAddress | None,
    server_url: str,
    output_dir: Path,
    template_dir: Path | None,
):
    jinja_env = get_environment(template_dir)
    message = generate_invite_email(
        jinja_env=jinja_env,
        from_addr=sender,
        to_addr=recipient,
        invitation_type=invitation_type,
        organization_id=organization_id,
        invitation_url=invitation_url,
        server_url=server_url,
        greeter_name=greeter_name,
        reply_to=reply_to,
    )

    write_mail_file_to_filesystem(message, output_dir, "invitation_mail")


@export_email.command(short_help="Export account create validation email")
@mail_templates_shared_options
@click.option(
    "--validation-code",
    type=ValidationCode,
    default=DEFAULT_VALIDATION_CODE.str,
    show_default=True,
)
def account_create(
    sender: EmailAddress,
    recipient: EmailAddress,
    validation_code: ValidationCode,
    server_url: str,
    output_dir: Path,
    template_dir: Path | None,
):
    jinja_env = get_environment(template_dir)
    message = _generate_account_create_validation_email(
        jinja_env=jinja_env,
        from_addr=sender,
        to_addr=recipient,
        validation_code=validation_code,
        server_url=server_url,
    )

    write_mail_file_to_filesystem(message, output_dir, "account_create_validation_email")


@export_email.command(short_help="Export account delete validation email")
@mail_templates_shared_options
@click.option(
    "--validation-code",
    type=ValidationCode,
    default=DEFAULT_VALIDATION_CODE.str,
    show_default=True,
)
def account_delete(
    sender: EmailAddress,
    recipient: EmailAddress,
    validation_code: ValidationCode,
    server_url: str,
    output_dir: Path,
    template_dir: Path | None,
):
    jinja_env = get_environment(template_dir)
    message = _generate_account_delete_validation_email(
        jinja_env=jinja_env,
        from_addr=sender,
        to_addr=recipient,
        validation_code=validation_code,
        server_url=server_url,
    )

    write_mail_file_to_filesystem(message, output_dir, "account_delete_validation_email")
