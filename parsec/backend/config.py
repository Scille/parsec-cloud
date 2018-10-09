from os import environ
import attr


def config_factory(raw_conf):
    raw_conf = {k.upper(): v for k, v in raw_conf.items() if v not in (None, "")}

    for mandatory in ("BLOCKSTORE_TYPE", "DB_URL"):
        if not raw_conf.get(mandatory):
            raise ValueError(f"Missing mandatory config {mandatory}")

    if "SENTRY_URL" not in raw_conf:
        raw_conf["SENTRY_URL"] = environ.get("SENTRY_URL") or None

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
        elif val == "S3":
            expected_envs = ("S3_REGION", "S3_BUCKET", "S3_KEY", "S3_SECRET")
            if any(env for env in expected_envs if env not in environ):
                raise ValueError(
                    f"S3 blockstore requires environment variables: {', '.join(expected_envs)}"
                )
            return val
        elif val == "SWIFT":
            expected_envs = (
                "SWIFT_AUTH_URL" "SWIFT_TENANT" "SWIFT_CONTAINER" "SWIFT_USER" "SWIFT_PASSWORD"
            )
            if any(env for env in expected_envs if env not in environ):
                raise ValueError(
                    f"SWIFT blockstore requires environment variables: {', '.join(expected_envs)}"
                )
            return val

        raise ValueError(
            "BLOCKSTORE_TYPE must be `MOCKED`, `POSTGRESQL`, `S3`, or `SWIFT` and environment variables must be set accordingly"
        )

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
