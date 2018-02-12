import os
import attr
import pytest
from hypothesis import strategies as st, note
from hypothesis.stateful import Bundle, rule, invariant

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import connect_core, core_factory


@attr.s
class OracleFS:
    root = attr.ib(default=attr.Factory(dict))

    def create_file(self, parent_path, name):
        import pdb; pdb.set_trace()
        parent_folder = self.get_folder(parent_path)
        if parent_folder is None or name in parent_folder:
            return None
        parent_folder[name] = '<file>'
        return 

    def create_folder(self, parent_path, name):
        parent_folder = self.get_folder(parent_path)
        if parent_folder is None or name in parent_folder:
            return None
        parent_folder[name] = {}

    def get_folder(self, path):
        elem = self.get_path(path)
        return elem if elem is not '<file>' else None
        
    def get_file(self, path):
        elem = self.get_path(path)
        return elem if elem is '<file>' else None
        
    def get_path(self, path):
        current_folder = self.root
        try:
            for item in path.split('/'):
                if item:
                    current_folder = current_folder[item]
        except KeyError:
            return None
        return current_folder


@attr.s
class File:
    path = attr.ib()


@attr.s
class Folder:
    path = attr.ib()

    def file_child(self, name):
        return File(os.path.join(self.path, name))

    def folder_child(self, name):
        return Folder(os.path.join(self.path, name))


@pytest.mark.slow
@pytest.mark.trio
async def test_offline_core_tree(TrioDriverRuleBasedStateMachine, backend_addr, tmpdir, alice, monitor):

    class CoreOfflineRWFile(TrioDriverRuleBasedStateMachine):
        Files = Bundle('file')
        Folders = Bundle('folder')
        count = 0

        async def trio_runner(self, task_status):
            self.oracle_fs = OracleFS()

            type(self).count += 1
            config = {
                'base_settings_path': tmpdir.mkdir('try-%s' % self.count).strpath,
                'backend_addr': backend_addr,
            }
            # Hack...
            alice.local_storage_db_path = ':memory:'

            async with core_factory(**config) as core:
                await core.login(alice)
                async with connect_core(core) as sock:

                    with self.open_communicator() as core_communicator:
                        self.core_cmd = core_communicator.send
                        task_status.started()

                        while True:
                            msg = await core_communicator.trio_recv()
                            await sock.send(msg)
                            rep = await sock.recv()
                            await core_communicator.trio_respond(rep)

        @rule(target=Folders)
        def init_root(self):
            return '/'

        @rule(target=Files, parent=Folders, name=st.text(min_size=1))
        def create_file(self, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd({'cmd': 'file_create', 'path': path})
            note(rep)
            if self.oracle_fs.create_file(parent, name) is None:
                assert rep['status'] != 'ok'
            else:
                assert rep == {'status': 'ok'}
            return path

        @rule(target=Folders, parent=Folders, name=st.text(min_size=1))
        def create_folder(self, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd({'cmd': 'file_folder', 'path': path})
            note(rep)
            if self.oracle_fs.create_file(parent, name) is None:
                assert rep['status'] != 'ok'
            else:
                assert rep == {'status': 'ok'}
            return path

        def delete_entry():
            pass

        def move_entry():
            pass

        def copy_entry():
            pass

    await CoreOfflineRWFile.run_test()
