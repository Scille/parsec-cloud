# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import shlex
import subprocess
import tempfile
from argparse import ArgumentParser
from base64 import b64encode
from binascii import unhexlify
from collections.abc import Generator, Sequence
from dataclasses import dataclass, field
from datetime import UTC, datetime, timedelta
from enum import Enum, auto
from pathlib import Path

ROOT_DIR = Path(__file__).parent.parent.resolve()

ROOT_CERT_DIR = Path("Root")
INTERMEDIATE_CERT_DIR = Path("Intermediate")
LEAF_CERT_DIR = Path("Cert")
CRL_DIR = Path("CRL")

SCRIPT_LAST_MODIFICATION = Path(__file__).stat().st_mtime


type Sha256Fingerprint = bytes


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
    extensions: dict[str, str] = field(default_factory=dict)
    revoked: bool = False
    signing: list[CertificateConfig] = field(default_factory=list)
    """Certificates that will by signed by this certificate"""


now = datetime.now()
now_100y = now + timedelta(days=356 * 100)
not_before = now
not_after = now_100y

COMMON_NAME = "CN"
ORGANIZATION = "O"
ORGANIZATIONAL_UNIT = "OU"
SUBJECT_ALT_NAME = "subjectAltName"
SAN_EMAIL = "email"
EMAIL_ADDRESS = "emailAddress"
KEY_USAGE = "keyUsage"
EXTENDED_KEY_USAGE = "extendedKeyUsage"
BASIC_CONSTRAINTS = "basicConstraints"

TRUSTCHAINS = [
    CertificateConfig(
        name="black_mesa",
        type=CertificateType.Root,
        subject={COMMON_NAME: "Black Mesa CA", ORGANIZATION: "Black Mesa"},
        key_algorithm=RSAKeyAlgorithm(
            # Before changing, make sure that webpki support that size.
            # Last time I've checked, webpki support RSA key between 2048-8192 in size
            # https://docs.rs/rustls-webpki/latest/webpki/ring/index.html
            length=2048
        ),
        not_before=not_before,
        not_after=not_after,
        extensions={BASIC_CONSTRAINTS: "CA:TRUE"},
        signing=[
            CertificateConfig(
                name="alice",
                subject={COMMON_NAME: "Alice"},
                key_algorithm=RSAKeyAlgorithm(length=2048),
                not_before=not_before,
                not_after=not_after,
                extensions={SUBJECT_ALT_NAME: f"{SAN_EMAIL}:alice@black_mesa.corp"},
            ),
            CertificateConfig(
                name="bob",
                subject={COMMON_NAME: "Bob", EMAIL_ADDRESS: "bob@black-mesa.corp"},
                key_algorithm=RSAKeyAlgorithm(length=2048),
                not_before=not_before,
                not_after=not_after,
                extensions={EXTENDED_KEY_USAGE: "clientAuth"},
            ),
            CertificateConfig(
                name="old-boby",
                subject={COMMON_NAME: "Boby", EMAIL_ADDRESS: "boby@black-mesa.corp"},
                key_algorithm=RSAKeyAlgorithm(length=2048),
                not_before=not_before,
                not_after=not_before,  # Not a typo, we want old-boby cert to be expired
                extensions={BASIC_CONSTRAINTS: "CA:FALSE"},
            ),
            CertificateConfig(
                name="revoked_anomalous_materials_laboratories",
                type=CertificateType.Intermediate,
                subject={
                    COMMON_NAME: "Anomalous Materials Laboratories",
                    ORGANIZATIONAL_UNIT: "Anomalous Materials Laboratories",
                },
                key_algorithm=RSAKeyAlgorithm(length=2048),
                not_before=not_before,
                not_after=not_after,
                revoked=True,
                extensions={BASIC_CONSTRAINTS: "CA:TRUE"},
                signing=[
                    CertificateConfig(
                        name="gordon",
                        subject={COMMON_NAME: "Gordon", EMAIL_ADDRESS: "gordon@black-mesa.corp"},
                        key_algorithm=RSAKeyAlgorithm(length=2048),
                        not_before=not_before,
                        not_after=not_after,
                        extensions={SUBJECT_ALT_NAME: f"{SAN_EMAIL}:gordon@black-mesa.corp"},
                    ),
                ],
            ),
            CertificateConfig(
                name="revoked_breen",
                subject={COMMON_NAME: "Breen", EMAIL_ADDRESS: "breen@black-mesa.corp"},
                key_algorithm=RSAKeyAlgorithm(length=2048),
                not_before=not_before,
                not_after=not_after,
                revoked=True,
            ),
        ],
    ),
    CertificateConfig(
        name="aperture_science",
        type=CertificateType.Root,
        subject={COMMON_NAME: "Aperture Science CA", ORGANIZATION: "Aperture Science"},
        key_algorithm=RSAKeyAlgorithm(length=2048),
        not_before=not_before,
        not_after=not_after,
        extensions={BASIC_CONSTRAINTS: "CA:TRUE"},
        signing=[
            CertificateConfig(
                name="glados_dev_team",
                type=CertificateType.Intermediate,
                subject={COMMON_NAME: "Glados dev team", ORGANIZATIONAL_UNIT: "Glados dev team"},
                key_algorithm=RSAKeyAlgorithm(length=2048),
                not_before=not_before,
                not_after=not_after,
                extensions={BASIC_CONSTRAINTS: "CA:TRUE,pathlen:5"},
                signing=[
                    CertificateConfig(
                        name="mallory-sign",
                        subject={COMMON_NAME: "Mallory", EMAIL_ADDRESS: "mallory@black-mesa.corp"},
                        key_algorithm=RSAKeyAlgorithm(length=2048),
                        not_before=not_before,
                        not_after=not_after,
                        extensions={KEY_USAGE: "digitalSignature"},
                    ),
                    CertificateConfig(
                        name="mallory-encrypt",
                        subject={COMMON_NAME: "Mallory", EMAIL_ADDRESS: "mallory@black-mesa.corp"},
                        key_algorithm=RSAKeyAlgorithm(length=2048),
                        not_before=not_before,
                        not_after=not_after,
                        extensions={KEY_USAGE: "dataEncipherment"},
                    ),
                ],
            )
        ],
    ),
]


