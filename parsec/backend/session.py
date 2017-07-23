import attr


@attr.s
class EGetAuthenticatedUser:
	pass


@attr.s
class Session:
	id = attr.ib()
