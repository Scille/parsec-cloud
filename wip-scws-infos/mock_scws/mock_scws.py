# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
Mock SCWS (SmartCard Web Service) — a development/testing stand-in for the
real native SCWS service.  Implements the HTTP API that scwsapi.js talks to.

Usage:
    uv run mock_scws.py [--port 41231]
    uv run mock_scws.py [--port 41231] --pki-dir /path/to/test-pki

The --pki-dir option points to a directory with the following layout:

    test-pki/
    ├── Cert/           # End-entity certificates (.crt + .key pairs)
    │   ├── alice.crt
    │   ├── alice.key
    │   ├── bob.crt
    │   ├── bob.key
    │   └── ...
    ├── Intermediate/   # (optional) Intermediate CA certs
    └── Root/           # (optional) Root CA certs

Each .crt is a PEM X.509 certificate; each .key is the matching PEM private
key (unencrypted).  Only Cert/ entries that have *both* .crt and .key are
loaded onto the mock smartcard token.
"""

from __future__ import annotations

import argparse
import base64
import hashlib
import os
import secrets
import time
import uuid
from dataclasses import dataclass, field
from pathlib import Path

from cryptography import x509
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding, rsa
from cryptography.hazmat.primitives.asymmetric.utils import Prehashed
from flask import Flask, Response, jsonify, request

# ---------------------------------------------------------------------------
# Data model
# ---------------------------------------------------------------------------

def _hex_handle() -> str:
    return secrets.token_hex(16).upper()


@dataclass
class MockObject:
    handle: str = field(default_factory=_hex_handle)
    obj_type: str = "certificate"  # certificate | publicKey | privateKey | dataContainer
    ck_label: str = ""
    ck_id: str = ""
    subject_name: str = ""
    issuer_name: str = ""
    root_cert: bool = False
    ca_cert: bool = False
    not_before: str = ""
    not_after: str = ""
    usages: list[dict] = field(default_factory=list)
    token_flag: bool = True
    private: bool = False
    modifiable: bool = True
    vslot: int = 0
    has_private_key: bool = False
    container: str = "none"
    application: str | None = None
    # Backing crypto material (only for objects generated at startup)
    _private_key: rsa.RSAPrivateKey | None = field(default=None, repr=False)
    _certificate: x509.Certificate | None = field(default=None, repr=False)
    _pem_cert: bytes | None = field(default=None, repr=False)

    def to_summary(self) -> dict:
        d: dict = {
            "type": self.obj_type,
            "handle": self.handle,
            "container": self.container,
            "token": self.token_flag,
            "private": self.private,
            "modifiable": self.modifiable,
            "vslot": self.vslot,
            "ckLabel": self.ck_label,
        }
        if self.ck_id:
            d["ckId"] = self.ck_id
        if self.application is not None:
            d["application"] = self.application
        if self.obj_type == "certificate":
            d.update({
                "subjectName": self.subject_name,
                "issuerName": self.issuer_name,
                "rootCert": self.root_cert,
                "caCert": self.ca_cert,
                "notBefore": self.not_before,
                "notAfter": self.not_after,
                "usages": self.usages,
                "hasPrivateKey": self.has_private_key,
            })
        return d

    def to_detail(self) -> dict:
        d = {"object": self.to_summary()}
        if self.obj_type == "certificate" and self._certificate is not None:
            cert = self._certificate
            d["details"] = {
                "subject": _x509_name_str(cert.subject),
                "issuer": _x509_name_str(cert.issuer),
                "notBefore": self.not_before,
                "notAfter": self.not_after,
                "serial": format(cert.serial_number, "X"),
                "version": cert.version.value + 1,
                "publicKeyAlg": "RSA",
                "publicKeySize": cert.public_key().key_size,
                "signatureAlg": "sha256WithRSAEncryption",
            }
        return d


def _x509_name_str(name: x509.Name) -> str:
    parts = []
    for attr in name:
        short = attr.rfc4514_attribute_name
        parts.append(f"{short:10s} = {attr.value}")
    return "\n".join(parts)


@dataclass
class PinInfo:
    label: str = "PIN Global"
    logged_in: bool = False
    expected_pin: str = "1234"


@dataclass
class MockToken:
    handle: str = field(default_factory=_hex_handle)
    reader_name: str = ""
    serial: str = ""
    full_serial: str = ""
    model: str = "MockCard v1.0"
    manufacturer: str = "Mock"
    label: str = "Mock Token"
    pins: list[PinInfo] = field(default_factory=lambda: [PinInfo()])
    objects: list[MockObject] = field(default_factory=list)

    def info_list(self) -> list[dict]:
        infos = []
        for pin in self.pins:
            infos.append({
                "manufacturerId": self.manufacturer,
                "model": self.model,
                "serialNumber": self.serial,
                "label": self.label,
                "pinLabel": pin.label,
                "puksAvailable": 1,
                "isPinExpired": False,
                "isUnblockBlocked": False,
                "isPukSupported": True,
                "flags": 1292,
                "Vslotflags": 0,
                "pinUseable": True,
                "userLogged": pin.logged_in,
                "isBioActivationRequired": False,
                "isBioFacialSupported": False,
                "isBioFingerprintSupported": False,
            })
        return infos


@dataclass
class MockReader:
    name: str = "Mock Smart Card Reader 0"
    card_present: bool = True
    status: str = "ok"
    reader_type: str = "normal"

    def to_dict(self) -> dict:
        return {
            "name": self.name,
            "card": self.card_present,
            "status": self.status,
            "type": self.reader_type,
            "isnfciosavailable": "false",
        }


@dataclass
class Environment:
    env_id: str = field(default_factory=_hex_handle)
    rsa_key: rsa.RSAPrivateKey = field(default_factory=lambda: rsa.generate_private_key(65537, 2048))
    tokens: dict[str, MockToken] = field(default_factory=dict)
    readers: list[MockReader] = field(default_factory=list)
    # Maps object-handle -> MockObject across all tokens
    all_objects: dict[str, MockObject] = field(default_factory=dict)
    # CA certificates for chain building (Root + Intermediate).
    # Keyed by the subject Name's rfc4514 string.
    ca_certs: dict[str, MockObject] = field(default_factory=dict)
    credential_enc_key: bytes | None = None


# ---------------------------------------------------------------------------
# Certificate generation helpers
# ---------------------------------------------------------------------------

def _generate_cert_and_key(
    cn: str,
    issuer_cn: str,
    usages: list[dict],
    vslot: int = 0,
) -> tuple[MockObject, MockObject, MockObject]:
    """Return (cert_obj, pubkey_obj, privkey_obj) backed by real crypto."""
    key = rsa.generate_private_key(65537, 2048)
    subject = x509.Name([
        x509.NameAttribute(x509.oid.NameOID.COMMON_NAME, cn),
        x509.NameAttribute(x509.oid.NameOID.ORGANIZATION_NAME, "Mock Org"),
    ])
    issuer = x509.Name([
        x509.NameAttribute(x509.oid.NameOID.COMMON_NAME, issuer_cn),
    ])
    import datetime as dt
    now = dt.datetime.now(dt.timezone.utc)
    cert = (
        x509.CertificateBuilder()
        .subject_name(subject)
        .issuer_name(issuer)
        .public_key(key.public_key())
        .serial_number(x509.random_serial_number())
        .not_valid_before(now)
        .not_valid_after(now + dt.timedelta(days=365 * 3))
        .sign(key, hashes.SHA256())
    )
    pem = cert.public_bytes(serialization.Encoding.PEM)
    ck_id = " ".join(f"{b:02X}" for b in os.urandom(20))
    not_before = cert.not_valid_before_utc.strftime("%Y-%m-%dT%H:%M:%SZ")
    not_after = cert.not_valid_after_utc.strftime("%Y-%m-%dT%H:%M:%SZ")

    cert_obj = MockObject(
        obj_type="certificate",
        ck_label=f"{cn} cert",
        ck_id=ck_id,
        subject_name=cn,
        issuer_name=issuer_cn,
        not_before=not_before,
        not_after=not_after,
        usages=usages,
        has_private_key=True,
        vslot=vslot,
        _private_key=key,
        _certificate=cert,
        _pem_cert=pem,
    )
    pub_obj = MockObject(
        obj_type="publicKey",
        ck_label=f"{cn} pubkey",
        ck_id=ck_id,
        vslot=vslot,
    )
    priv_obj = MockObject(
        obj_type="privateKey",
        ck_label=f"{cn} privkey",
        ck_id=ck_id,
        private=True,
        vslot=vslot,
        _private_key=key,
    )
    return cert_obj, pub_obj, priv_obj


# ---------------------------------------------------------------------------
# Loading certificates from a PKI directory
# ---------------------------------------------------------------------------

# Map X.509 Extended Key Usage OIDs to SCWS usage short/long names
_EKU_MAP: dict[str, dict] = {
    "1.3.6.1.5.5.7.3.1": {"shortName": "serverAuth", "longName": "TLS Web Server Authentication", "oid": "1.3.6.1.5.5.7.3.1"},
    "1.3.6.1.5.5.7.3.2": {"shortName": "clientAuth", "longName": "TLS Web Client Authentication", "oid": "1.3.6.1.5.5.7.3.2"},
    "1.3.6.1.5.5.7.3.3": {"shortName": "codeSigning", "longName": "Code Signing", "oid": "1.3.6.1.5.5.7.3.3"},
    "1.3.6.1.5.5.7.3.4": {"shortName": "emailProtection", "longName": "E-mail Protection", "oid": "1.3.6.1.5.5.7.3.4"},
    "1.3.6.1.5.5.7.3.8": {"shortName": "timeStamping", "longName": "Time Stamping", "oid": "1.3.6.1.5.5.7.3.8"},
    "1.3.6.1.5.5.7.3.9": {"shortName": "OCSPSigning", "longName": "OCSP Signing", "oid": "1.3.6.1.5.5.7.3.9"},
    "1.3.6.1.4.1.311.20.2.2": {"shortName": "msSmartcardLogin", "longName": "Microsoft Smartcard Login", "oid": "1.3.6.1.4.1.311.20.2.2"},
    "1.3.6.1.5.2.3.4": {"shortName": "pkInitClientAuth", "longName": "PKINIT Client Auth", "oid": "1.3.6.1.5.2.3.4"},
}


def _derive_usages(cert: x509.Certificate) -> list[dict]:
    """Derive SCWS-style usage list from X.509 extensions."""
    usages: list[dict] = []

    # Try Extended Key Usage first
    try:
        eku = cert.extensions.get_extension_for_class(x509.ExtendedKeyUsage)
        for usage_oid in eku.value:
            dotted = usage_oid.dotted_string
            if dotted in _EKU_MAP:
                usages.append(_EKU_MAP[dotted])
            else:
                usages.append({"shortName": usage_oid._name or dotted, "oid": dotted})
        if usages:
            return usages
    except x509.ExtensionNotFound:
        pass

    # Fall back to Key Usage bit field
    try:
        ku = cert.extensions.get_extension_for_class(x509.KeyUsage)
        v = ku.value
        if v.digital_signature:
            usages.append({"shortName": "clientAuth"})
        if v.key_encipherment or v.key_agreement:
            usages.append({"shortName": "encrypt"})
        if v.data_encipherment:
            usages.append({"shortName": "encrypt"})
        if v.content_commitment:  # non_repudiation
            usages.append({"shortName": "timeStamping"})
        if v.key_cert_sign or v.crl_sign:
            usages.append({"shortName": "anyCA"})
        if usages:
            return usages
    except x509.ExtensionNotFound:
        pass

    return [{"shortName": "all"}]


def _get_subject_cn(name: x509.Name) -> str:
    """Extract the CN from an X.509 Name, with fallback to O or full RDN."""
    cns = name.get_attributes_for_oid(x509.oid.NameOID.COMMON_NAME)
    if cns:
        return cns[-1].value  # last (most specific) CN
    orgs = name.get_attributes_for_oid(x509.oid.NameOID.ORGANIZATION_NAME)
    if orgs:
        return orgs[0].value
    return name.rfc4514_string()


def _load_cert_key_pair(
    crt_path: Path,
    key_path: Path,
    vslot: int = 0,
) -> tuple[MockObject, MockObject, MockObject]:
    """Load a .crt + .key pair from disk and return (cert_obj, pubkey_obj, privkey_obj)."""
    pem_cert_data = crt_path.read_bytes()
    pem_key_data = key_path.read_bytes()

    cert = x509.load_pem_x509_certificate(pem_cert_data)
    key = serialization.load_pem_private_key(pem_key_data, password=None)

    subject_cn = _get_subject_cn(cert.subject)
    issuer_cn = _get_subject_cn(cert.issuer)
    not_before = cert.not_valid_before_utc.strftime("%Y-%m-%dT%H:%M:%SZ")
    not_after = cert.not_valid_after_utc.strftime("%Y-%m-%dT%H:%M:%SZ")
    usages = _derive_usages(cert)

    # Derive ckId from the certificate's Subject Key Identifier if available,
    # otherwise hash the public key.
    try:
        ski = cert.extensions.get_extension_for_class(x509.SubjectKeyIdentifier)
        ck_id_bytes = ski.value.digest
    except x509.ExtensionNotFound:
        pub_der = cert.public_key().public_bytes(serialization.Encoding.DER, serialization.PublicFormat.SubjectPublicKeyInfo)
        ck_id_bytes = hashlib.sha1(pub_der).digest()

    ck_id = " ".join(f"{b:02X}" for b in ck_id_bytes)
    label = crt_path.stem  # e.g. "alice", "mallory-sign"

    is_ca = False
    try:
        bc = cert.extensions.get_extension_for_class(x509.BasicConstraints)
        is_ca = bc.value.ca
    except x509.ExtensionNotFound:
        pass

    cert_obj = MockObject(
        obj_type="certificate",
        ck_label=f"{subject_cn} from {issuer_cn}",
        ck_id=ck_id,
        subject_name=subject_cn,
        issuer_name=issuer_cn,
        root_cert=(subject_cn == issuer_cn),
        ca_cert=is_ca,
        not_before=not_before,
        not_after=not_after,
        usages=usages,
        has_private_key=True,
        vslot=vslot,
        _private_key=key,
        _certificate=cert,
        _pem_cert=pem_cert_data,
    )
    pub_obj = MockObject(
        obj_type="publicKey",
        ck_label=f"{label} pubkey",
        ck_id=ck_id,
        vslot=vslot,
    )
    priv_obj = MockObject(
        obj_type="privateKey",
        ck_label=f"{label} privkey",
        ck_id=ck_id,
        private=True,
        vslot=vslot,
        _private_key=key,
    )
    return cert_obj, pub_obj, priv_obj


def _load_ca_cert(crt_path: Path) -> MockObject:
    """Load a CA certificate (no private key needed)."""
    pem_data = crt_path.read_bytes()
    cert = x509.load_pem_x509_certificate(pem_data)
    subject_cn = _get_subject_cn(cert.subject)
    issuer_cn = _get_subject_cn(cert.issuer)
    not_before = cert.not_valid_before_utc.strftime("%Y-%m-%dT%H:%M:%SZ")
    not_after = cert.not_valid_after_utc.strftime("%Y-%m-%dT%H:%M:%SZ")

    return MockObject(
        obj_type="certificate",
        ck_label=f"{subject_cn} CA",
        subject_name=subject_cn,
        issuer_name=issuer_cn,
        root_cert=(cert.subject == cert.issuer),
        ca_cert=True,
        not_before=not_before,
        not_after=not_after,
        usages=[{"shortName": "anyCA"}],
        has_private_key=False,
        _certificate=cert,
        _pem_cert=pem_data,
    )


def _load_pki_dir(pki_dir: Path) -> tuple[list[MockObject], dict[str, MockObject]]:
    """Scan a PKI directory and load certificates.

    Returns:
        - A flat list of end-entity MockObjects (cert + pubkey + privkey per pair)
          loaded from the Cert/ folder.
        - A dict of CA MockObjects (from Root/ and Intermediate/) keyed by the
          certificate subject's rfc4514 string, used for chain building.
    """
    # --- Load CA certs from Root/ and Intermediate/ ---
    ca_certs: dict[str, MockObject] = {}
    for subdir in ("Root", "Intermediate"):
        ca_dir = pki_dir / subdir
        if not ca_dir.is_dir():
            continue
        for crt_path in sorted(ca_dir.glob("*.crt")):
            ca_obj = _load_ca_cert(crt_path)
            # Key by the rfc4514 subject so we can match issuer → subject
            subject_key = ca_obj._certificate.subject.rfc4514_string()
            ca_certs[subject_key] = ca_obj
            print(f"  CA cert: {crt_path.stem} ({ca_obj.subject_name})"
                  f"{' [root]' if ca_obj.root_cert else ' [intermediate]'}")

    # --- Load end-entity certs from Cert/ ---
    cert_dir = pki_dir / "Cert"
    if not cert_dir.is_dir():
        cert_dir = pki_dir

    crt_files = sorted(cert_dir.glob("*.crt"))
    objects: list[MockObject] = []
    loaded: list[str] = []

    for crt_path in crt_files:
        key_path = crt_path.with_suffix(".key")
        if not key_path.exists():
            continue
        cert_obj, pub_obj, priv_obj = _load_cert_key_pair(crt_path, key_path, vslot=0)
        objects.extend([cert_obj, pub_obj, priv_obj])
        loaded.append(crt_path.stem)

    if loaded:
        print(f"Loaded {len(loaded)} end-entity certificate(s): {', '.join(loaded)}")
    else:
        print(f"Warning: no .crt/.key pairs found in {cert_dir}")

    if ca_certs:
        print(f"Loaded {len(ca_certs)} CA certificate(s) for chain building")

    return objects, ca_certs


# ---------------------------------------------------------------------------
# Build default mock state
# ---------------------------------------------------------------------------

def build_default_state(
    pki_dir: Path | None = None,
) -> tuple[list[MockReader], list[MockToken], dict[str, MockObject]]:
    """Build a realistic default smartcard with multiple certificates.

    If pki_dir is provided, certificates are loaded from disk instead of
    being auto-generated.

    Returns (readers, tokens, ca_certs) where ca_certs is a dict keyed by
    rfc4514 subject string for chain building.
    """
    ca_certs: dict[str, MockObject] = {}

    # Physical reader + token
    reader = MockReader(name="Mock Smart Card Reader 0", card_present=True)
    token = MockToken(
        reader_name=reader.name,
        serial="1234567890123456",
        full_serial="00001234567890123456",
        model="MockCard v1.0",
        manufacturer="Mock Inc.",
        label="Mock Agent Card",
        pins=[
            PinInfo(label="PIN Global", expected_pin="1234"),
            PinInfo(label="PIN Signature", expected_pin="123456"),
        ],
    )

    if pki_dir is not None:
        token.objects, ca_certs = _load_pki_dir(pki_dir)
    else:
        # Authentication certificate (vslot 0 → PIN Global)
        auth_cert, auth_pub, auth_priv = _generate_cert_and_key(
            cn="Alice Mock",
            issuer_cn="Mock CA Authentication",
            usages=[
                {"shortName": "clientAuth", "longName": "TLS Web Client Authentication", "oid": "1.3.6.1.5.5.7.3.2"},
            ],
            vslot=0,
        )

        # Signature certificate (vslot 1 → PIN Signature)
        sig_cert, sig_pub, sig_priv = _generate_cert_and_key(
            cn="Alice Mock",
            issuer_cn="Mock CA Signature",
            usages=[{"shortName": "timeStamping"}],
            vslot=1,
        )

        # Confidentiality / encryption certificate (vslot 0)
        enc_cert, enc_pub, enc_priv = _generate_cert_and_key(
            cn="Alice Mock",
            issuer_cn="Mock CA Confidentiality",
            usages=[{"shortName": "ipsecUser"}, {"shortName": "encrypt"}],
            vslot=0,
        )

        token.objects = [auth_cert, auth_pub, auth_priv, sig_cert, sig_pub, sig_priv, enc_cert, enc_pub, enc_priv]

    return [reader], [token], ca_certs


# ---------------------------------------------------------------------------
# Flask application
# ---------------------------------------------------------------------------

app = Flask(__name__)

# Global state -  single environment for simplicity.
_challenge_store: dict[str, str] = {}  # challenge-hex -> timestamp
_environments: dict[str, Environment] = {}
_pki_dir: Path | None = None  # Set via --pki-dir CLI argument


def _get_env() -> Environment | None:
    env_id = request.args.get("env")
    if env_id and env_id in _environments:
        return _environments[env_id]
    return None


def _cors(resp: Response) -> Response:
    origin = request.headers.get("Origin", "*")
    resp.headers["Access-Control-Allow-Origin"] = origin
    resp.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    resp.headers["Access-Control-Allow-Headers"] = "Content-Type"
    resp.headers["Vary"] = "Origin"
    return resp


@app.after_request
def after_request(resp: Response) -> Response:
    return _cors(resp)


# ---- Service discovery & authentication ----

@app.route("/dyn/get_challenge", methods=["GET"])
def get_challenge():
    """Mutual authentication step 1: validate the webapp certificate and return
    a challenge + cryptogram."""
    token_param = request.args.get("token", "")
    challenge_in = request.args.get("challenge", "")

    # The real service rejects non-certificate tokens with 500.
    # Heuristic: if the token doesn't look like a base-64 certificate, reject.
    if not token_param.startswith("MIIF") and not token_param.startswith("MIIG") and not token_param.startswith("MIIH") and not token_param.startswith("MIIE") and not token_param.startswith("MIID"):
        return Response('{"errString":"CKR_GENERAL_ERROR"}', status=500, content_type="application/json")

    new_challenge = secrets.token_hex(16).upper()
    # Store it so create_env can reference it
    _challenge_store[new_challenge] = str(time.time())
    # Generate a fake cryptogram (in real life this is the SCWS private-key signature of the input challenge)
    cryptogram = secrets.token_hex(256).upper()

    return jsonify({"challenge": new_challenge, "cryptogram": cryptogram, "keyID": "0"})


@app.route("/dyn/create_env", methods=["GET"])
def create_env():
    """Create an environment after mutual authentication."""
    # sig = request.args.get("sig", "")  # We don't verify in the mock
    encalg = request.args.get("encalg", "SHA-256")

    readers, tokens, ca_certs = build_default_state(pki_dir=_pki_dir)
    env = Environment(readers=readers, ca_certs=ca_certs)

    # Register CA cert objects so they can be accessed by handle (for export, etc.)
    for ca_obj in ca_certs.values():
        env.all_objects[ca_obj.handle] = ca_obj

    # Register tokens
    for t in tokens:
        env.tokens[t.handle] = t
        for obj in t.objects:
            env.all_objects[obj.handle] = obj

    _environments[env.env_id] = env

    pub_pem = env.rsa_key.public_key().public_bytes(
        serialization.Encoding.PEM, serialization.PublicFormat.SubjectPublicKeyInfo
    ).decode()

    return jsonify({
        "envId": env.env_id,
        "pubKeyPem": pub_pem,
        "encalg": f"RSA-OAEP/{encalg}",
        "credentialEncAlg": "AES128-CBC",
    })


# ---- Credential encryption key (AES) ----

@app.route("/dyn/set_credential_encryption_key", methods=["POST"])
def set_credential_encryption_key():
    env_id = request.args.get("envId")
    env = _environments.get(env_id) if env_id else None
    if env is None:
        return Response("env not found", status=400)
    # In real SCWS, the body is the RSA-encrypted AES key. We just store it.
    env.credential_enc_key = request.get_data()
    return jsonify(True)


# ---- Readers & events ----

@app.route("/dyn/get_readers", methods=["GET"])
def get_readers():
    env = _get_env()
    if env is None:
        return jsonify([])
    return jsonify([r.to_dict() for r in env.readers])


@app.route("/dyn/get_vscaccounts", methods=["GET"])
def get_vscaccounts():
    return jsonify({"vscAccounts": []})


@app.route("/dyn/get_event", methods=["GET"])
def get_event():
    """Long-poll for events. In mock mode we just return a reader event after a
    short delay, then block subsequent calls (return 204 / hang)."""
    env = _get_env()
    if env is None:
        return Response("", status=204)
    if env.readers:
        # Return one reader event, then nothing
        reader_data = env.readers[0].to_dict()
        return jsonify({"reader": reader_data})
    return Response("", status=204)


# ---- Token connection ----

@app.route("/dyn/connect", methods=["GET"])
def connect():
    env = _get_env()
    if env is None:
        return Response("env not found", status=400)

    reader_name = request.args.get("reader") or request.args.get("vscReader", "")

    # SOFTREADER handling
    if reader_name == "\\SOFTREADER":
        soft = MockToken(
            reader_name="\\SOFTREADER",
            serial="\x00" * 16,
            full_serial="",
            model="SoftToken",
            manufacturer="Mock Inc.",
            label="SoftToken",
            pins=[PinInfo(label="SoftToken PIN", logged_in=True)],
        )
        env.tokens[soft.handle] = soft
        return jsonify({
            "handle": soft.handle,
            "serial": soft.serial,
            "fullSerial": soft.full_serial,
            "model": soft.model,
        })

    # Find the matching physical token
    for t in env.tokens.values():
        if t.reader_name == reader_name:
            return jsonify({
                "handle": t.handle,
                "serial": t.serial,
                "fullSerial": t.full_serial,
                "model": t.model,
            })
    return Response('{"errString":"CKR_TOKEN_NOT_PRESENT"}', status=500, content_type="application/json")


@app.route("/dyn/disconnect", methods=["GET"])
def disconnect():
    return jsonify(True)


# ---- Token info & objects ----

@app.route("/dyn/get_token_info", methods=["GET"])
def get_token_info():
    env = _get_env()
    token_handle = request.args.get("token", "")
    if env is None or token_handle not in env.tokens:
        return Response("token not found", status=400)
    token = env.tokens[token_handle]
    return jsonify({"infos": token.info_list(), "sbrMode": False})


@app.route("/dyn/get_token_objects", methods=["GET"])
def get_token_objects():
    env = _get_env()
    token_handle = request.args.get("token", "")
    if env is None or token_handle not in env.tokens:
        return Response("token not found", status=400)
    token = env.tokens[token_handle]
    return jsonify({"objects": [o.to_summary() for o in token.objects]})


@app.route("/dyn/get_object_data", methods=["GET"])
def get_object_data():
    env = _get_env()
    obj_handle = request.args.get("object", "")
    if env is None or obj_handle not in env.all_objects:
        return Response("object not found", status=400)
    obj = env.all_objects[obj_handle]
    return jsonify(obj.to_detail())


@app.route("/dyn/export_object", methods=["GET"])
def export_object():
    env = _get_env()
    obj_handle = request.args.get("object", "")
    fmt = request.args.get("format", "PEM")
    if env is None or obj_handle not in env.all_objects:
        return Response("object not found", status=400)
    obj = env.all_objects[obj_handle]
    if obj._pem_cert is None:
        return Response("no certificate data", status=400)
    # The real SCWS returns base64 of the PEM
    encoded = base64.b64encode(obj._pem_cert).decode()
    return Response(encoded, content_type="text/plain")


@app.route("/dyn/get_object_value", methods=["GET"])
def get_object_value():
    """Return hex-encoded value for data container objects."""
    env = _get_env()
    obj_handle = request.args.get("object", "")
    if env is None or obj_handle not in env.all_objects:
        return Response("object not found", status=400)
    # Return empty hex string for mock data containers
    return Response("", content_type="text/plain")


# ---- PIN / Login ----

@app.route("/dyn/login", methods=["POST"])
def login():
    env = _get_env()
    token_handle = request.args.get("token", "")
    pin_index = int(request.args.get("pin", "0"))
    encrypted = request.args.get("e", "false")
    if env is None or token_handle not in env.tokens:
        return Response("token not found", status=400)
    token = env.tokens[token_handle]
    if pin_index >= len(token.pins):
        return Response('{"errString":"CKR_PIN_INCORRECT"}', status=500, content_type="application/json")

    pin_info = token.pins[pin_index]
    body = request.get_data(as_text=True).strip()

    # Accept login without verification when body is "#" (PIN entered on device)
    # or when encrypted=true (we can't decrypt it in mock mode)
    if body == "#" or encrypted == "true":
        pin_info.logged_in = True
        return jsonify(True)

    # Plain-text PIN
    pin_value = body.rstrip("\n")
    if pin_value == pin_info.expected_pin:
        pin_info.logged_in = True
        return jsonify(True)

    return Response('{"errString":"CKR_PIN_INCORRECT"}', status=500, content_type="application/json")


@app.route("/dyn/logout", methods=["GET"])
def logout():
    env = _get_env()
    token_handle = request.args.get("token", "")
    pin_index = int(request.args.get("pin", "0"))
    if env is None or token_handle not in env.tokens:
        return Response("token not found", status=400)
    token = env.tokens[token_handle]
    if pin_index < len(token.pins):
        token.pins[pin_index].logged_in = False
    return jsonify(True)


@app.route("/dyn/start_auto_login", methods=["GET"])
def start_auto_login():
    return jsonify(True)


@app.route("/dyn/stop_auto_login", methods=["GET"])
def stop_auto_login():
    return jsonify(True)


# ---- Crypto operations ----

def _find_key(env: Environment, key_handle: str) -> rsa.RSAPrivateKey | None:
    obj = env.all_objects.get(key_handle)
    if obj and obj._private_key:
        return obj._private_key
    # Also search by iterating all objects (the handle might be for a private key)
    for o in env.all_objects.values():
        if o.handle == key_handle and o._private_key:
            return o._private_key
    return None


@app.route("/dyn/sign", methods=["POST"])
def sign():
    env = _get_env()
    key_handle = request.args.get("key", "")
    if env is None:
        return Response("env not found", status=400)

    private_key = _find_key(env, key_handle)
    if private_key is None:
        return Response('{"errString":"CKR_KEY_HANDLE_INVALID"}', status=500, content_type="application/json")

    # Extract hash from multipart form or raw body
    hash_hex = None
    if request.content_type and "multipart" in request.content_type:
        hash_hex = request.form.get("hash")
    if not hash_hex:
        hash_hex = request.get_data(as_text=True).strip()

    if not hash_hex:
        return Response('{"errString":"MWR_SCWS_BAD_PARAMS"}', status=400, content_type="application/json")

    hash_bytes = bytes.fromhex(hash_hex)

    mgf_alg = request.args.get("mgfAlg")
    salt_len = request.args.get("saltLen")
    hash_algorithm_param = request.args.get("hashAlgorithm")

    if mgf_alg and salt_len is not None:
        # PSS padding
        hash_algo_map = {"sha1": hashes.SHA1(), "sha256": hashes.SHA256(), "sha384": hashes.SHA384(), "sha512": hashes.SHA512()}
        mgf_hash = hash_algo_map.get(mgf_alg, hashes.SHA256())
        sig = private_key.sign(
            hash_bytes,
            padding.PSS(
                mgf=padding.MGF1(mgf_hash),
                salt_length=int(salt_len),
            ),
            Prehashed(hashes.SHA256()),
        )
    else:
        # PKCS1v15 padding
        sig = private_key.sign(
            hash_bytes,
            padding.PKCS1v15(),
            Prehashed(hashes.SHA256()),
        )

    return Response(sig.hex().upper(), content_type="text/plain")


@app.route("/dyn/decrypt", methods=["POST"])
def decrypt():
    env = _get_env()
    key_handle = request.args.get("key", "")
    if env is None:
        return Response("env not found", status=400)

    private_key = _find_key(env, key_handle)
    if private_key is None:
        return Response('{"errString":"CKR_KEY_HANDLE_INVALID"}', status=500, content_type="application/json")

    # Extract ciphertext from multipart form or raw body
    data_hex = None
    if request.content_type and "multipart" in request.content_type:
        data_hex = request.form.get("data")
    if not data_hex:
        data_hex = request.get_data(as_text=True).strip()

    if not data_hex:
        return Response('{"errString":"MWR_SCWS_BAD_PARAMS"}', status=400, content_type="application/json")

    ciphertext = bytes.fromhex(data_hex)

    alg = request.args.get("alg", "")
    hash_alg = request.args.get("hashAlg", "sha256")
    mgf = request.args.get("mgf", "sha256")

    hash_algo_map = {"sha1": hashes.SHA1(), "sha256": hashes.SHA256(), "sha384": hashes.SHA384(), "sha512": hashes.SHA512()}

    try:
        if alg == "oaep":
            plaintext = private_key.decrypt(
                ciphertext,
                padding.OAEP(
                    mgf=padding.MGF1(hash_algo_map.get(mgf, hashes.SHA256())),
                    algorithm=hash_algo_map.get(hash_alg, hashes.SHA256()),
                    label=None,
                ),
            )
        else:
            # Default: PKCS1v15
            plaintext = private_key.decrypt(ciphertext, padding.PKCS1v15())
    except Exception as e:
        return Response(f'{{"errString":"CKR_ENCRYPTED_DATA_INVALID","message":"{e}"}}', status=500, content_type="application/json")

    return Response(plaintext.hex(), content_type="text/plain")


# ---- Product info ----

@app.route("/dyn/get_product_info", methods=["GET"])
def get_product_info():
    return jsonify({
        "company": "Mock SCWS",
        "product": "Mock SmartCard Web Service",
        "version": "1.0.0.0",
    })


# ---- Certificate trust & chain building ----

def _build_cert_chain(env: Environment, leaf_obj: MockObject) -> list[dict]:
    """Walk up the issuer chain using loaded CA certs.

    Returns a list of certificate summaries: index 0 is the leaf (end-entity),
    last is the root.  The scwsapi.js client replaces index 0 with the
    original Certificate object on its side, so we still include it here.
    """
    chain = [leaf_obj.to_summary()]
    seen: set[str] = set()
    current = leaf_obj

    while current._certificate is not None:
        issuer_rfc4514 = current._certificate.issuer.rfc4514_string()
        subject_rfc4514 = current._certificate.subject.rfc4514_string()

        # Self-signed → we've reached a root, stop.
        if issuer_rfc4514 == subject_rfc4514:
            break

        # Avoid loops
        if issuer_rfc4514 in seen:
            break
        seen.add(issuer_rfc4514)

        ca_obj = env.ca_certs.get(issuer_rfc4514)
        if ca_obj is None:
            break  # Issuer not available — partial chain

        chain.append(ca_obj.to_summary())
        current = ca_obj

    return chain


@app.route("/dyn/get_certificate_trust", methods=["GET"])
def get_certificate_trust():
    env = _get_env()
    obj_handle = request.args.get("object", "")
    if env is None or obj_handle not in env.all_objects:
        return Response("object not found", status=400)

    obj = env.all_objects[obj_handle]
    if obj.obj_type != "certificate" or obj._certificate is None:
        return jsonify({"trustStatus": "ok", "usages": obj.usages, "certPath": []})

    cert_path = _build_cert_chain(env, obj)
    return jsonify({
        "trustStatus": "ok",
        "usages": obj.usages,
        "certPath": cert_path,
    })


# ---- Cert stores (stub) ----

@app.route("/dyn/get_certstores", methods=["GET"])
def get_certstores():
    return jsonify({"objects": []})


# ---- Object deletion (stub) ----

@app.route("/dyn/delete_objects", methods=["GET"])
def delete_objects():
    return jsonify(True)


# ---- CORS preflight ----

@app.route("/dyn/<path:path>", methods=["OPTIONS"])
def options_handler(path):
    return Response("", status=204)


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def main():
    global _pki_dir

    parser = argparse.ArgumentParser(description="Mock SCWS service")
    parser.add_argument("--port", type=int, default=41231, help="Port to listen on (default: 41231)")
    parser.add_argument("--host", default="127.0.0.1", help="Host to bind to (default: 127.0.0.1)")
    parser.add_argument(
        "--pki-dir",
        type=Path,
        default=None,
        help="Path to a PKI directory containing Cert/*.crt + Cert/*.key pairs. "
             "If omitted, auto-generated certificates are used.",
    )
    args = parser.parse_args()

    if args.pki_dir is not None:
        _pki_dir = args.pki_dir.resolve()
        if not _pki_dir.is_dir():
            parser.error(f"PKI directory does not exist: {_pki_dir}")
        print(f"Using PKI directory: {_pki_dir}")
    else:
        print("No --pki-dir specified, using auto-generated certificates")

    print(f"Mock SCWS service starting on http://{args.host}:{args.port}")
    print("Endpoints available under /dyn/...")
    print()
    print("Token: Mock Agent Card")
    print("  PIN Global:    1234")
    print("  PIN Signature: 123456")
    print()

    app.run(host=args.host, port=args.port, debug=True)


if __name__ == "__main__":
    main()
