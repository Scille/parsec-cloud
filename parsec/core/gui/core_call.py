from parsec.core.app import NotLoggedError
from parsec.core.devices_manager import (
    invite_user,
    claim_user,
    declare_device,
    accept_device_configuration_try,
    configure_new_device,
)


class _CoreCall:
    def __init__(self, parsec_core, trio_portal, cancel_scope):
        self._parsec_core = parsec_core
        self._trio_portal = trio_portal
        self._cancel_scope = cancel_scope

    def is_debug(self):
        return self._parsec_core.config.debug

    def get_entry_path(self, id):
        return self._trio_portal.run(self._parsec_core.fs.get_entry_path, id)

    def connect_signal(self, signal, cb):
        self._parsec_core.signal_ns.signal(signal).connect(cb, weak=True)

    def stop(self):
        self._trio_portal.run_sync(self._cancel_scope.cancel)

    def stat(self, *args, **kwargs):
        return self._trio_portal.run(self._parsec_core.fs.stat, *args, **kwargs)

    def file_create(self, *args, **kwargs):
        return self._trio_portal.run(self._parsec_core.fs.file_create, *args, **kwargs)

    def file_open(self, *args, **kwargs):
        return self._trio_portal.run(self._parsec_core.fs.file_fd_open, *args, **kwargs)

    def file_close(self, *args, **kwargs):
        return self._trio_portal.run(self._parsec_core.fs.file_fd_close, *args, **kwargs)

    def file_write(self, *args, **kwargs):
        return self._trio_portal.run(self._parsec_core.fs.file_fd_write, *args, **kwargs)

    def create_folder(self, *args, **kwargs):
        return self._trio_portal.run(self._parsec_core.fs.folder_create, *args, **kwargs)

    def delete_folder(self, *args, **kargs):
        return self._trio_portal.run(self._parsec_core.fs.delete, *args, **kargs)

    def delete_file(self, *args, **kargs):
        return self._trio_portal.run(self._parsec_core.fs.delete, *args, **kargs)

    def mount(self, *args, **kwargs):
        self._trio_portal.run(self._parsec_core.fuse_manager.start_mountpoint, *args, **kwargs)

    def unmount(self, *args, **kwargs):
        self._trio_portal.run(self._parsec_core.fuse_manager.stop_mountpoint, *args, **kwargs)

    def is_mounted(self, *args, **kwargs):
        if self._parsec_core.fuse_manager:
            return self._parsec_core.fuse_manager.is_started(*args, **kwargs)
        return False

    def login(self, *args, **kwargs):
        self._trio_portal.run(self._parsec_core.login, *args, **kwargs)

    def logout(self, *args, **kwargs):
        try:
            self._trio_portal.run(self._parsec_core.logout, *args, **kwargs)
        except NotLoggedError:
            pass

    def get_devices(self, *args, **kwargs):
        return self._parsec_core.local_devices_manager.list_available_devices()

    def register_new_device(self, *args):
        """
        Locally register the new device.
        """
        return self._parsec_core.local_devices_manager.register_new_device(*args)

    def load_device(self, *args):
        """
        Load device from local.
        """
        return self._parsec_core.local_devices_manager.load_device(*args)

    def invite_user(self, *args):
        return self._trio_portal.run(invite_user, self._parsec_core.backend_cmds_sender, *args)

    def declare_device(self, device_name):
        return self._trio_portal.run(
            declare_device, self._parsec_core.backend_cmds_sender, device_name
        )

    def accept_device_configuration_try(self, config_try_id, password):
        return self._trio_portal.run(
            accept_device_configuration_try,
            self._parsec_core.backend_cmds_sender,
            self._parsec_core.auth_device,
            config_try_id,
            password,
        )

    def configure_device(self, device_id, password, configure_device_token):
        user_privkey, device_signkey, user_manifest_access = self._trio_portal.run(
            configure_new_device,
            self._parsec_core.backend_addr,
            device_id,
            configure_device_token,
            password,
        )

        self._parsec_core.local_devices_manager.register_new_device(
            device_id,
            user_privkey.encode(),
            device_signkey.encode(),
            user_manifest_access,
            password,
        )

    def claim_user(self, *args):
        return self._trio_portal.run(claim_user, self._parsec_core.backend_addr, *args)

    def create_workspace(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.workspace_create, *args)

    def share_workspace(self, *args, **kwargs):
        return self._trio_portal.run(self._parsec_core.fs.share, *args, **kwargs)


_CORE_CALL = None


def core_call():
    return _CORE_CALL


def init_core_call(parsec_core, trio_portal, cancel_scope):
    global _CORE_CALL

    _CORE_CALL = _CoreCall(parsec_core, trio_portal, cancel_scope)
