from os import environ
import attr


blockstore_environ_vars = {
    "S3": ["S3_REGION", "S3_BUCKET", "S3_KEY", "S3_SECRET"],
    "SWIFT": ["SWIFT_AUTHURL", "SWIFT_TENANT", "SWIFT_CONTAINER", "SWIFT_USER", "SWIFT_PASSWORD"],
}


def config_factory(raw_conf):
    raw_conf = {k.upper(): v for k, v in raw_conf.items() if v not in (None, "")}

    for mandatory in ("BLOCKSTORE_TYPE", "DB_URL"):
        if not raw_conf.get(mandatory):
            raise ValueError(f"Missing mandatory config {mandatory}")

    if "SENTRY_URL" not in raw_conf:
        raw_conf["SENTRY_URL"] = environ.get("SENTRY_URL") or None

    for blockstore_type, environ_vars in blockstore_environ_vars.items():
        for environ_var in environ_vars:
            raw_conf[environ_var] = environ.get(environ_var)

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
            for environ_var in blockstore_environ_vars[val]:
                if not getattr(self, environ_var.lower()):
                    raise ValueError(
                        f"Blockstore {val} requires environment variables: {', '.join(blockstore_environ_vars[val])}"
                    )
            else:
                return val

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
