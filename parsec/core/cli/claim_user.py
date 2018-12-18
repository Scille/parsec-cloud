import click


async def _claim_user(backend_addr, token, device_id, password, pkcs11):
    device = generate_new_device(device_id, backend_addr, root_signing_key.verify_key)

    try:
        if pkcs11:
            token_id = click.prompt("PCKS11 token id", type=int)
            key_id = click.prompt("PCKS11 key id", type=int)
            pin = click.prompt("PCKS11 pin", hide_input=True)
            save_device_with_pkcs11(config.config_dir, device, token_id, key_id)

        else:
            if password is None:
                password = click.prompt("password", hide_input=True, confirmation_prompt=True)
            save_device_with_password(config.config_dir, device, password)

    except DeviceManagerError as exc:
        raise SystemExit(f"Cannot save device {device.device_id}: {exc}")

    public_key = PublicKey.generate()
    signing_key = SigningKey.generate()

    async with backend_anonymous_cmds_factory(backend_addr) as cmds:
        invitation_creator = await cmds.user_get_invitation_creator(device_id.user_id)

        encrypted_claim = generate_user_encrypted_claim(
            invitation_creator.public_key, token, device_id, public_key, verify_key
        )
        await cmds.user_claim(device_id.user_id, encrypted_claim)

        if pkcs11:
            save_device_with_password(config_dir, device_id, token_id, key_id, pin)
        else:
            save_device_with_password(config_dir, device_id, password)

    async with logged_core_factory(config, device) as core:
        async with spinner("Waiting for invitation reply"):
            claimd_args = await core.user_claim(claimd_user_id)
        click.secho("✓", fg="green")

        display_device = click.style(claimd_args[0], fg="yellow")
        async with spinner(f"Adding {display_device} to backend"):
            await core.user_create(*claimd_args)
        click.secho("✓", fg="green")


@click.command()
@core_config_options
@click.option("--device", "-D", type=DeviceID, required=True)
@click.option("--token", "-T", required=True)
@click.option("--backend-addr", "-B", required=True)
@click.password_option("--password", "-P")
@click.option("--pkcs11", is_flag=True)
def claim_user(config, backend_addr, device, token, password, pkcs11, **kwargs):
    if password and pkcs11:
        raise SystemExit("Password are PKCS11 options are exclusives.")

    trio.run(_claim_user(backend_addr, config, device, token, password, pkcs11))
