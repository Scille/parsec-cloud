

class _CoreCall:
    def __init__(self, parsec_core, trio_portal, cancel_scope):
        self._parsec_core = parsec_core
        self._trio_portal = trio_portal
        self._cancel_scope = cancel_scope

    def cancel(self):
        self._trio_portal.run_sync(self._cancel_scope.cancel)

    def run_stat(self, *args, **kwargs):
        return self._trio_portal.run(self._parsec_core.fs.stat, *args, **kwargs)

    def run_create_folder(self, *args, **kwargs):
        return self._trio_portal.run(self._parsec_core.fs.folder_create, *args, **kwargs)

    def run_delete_folder(self, *args, **kargs):
        return self._trio_portal.run(self._parsec_core.fs.delete, *args, **kargs)


_CORE_CALL = None


def core_call():
    return _CORE_CALL

def init_core_call(parsec_core, trio_portal, cancel_scope):
    global _CORE_CALL

    _CORE_CALL = _CoreCall(parsec_core, trio_portal, cancel_scope)