def main(output_dir: Path) -> None:
    output_dir.mkdir(parents=True, exist_ok=True)
    readme_file_content = [
        "# Test PKI certificates",
        "",
        "⚠️ Auto-generated file, don't edit by hand ! ⚠️",
        "",
        "This directory contains X509 certificates for test purpose.",
        "See `misc/gen_test_pki.py` for the generation script.",
        f"Last generation date: {datetime.fromtimestamp(SCRIPT_LAST_MODIFICATION, tz=UTC).isoformat()}",
        "",
        "Certificates:",
        "",
    ]

    all_revoked: dict[str, list[CertificateConfig]] = {}
    """Maps root certificate name to the list of revoked children"""

    for chain in TRUSTCHAINS:
        certifs = create_trustchain(chain, None, output_dir)
        for name, fingerprint in certifs.items():
            hex_fingerprint = fingerprint.hex().upper()
            coded_fingerprint = f"sha256-{b64encode(fingerprint).decode('utf-8')}"
            readme_file_content.append(
                f"- {name}: SHA256={hex_fingerprint} (aka `{coded_fingerprint}`)"
            )

        revoked = [child for child in chain.signing if child.revoked]
        if revoked:
            all_revoked[chain.name] = revoked

    for root_name, revoked_certs in all_revoked.items():
        generate_crl(root_name, revoked_certs, output_dir)

    if all_revoked:
        readme_file_content.append("")
        readme_file_content.append("CRL files:")
        readme_file_content.append("")
        for root_name, revoked_certs in all_revoked.items():
            revoked_names = ", ".join(c.name for c in revoked_certs)
            readme_file_content.append(
                f"- {CRL_DIR / root_name}.crl: revokes {revoked_names} (signed by {root_name})"
            )
        readme_file_content.append("")

    readme_file = output_dir / "README.md"
    readme_file.write_text("\n".join(readme_file_content))


def create_trustchain(
    chain: CertificateConfig, signer: CertificateConfig | None, output_dir: Path
) -> dict[str, Sha256Fingerprint]:
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
    cert_file = workdir / (chain.name + ".crt")
    pkcs12_file = workdir / (chain.name + ".pfx")

    if key_file.exists():
        print(f"Skipping {key_file} creation since it already exists")
    else:
        print(f"Creating key {key_file} of type {chain.key_algorithm}")
        key_file.parent.mkdir(parents=True, exist_ok=True)
        chain.key_algorithm.run_openssl_cmd(key_file)

        print(f"Creating cert {cert_file}")
        cert_file.parent.mkdir(parents=True, exist_ok=True)

        if signer is None:
            generate_self_signed_cert(chain, key_file, cert_file)
        else:
            generate_signed_cert(chain, signer, key_file, cert_file, output_dir)

        generate_pkcs12_file(chain, cert_file, key_file, pkcs12_file)

    generated = {}

    fingerprint = get_sha256_fingerprint(cert_file)
    generated[chain.name] = fingerprint

    for children in chain.signing:
        generated |= create_trustchain(children, chain, output_dir)

    return generated


def generate_self_signed_cert(chain: CertificateConfig, key_file: Path, cert_file: Path):
    check_run(
        [
            "openssl",
            "req",
            "-x509",  # Output a certificate instead of a CSR
            "-key",
            key_file,
            "-out",
            cert_file,
            "-not_before",
            format_date_for_openssl(chain.not_before),
            "-not_after",
            format_date_for_openssl(chain.not_after),
            "-subj",
            gen_subject_option_arg(chain.subject),
            "-batch",
            *gen_extensions_args(chain.extensions),
        ]
    )


