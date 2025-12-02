# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import shlex
import subprocess
from argparse import ArgumentParser, Namespace
from collections.abc import Sequence
from dataclasses import dataclass, field
from datetime import datetime, timedelta
from enum import Enum, auto
from pathlib import Path

ROOT_CERT_DIR = Path("Root")
INTERMEDIATE_CERT_DIR = Path("Intermediate")
LEAF_CERT_DIR = Path("Cert")

SCRIPT_LAST_MODIFICATION = Path(__file__).stat().st_mtime


def file_need_update(file: Path) -> bool:
    try:
        return file.stat().st_mtime < SCRIPT_LAST_MODIFICATION
    except FileNotFoundError:
        return True


@dataclass
class KeyAlgorithm:
    id: str = field(default="", init=False)

    def run_openssl_cmd(self, path: Path):
        raise NotImplementedError


@dataclass
class RSAKeyAlgorithm(KeyAlgorithm):
    length: int
    id = "RSA"

    def run_openssl_cmd(self, path: Path):
        return check_run(["openssl", "genrsa", "-out", path, str(self.length)])


def check_run(args: Sequence[str | Path]) -> int:
    print(f"+ {shlex.join(map(str, args))}")
    return subprocess.check_call(args)


class CertificateType(Enum):
    Root = auto()
    Intermediate = auto()
    Leaf = auto()

    def get_path(self) -> Path:
        match self:
            case CertificateType.Root:
                return ROOT_CERT_DIR
            case CertificateType.Intermediate:
                return INTERMEDIATE_CERT_DIR
            case CertificateType.Leaf:
                return LEAF_CERT_DIR
            case _:
                assert False, f"{self} not handled"


@dataclass
class CertificateConfig:
    name: str
    subject: dict[str, str]
    key_algorithm: KeyAlgorithm
    not_before: datetime
    """Certificate is not valid before that date"""
    not_after: datetime
    """Certificate is not valid after that date"""
    type: CertificateType = CertificateType.Leaf
    signing: list[CertificateConfig] = field(default_factory=list)
    """Certificates that will by signed by this certificate"""


now = datetime.now()
now_24h = now + timedelta(hours=24)

TRUSTCHAINS = [
    CertificateConfig(
        name="black_mesa",
        type=CertificateType.Root,
        subject=dict(CN="Black Mesa CA", O="Black Mesa"),
        key_algorithm=RSAKeyAlgorithm(length=2048),
        not_before=now,
        not_after=now + timedelta(hours=24),
        signing=[
            CertificateConfig(
                name="alice",
                subject=dict(CN="Alice", emailAddress="alice@black-mesa.corp"),
                key_algorithm=RSAKeyAlgorithm(length=2048),
                not_before=now,
                not_after=now_24h,
            ),
            CertificateConfig(
                name="bob",
                subject=dict(CN="Bob", emailAddress="bob@black-mesa.corp"),
                key_algorithm=RSAKeyAlgorithm(length=2048),
                not_before=now,
                not_after=now_24h,
            ),
            CertificateConfig(
                name="old-boby",
                subject=dict(CN="Boby", emailAddress="boby@black-mesa.corp"),
                key_algorithm=RSAKeyAlgorithm(length=2048),
                not_before=now,
                not_after=now,  # Not a typo, we want old-boby cert to be expired
            ),
        ],
    ),
    CertificateConfig(
        name="aperture_science",
        type=CertificateType.Root,
        subject=dict(CN="Aperture Science CA", O="Aperture Science"),
        key_algorithm=RSAKeyAlgorithm(length=2048),
        not_before=now,
        not_after=now + timedelta(hours=24),
        signing=[
            CertificateConfig(
                name="glados_dev_team",
                type=CertificateType.Intermediate,
                subject=dict(CN="Glados dev team", OU="Glados dev team"),
                key_algorithm=RSAKeyAlgorithm(length=2048),
                not_before=now,
                not_after=now_24h,
                signing=[
                    CertificateConfig(
                        name="mallory",
                        subject=dict(CN="Mallory", emailAddress="mallory@black-mesa.corp"),
                        key_algorithm=RSAKeyAlgorithm(length=2048),
                        not_before=now,
                        not_after=now_24h,
                    )
                ],
            )
        ],
    ),
]


