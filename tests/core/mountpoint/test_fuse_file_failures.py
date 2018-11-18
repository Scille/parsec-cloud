import os
import pytest


@pytest.mark.xfail(reason="FUSE's lower layers seems to hate this...")
@pytest.mark.fuse
def test_fuse_grow_by_truncate(tmpdir, fuse_service):
    fuse_service.start()
    try:
        oracle_fd = os.open(tmpdir / f"oracle-test", os.O_RDWR | os.O_CREAT)
        fd = os.open(fuse_service.default_workspace / "bar.txt", os.O_RDWR | os.O_CREAT)

        length = 1
        os.ftruncate(fd, length)
        os.ftruncate(oracle_fd, length)

        size = 1
        data = os.read(fd, size)
        expected_data = os.read(oracle_fd, size)
        assert data == expected_data

    finally:
        fuse_service.stop()


@pytest.mark.xfail(reason="FUSE's lower layers seems to hate this...")
@pytest.mark.fuse
def test_fuse_empty_read_then_reopen(tmpdir, fuse_service):
    fuse_service.start()
    try:
        oracle_fd = os.open(tmpdir / f"oracle-test", os.O_RDWR | os.O_CREAT)
        fd = os.open(fuse_service.default_workspace / "bar.txt", os.O_RDWR | os.O_CREAT)

        content = b"\x00"
        expected_ret = os.write(oracle_fd, content)
        ret = os.write(fd, content)
        assert ret == expected_ret

        size = 1
        expected_data = os.read(oracle_fd, size)
        data = os.read(fd, size)
        assert data == expected_data

        size = 0
        expected_data = os.read(oracle_fd, size)
        data = os.read(fd, size)
        assert data == expected_data

        os.close(oracle_fd)
        os.close(fd)
        oracle_fd = os.open(tmpdir / f"oracle-test", os.O_RDWR)
        fd = os.open(fuse_service.default_workspace / "bar.txt", os.O_RDWR)

        size = 1
        expected_data = os.read(oracle_fd, size)
        data = os.read(fd, size)
        assert data == expected_data

    finally:
        fuse_service.stop()
