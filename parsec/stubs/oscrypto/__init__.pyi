from __future__ import annotations

def use_openssl(
    libcrypto_path: str,
    libssl_path: str,
    trust_list_path: str | None = None,
) -> None: ...