def main():
    args = parse_args()
    output_dir: Path = args.output_dir

    output_dir.mkdir(parents=True, exist_ok=True)
    gitignore = output_dir / ".gitignore"
    gitignore.write_text("*")
    for chain in TRUSTCHAINS:
        create_trustchain(chain, None, output_dir)


def parse_args() -> Namespace:
    parser = ArgumentParser()
    parser.add_argument("--output-dir", default=Path("test-pki"), type=Path)

    return parser.parse_args()


def create_trustchain(chain: CertificateConfig, signer: CertificateConfig | None, output_dir: Path):
    if signer:
        print(f"Creating trustchain for {chain.name} signed by {signer.name}")
        # Inherit signer subject
        old_subject = chain.subject
        chain.subject = {**signer.subject, **old_subject}
    else:
        print(f"Creating trustchain for {chain.name}")

    workdir = output_dir / chain.type.get_path()

    # Create key file
    key_file = workdir / (chain.name + ".key")

    if file_need_update(key_file):
        print(f"Creating key {key_file} of type {chain.key_algorithm}")
        key_file.parent.mkdir(parents=True, exist_ok=True)
        chain.key_algorithm.run_openssl_cmd(key_file)

    cert_file = workdir / (chain.name + ".crt")
    if file_need_update(cert_file):
        print(f"Creating cert {cert_file}")
        cert_file.parent.mkdir(parents=True, exist_ok=True)

        if signer is None:
            generate_self_signed_cert(chain, key_file, cert_file)
        else:
            generate_signed_cert(chain, signer, key_file, cert_file, output_dir)

        pkcs12_file = workdir / (chain.name + ".pfx")
        generate_pkcs12_file(chain, cert_file, key_file, pkcs12_file)

    for children in chain.signing:
        create_trustchain(children, chain, output_dir)


def generate_self_signed_cert(chain, key_file: Path, cert_file: Path):
    check_run(
        [
            "openssl",
            "req",
            "-x509",  # Output a certificate instead of a CSR
            "-key",
            key_file,
            "-out",
            cert_file,
            "-subj",
            gen_subject_option_arg(chain.subject),
            "-batch",
        ]
    )


def gen_subject_option_arg(subject: dict[str, str]) -> str:
    return "".join(f"/{k}={v}" for k, v in subject.items())


def generate_signed_cert(
    chain: CertificateConfig,
    signer: CertificateConfig,
    key_file: Path,
    cert_file: Path,
    output_dir: Path,
):
    csr_file = cert_file.with_suffix(".csr")
    check_run(
        [
            "openssl",
            "req",
            "-new",
            "-key",
            key_file,
            "-out",
            csr_file,
            "-subj",
            gen_subject_option_arg(chain.subject),
            "-batch",
        ]
    )

    ca_workdir = output_dir / signer.type.get_path()
    ca_cert = ca_workdir / (signer.name + ".crt")
    ca_key = ca_workdir / (signer.name + ".key")
    ca_serial_file = ca_workdir / (signer.name + ".srl")
    check_run(
        [
            "openssl",
            "x509",
            "-req",
            "-in",
            csr_file,
            "-out",
            cert_file,
            "-CA",
            ca_cert,
            "-CAkey",
            ca_key,
            "-copy_extensions",
            "copyall",
        ]
        + (["-CAserial", ca_serial_file] if ca_serial_file.exists() else ["-CAcreateserial"])
    )


def generate_pkcs12_file(chain: CertificateConfig, cert_file: Path, key_file: Path, out_file: Path):
    check_run(
        [
            "openssl",
            "pkcs12",
            "-export",
            "-in",
            cert_file,
            "-inkey",
            key_file,
            "-out",
            out_file,
            "-passout",
            "pass:",  # Do not use a password
            "-name",
            chain.name,
        ]
    )


if __name__ == "__main__":
    main()
