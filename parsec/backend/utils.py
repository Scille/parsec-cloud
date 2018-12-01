def anonymous_api(fn):
    fn._anonymous_api_allowed = True
    return fn


def check_anonymous_api_allowed(fn):
    if not getattr(fn, "_anonymous_api_allowed", False):
        raise RuntimeError(
            f"Trying to add {fn!r} to the anonymous api "
            "(need to use @anonymous_api decorator for this)."
        )
