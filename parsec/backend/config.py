from os import environ
import attr


blockstore_params = {
    "S3": ["REGION", "BUCKET", "KEY", "SECRET"],
    "SWIFT": ["AUTHURL", "TENANT", "CONTAINER", "USER", "PASSWORD"],
}


def config_factory(raw_conf):
    raw_conf = {k.upper(): v for k, v in raw_conf.items() if v not in (None, "")}

    for mandatory in ("BLOCKSTORE_TYPE", "DB_URL"):
        if not raw_conf.get(mandatory):
            raise ValueError(f"Missing mandatory config {mandatory}")

    if "SENTRY_URL" not in raw_conf:
        raw_conf["SENTRY_URL"] = environ.get("SENTRY_URL") or None

    for blockstore_type, params in blockstore_params.items():
        for param in params:
            key = f"{blockstore_type}_{param}"
            raw_conf[key] = environ.get(key)

    return BackendConfig(**{k.lower(): v for k, v in raw_conf.items()})


@attr.s(slots=True, frozen=True)
class BackendConfig:

    blockstore_type = attr.ib()

    @blockstore_type.validator
    def _validate_blockstore_type(self, field, val):
        if val == "MOCKED":
            return val
        elif val == "POSTGRESQL":
            return val
        elif val in ["S3", "SWIFT"]:
            if all(
                [
                    getattr(self, f"{val.lower()}_{param.lower()}")
                    for param in blockstore_params[val]
                ]
            ):
                return val
            else:
                missings = [
                    param
                    for param in blockstore_params[val]
                    if not getattr(self, f"{val.lower()}_{param.lower()}")
                ]
                plural = "s" if len(missings) > 1 else ""
                raise ValueError(
                    ", ".join(missings) + f" environment variable{plural} must be set accordingly"
                )
        raise ValueError("BLOCKSTORE_TYPE must be `MOCKED`, `POSTGRESQL`, `S3`, or `SWIFT`")

    db_url = attr.ib()

    @db_url.validator
    def _validate_db_url(self, field, val):
        if val == "MOCKED":
            return val
        elif val.startswith("postgresql://"):
            return val
        raise ValueError("DB_URL must be `MOCKED` or `postgresql://...`")

    sentry_url = attr.ib(default=None)

    debug = attr.ib(default=False)
    handshake_challenge_size = attr.ib(default=48)

    s3_region = attr.ib(default=None)
    s3_bucket = attr.ib(default=None)
    s3_key = attr.ib(default=None)
    s3_secret = attr.ib(default=None)

    swift_authurl = attr.ib(default=None)
    swift_tenant = attr.ib(default=None)
    swift_container = attr.ib(default=None)
    swift_user = attr.ib(default=None)
    swift_password = attr.ib(default=None)
