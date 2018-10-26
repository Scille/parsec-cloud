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

    def connect_event(self, event, cb):
        self._parsec_core.event_bus.connect(event, cb, weak=True)

    def stop(self):
        self._trio_portal.run_sync(self._cancel_scope.cancel)

    def stat(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.stat, *args)

    def file_create(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.file_create, *args)

    def file_open(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.file_fd_open, *args)

    def file_close(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.file_fd_close, *args)

    def file_write(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.file_fd_write, *args)

    def file_read(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.file_fd_read, *args)

    def create_folder(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.folder_create, *args)

    def delete_folder(self, *args, **kargs):
        return self._trio_portal.run(self._parsec_core.fs.delete, *args, **kargs)

    def delete_file(self, *args, **kargs):
        return self._trio_portal.run(self._parsec_core.fs.delete, *args, **kargs)

    def move_folder(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.move, *args)

    def move_file(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.move, *args)

    def move_workspace(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.move, *args)

    def mount(self, mountpoint):
        self._trio_portal.run(self._parsec_core.mountpoint_manager.start, mountpoint)

    def unmount(self, *args):
        self._trio_portal.run(self._parsec_core.mountpoint_manager.stop)

    def is_mounted(self, *args):
        try:
            return self._parsec_core.mountpoint_manager.is_started(*args)
        except NotLoggedError:
            return False

    def login(self, *args):
        self._trio_portal.run(self._parsec_core.login, *args)

    def logout(self, *args):
        try:
            self._trio_portal.run(self._parsec_core.logout, *args)
        except NotLoggedError:
            pass

    def get_devices(self, *args):
        return self._parsec_core.local_devices_manager.list_available_devices()

    def register_new_device(self, *args):
        """
        Locally register the new device.
        """
        return self._parsec_core.local_devices_manager.register_new_device(*args)

    def load_device(self, *args, **kwargs):
        """
        Load device from local.
        """
        return self._parsec_core.local_devices_manager.load_device(*args, **kwargs)

    def invite_user(self, *args):
        return self._trio_portal.run(invite_user, self._parsec_core.backend_cmds_sender, *args)

    def declare_device(self, device_name):
        return self._trio_portal.run(
            declare_device, self._parsec_core.backend_cmds_sender, device_name
        )

    def accept_device_configuration_try(
        self,
        config_try_id,
        password=None,
        nitrokey_pin=None,
        nitrokey_token_id=0,
        nitrokey_key_id=0,
    ):
        return self._trio_portal.run(
            accept_device_configuration_try,
            self._parsec_core.backend_cmds_sender,
            self._parsec_core.auth_device,
            config_try_id,
            password,
            nitrokey_pin,
            nitrokey_token_id,
            nitrokey_key_id,
        )

    def configure_new_device(
        self,
        device_id,
        configure_device_token,
        use_nitrokey=False,
        password=None,
        nitrokey_token_id=None,
        nitrokey_key_id=None,
    ):
        return self._trio_portal.run(
            configure_new_device,
            self._parsec_core.backend_addr,
            device_id,
            configure_device_token,
            password,
            use_nitrokey,
            nitrokey_token_id,
            nitrokey_key_id,
        )

    def claim_user(self, *args):
        return self._trio_portal.run(claim_user, self._parsec_core.backend_addr, *args)

    def create_workspace(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.workspace_create, *args)

    def share_workspace(self, *args):
        return self._trio_portal.run(self._parsec_core.fs.share, *args)


_CORE_CALL = None


def core_call():
    return _CORE_CALL


def init_core_call(parsec_core, trio_portal, cancel_scope):
    global _CORE_CALL

    _CORE_CALL = _CoreCall(parsec_core, trio_portal, cancel_scope)
