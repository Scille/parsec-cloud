# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import pytest
from hypothesis.stateful import RuleBasedStateMachine, initialize, rule, run_state_machine_as_test
from hypothesis import strategies as st


@pytest.mark.slow
@pytest.mark.mountpoint
@pytest.mark.skipif(os.name == "nt", reason="Seems to spiral into infinite loop so far...")
@pytest.mark.xfail(reason="FUSE's lower layers seems to hate this...")
def test_fuse_file_operations(tmpdir, hypothesis_settings, mountpoint_service):
    tentative = 0

    class FuseFileOperationsStateMachine(RuleBasedStateMachine):
        @initialize()
        def init(self):
            nonlocal tentative
            tentative += 1

            mountpoint_service.start()

            self.oracle_fd = os.open(tmpdir / f"oracle-test-{tentative}", os.O_RDWR | os.O_CREAT)
            self.fd = os.open(
                mountpoint_service.get_default_workspace_mountpoint() / "bar.txt",
                os.O_RDWR | os.O_CREAT,
            )

        def teardown(self):
            mountpoint_service.stop()

        @rule(size=st.integers(min_value=0))
        def read(self, size):
            expected_data = os.read(self.oracle_fd, size)
            data = os.read(self.fd, size)
            assert data == expected_data

        @rule(content=st.binary())
        def write(self, content):
            expected_ret = os.write(self.oracle_fd, content)
            ret = os.write(self.fd, content)
            assert ret == expected_ret

        @rule(
            length=st.integers(min_value=0),
            # Given FUSE takes control over the cursor position (i.e. it gives us an offset
            # parameters for read/write operations), it cannot handle SEEK_END properly (given it
            # doesn't know the size of the file, it decides not to move the cursor when handling
            # lseek with this option...)
            # seek_type=st.one_of(st.just(os.SEEK_SET), st.just(os.SEEK_CUR), st.just(os.SEEK_END)),
            seek_type=st.one_of(st.just(os.SEEK_SET), st.just(os.SEEK_CUR)),
        )
        def seek(self, length, seek_type):
            pos = os.lseek(self.fd, length, seek_type)
            expected_pos = os.lseek(self.oracle_fd, length, seek_type)
            assert pos == expected_pos

        @rule(length=st.integers(min_value=0))
        def truncate(self, length):
            os.ftruncate(self.fd, length)
            os.ftruncate(self.oracle_fd, length)

        @rule()
        def reopen(self):
            os.close(self.fd)
            self.fd = os.open(
                mountpoint_service.get_default_workspace_mountpoint() / "bar.txt", os.O_RDWR
            )
            os.close(self.oracle_fd)
            self.oracle_fd = os.open(tmpdir / f"oracle-test-{tentative}", os.O_RDWR)

    run_state_machine_as_test(FuseFileOperationsStateMachine, settings=hypothesis_settings)
