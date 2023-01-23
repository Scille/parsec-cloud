# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import argparse
import trio
import tempfile
import json
from pathlib import Path

from parsec._parsec import (
    BackendAddr,
    BackendOrganizationBootstrapAddr,
    DeviceLabel,
    HumanHandle,
    OrganizationID,
    save_device_with_password_in_config,
)
from parsec.core.local_device import AvailableDevice, get_available_device, key_file_serializer
from parsec.core.invite import bootstrap_organization


async def create_org_and_device(
    human_handle: HumanHandle, device_label: DeviceLabel, password: str, backend_addr: BackendAddr
):
    device = await bootstrap_organization(
        backend_addr, human_handle=human_handle, device_label=device_label
    )
    config_dir = tempfile.mkdtemp(prefix="megashark-e2e-")
    save_device_with_password_in_config(
        config_dir,
        device,
        password,
    )
    return Path(get_available_device(Path(config_dir), device.slug).key_file_path)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    parser.add_argument("--label", default="Hideout")
    parser.add_argument("--email", default="zana@wraeclast.nz")
    parser.add_argument("--name", default="Zana")
    parser.add_argument("--password", default="P@ssw0rd")
    parser.add_argument("--org", default="Wraeclast")
    parser.add_argument("--backend-addr", default="parsec://localhost:6888?no_ssl=true")
    parser.add_argument("--output", "-o")

    args = parser.parse_args()
    human_handle = HumanHandle(
        email=args.email,
        label=args.name,
    )
    org_id = OrganizationID(args.org)
    device_label = DeviceLabel(args.label)
    backend_addr = BackendOrganizationBootstrapAddr.build(
        backend_addr=BackendAddr.from_url(args.backend_addr),
        organization_id=org_id,
        token=None,
    )

    key_file_path = trio.run(
        create_org_and_device, human_handle, device_label, args.password, backend_addr
    )

    data = key_file_serializer.loads(key_file_path.read_bytes())

    data["salt"] = [c for c in data["salt"]]
    data["ciphertext"] = [c for c in data["ciphertext"]]
    data["human_handle"] = [data["human_handle"].email, data["human_handle"].label]
    data["device_label"] = data["device_label"].str
    data["device_id"] = data["device_id"].str
    data["organization_id"] = data["organization_id"].str
    data["type"] = "password"

    with open(args.output, "w+") as fd:
        fd.write("/*\n")
        fd.write(" * Auto-generated.\n\n")
        fd.write(" * A device that can be stored to local storage for tests.\n")
        fd.write(f" * Backend Addr: {args.backend_addr}\n")
        fd.write(f" * DeviceLabel: {args.label}\n")
        fd.write(f" * HumanHandle.label: {args.name}\n")
        fd.write(f" * HumanHandle.email: {args.email}\n")
        fd.write(f" * OrganizationID: {args.org}\n")
        fd.write(f" * Password: {args.password}\n")
        fd.write("*/\n\n")
        fd.write(f"export const DEFAULT_DEVICE='{json.dumps([data])}'\n")