def format_date_for_openssl(date: datetime) -> str:
    # cspell:ignore YYMMDDHHMMSSZ
    """Openssl expect the date to be formatted in the follow schema: [CC]YYMMDDHHMMSSZ"""
    return date.astimezone(UTC).strftime("%Y%m%d%H%M%SZ")


def gen_subject_option_arg(subject: dict[str, str]) -> str:
    return "".join(f"/{k}={v}" for k, v in subject.items())


def gen_extensions_args(extensions: dict[str, str]) -> Generator[str, None, None]:
    return (f"--addext={k}={v}" for k, v in extensions.items())


def generate_signed_cert(
    chain: CertificateConfig,
    signer: CertificateConfig,
    key_file: Path,
    cert_file: Path,
    output_dir: Path,
) -> None:
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
            "-not_before",
            format_date_for_openssl(chain.not_before),
            "-not_after",
            format_date_for_openssl(chain.not_after),
            "-subj",
            gen_subject_option_arg(chain.subject),
            "-batch",
            *gen_extensions_args(chain.extensions),
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
            # Not using "-preserve_dates" because during testing, it does not seems to be taken into account
            "-not_before",
            format_date_for_openssl(chain.not_before),
            "-not_after",
            format_date_for_openssl(chain.not_after),
            "-copy_extensions",
            "copyall",
        ]
        + (["-CAserial", ca_serial_file] if ca_serial_file.exists() else ["-CAcreateserial"])
    )


def get_sha256_fingerprint(cert_file: Path) -> Sha256Fingerprint:
    raw_fingerprint = subprocess.check_output(
        [
            "openssl",
            "x509",
            "-in",
            cert_file,
            "-inform",
            "pem",
            "-noout",
            "-fingerprint",
            "-sha256",
        ]
    )
    PREFIX = b"sha256 Fingerprint="
    assert raw_fingerprint.startswith(PREFIX)
    return unhexlify(raw_fingerprint[len(PREFIX) :].strip().decode("utf-8").replace(":", ""))


def generate_pkcs12_file(
    chain: CertificateConfig, cert_file: Path, key_file: Path, out_file: Path
) -> None:
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


def generate_crl(
    root_name: str,
    revoked_certs: list[CertificateConfig],
    output_dir: Path,
) -> None:
    ca_workdir = output_dir / ROOT_CERT_DIR
    ca_cert = ca_workdir / (root_name + ".crt")
    ca_key = ca_workdir / (root_name + ".key")

    crl_workdir = output_dir / CRL_DIR
    crl_workdir.mkdir(parents=True, exist_ok=True)
    crl_file = crl_workdir / (root_name + ".crl")

    # By default `openssl ca` reads config from `/usr/lib/ssl/openssl.cnf`,
    # we must overwrite this with our own config.
    #
    # This is typically needed since OpenSSL CRLs are tracked with an index
    # and a database that are handled as files.
    with tempfile.TemporaryDirectory() as tmpdir:
        tmpdir_path = Path(tmpdir)

        index_file = tmpdir_path / "index.txt"
        index_file.touch()
        index_attr_file = tmpdir_path / "index.txt.attr"
        index_attr_file.write_text("unique_subject = no\n")
        crlnumber_file = tmpdir_path / "crlnumber"
        crlnumber_file.write_text("1000\n")

        openssl_conf = tmpdir_path / "openssl.cnf"
        openssl_conf.write_text(
            f"""[ca]
default_ca = CA_default

[CA_default]
database = {index_file}
crlnumber = {crlnumber_file}
default_md = sha256
# How long before next CRL, we use 100 years so the validation
# never complains a new CRL should have been issued.
default_crl_days = 36500
"""
        )

        # Revoke each certificate
        for cert in revoked_certs:
            cert_workdir = output_dir / cert.type.get_path()
            cert_file = cert_workdir / (cert.name + ".crt")
            check_run(
                [
                    "openssl",
                    "ca",
                    "-config",
                    openssl_conf,
                    "-cert",
                    ca_cert,
                    "-keyfile",
                    ca_key,
                    "-revoke",
                    cert_file,
                ]
            )

        # Build the CRL
        check_run(
            [
                "openssl",
                "ca",
                "-config",
                openssl_conf,
                "-cert",
                ca_cert,
                "-keyfile",
                ca_key,
                # cspell: disable-next-line
                "-gencrl",
                "-out",
                crl_file,
            ]
        )

    print(f"Generated CRL {crl_file}")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument(
        "--output-dir", default=ROOT_DIR / "libparsec/crates/platform_pki/test-pki/", type=Path
    )

    args = parser.parse_args()
    main(args.output_dir)
