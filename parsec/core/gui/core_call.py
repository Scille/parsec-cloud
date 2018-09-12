from parsec.core.devices_manager import invite_user, claim_user


class _CoreCall:
    def __init__(self, parsec_core, trio_portal, cancel_scope):
        self._parsec_core = parsec_core
        self._trio_portal = trio_portal
        self._cancel_scope = cancel_scope

    def stop(self):
        self._trio_portal.run_sync(self._cancel_scope.cancel)

    def stat(self, *args, **kwargs):
        return self._trio_portal.run(self._parsec_core.fs.stat, *args, **kwargs)

    def create_folder(self, *args, **kwargs):
        return self._trio_portal.run(self._parsec_core.fs.folder_create, *args, **kwargs)

    def delete_folder(self, *args, **kargs):
        return self._trio_portal.run(self._parsec_core.fs.delete, *args, **kargs)

    def delete_file(self, *args, **kargs):
        return self._trio_portal.run(self._parsec_core.fs.delete, *args, **kargs)

    def mount(self, *args, **kwargs):
        self._trio_portal.run(
            self._parsec_core.fuse_manager.start_mountpoint,
            *args, **kwargs)

    def unmount(self, *args, **kwargs):
        self._trio_portal.run(
            self._parsec_core.fuse_manager.stop_mountpoint,
            *args, **kwargs)

    def is_mounted(self, *args, **kwargs):
        if self._parsec_core.fuse_manager:
            return self._parsec_core.fuse_manager.is_started(*args, **kwargs)
        return False

    def login(self, *args, **kwargs):
        self._trio_portal.run(self._parsec_core.login, *args, **kwargs)

    def logout(self, *args, **kwargs):
        self._trio_portal.run(self._parsec_core.logout, *args, **kwargs)

    def get_devices(self, *args, **kwargs):
        return self._parsec_core.local_devices_manager.list_available_devices()

    def register_new_device(self, *args):
        return self._parsec_core.local_devices_manager.register_new_device(
            *args)

    def load_device(self, *args):
        return self._parsec_core.local_devices_manager.load_device(*args)

    def invite_user(self, *args):
        return self._trio_portal.run(invite_user, self._parsec_core.backend_cmds_sender,
                                     *args)

    def claim_user(self, *args):
        return self._trio_portal.run(claim_user, self._parsec_core.backend_addr,
                                     *args)

    def create_workspace(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.workspace_create, *args)


_CORE_CALL = None


def core_call():
    return _CORE_CALL


def init_core_call(parsec_core, trio_portal, cancel_scope):
    global _CORE_CALL

    _CORE_CALL = _CoreCall(parsec_core, trio_portal, cancel_scope)
