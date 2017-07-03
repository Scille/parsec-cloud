import attr


@attr.s
class BackendCmd:
    cmd = attr.ib()
    msg = attr.ib(default=dict)
