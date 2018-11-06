try:
    from parsec.core.gui._resources_rc import *  # noqa
except ImportError as exc:
    raise ModuleNotFoundError(
        """PyQt resources bundle hasn't been generated.
You must install the parsec package or run `python setup.py generate_pyqt_resources_bundle`
"""
    ) from exc
