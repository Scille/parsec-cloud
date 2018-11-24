import click
import traceback


def _generate_bad_cmd(exc, hint=None):
    error_msg = "".join(
        [
            "Not available: Importing this module has failed with error:\n\n",
            *traceback.format_exception(exc, exc, exc.__traceback__),
            f"\n\n{hint}\n" if hint else "",
        ]
    )

    @click.command(
        context_settings=dict(ignore_unknown_options=True),
        help=f"Not available{' (' + hint + ')' if hint else ''}",
    )
    @click.argument("args", nargs=-1, type=click.UNPROCESSED)
    def bad_cmd(args):
        raise SystemExit(error_msg)

    return bad_cmd


try:
    from parsec.core.cli import core_cmd
except ImportError as exc:
    core_cmd = _generate_bad_cmd(exc)


try:
    from parsec.backend.cli import backend_cmd, init_cmd, revoke_cmd
except ImportError as exc:
    backend_cmd = _generate_bad_cmd(exc)
    init_cmd = _generate_bad_cmd(exc)
    revoke_cmd = _generate_bad_cmd(exc)


@click.group()
def cli():
    pass


cli.add_command(core_cmd, "core")
cli.add_command(backend_cmd, "backend")
cli.add_command(init_cmd, "init")
cli.add_command(revoke_cmd, "revoke_user")


if __name__ == "__main__":
    cli()
