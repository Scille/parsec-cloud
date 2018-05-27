import pytest
from hypothesis import given, strategies as st, note, assume

from parsec.core.fs2.buffer_ordering import merge_buffers, Buffer


writting_strategy = st.tuples(st.integers(min_value=0, max_value=2**16), st.integers(min_value=0, max_value=2**16))


def _i_to_chr(i):
    return chr(ord("a") + (i % 26))


@given(writtings=st.lists(elements=writting_strategy))
def test_merge_buffers(writtings):
    _tester(writtings)


@given(writtings=st.lists(elements=writting_strategy), size=st.integers(min_value=0, max_value=2**16), offset=st.integers(min_value=0, max_value=2**16))
def test_merge_buffers_with_size_and_offset(writtings, size, offset):
    _tester(writtings, (size, offset))

def test_x():
    _tester([(0, 1)], (0, 0))

def _tester(writtings, size_and_offset=()):
    writtings = [sorted(x) for x in writtings]

    def build_data(entries):
        size = max([end for start, end, _ in entries if start != end], default=0)
        data = bytearray(size)
        for start, end, sign in entries:
            data[start:end] = sign.encode() * (end - start)
        return data.decode()

    buffers = [Buffer(*writting, _i_to_chr(i)) for i, writting in enumerate(writtings)]
    normalized = merge_buffers(buffers, *size_and_offset)

    expected_start = min([start for start, end in writtings if start != end], default=0)
    expected_end = max([end for start, end in writtings if start != end], default=0)

    if size_and_offset:
        size, offset = size_and_offset
        expected_start = min(offset, expected_start)
        expected_end = min(offset + size, expected_end)

    assert normalized.start == expected_start
    assert normalized.end == expected_end

    data_elems = []
    previous_cs_end = -1
    for cs in normalized.spaces:
        assert cs.start >= expected_start
        assert cs.end <= expected_end

        assert cs.start > previous_cs_end
        previous_cs_end = cs.end

        previous_bs_end = cs.start
        for bs in cs.buffers:
            assert bs.start == previous_bs_end
            previous_bs_end = bs.end

            assert bs.start >= bs.buffer.start
            assert bs.end <= bs.buffer.end

            data_elems.append((bs.start, bs.end, bs.buffer.data))

    expected = build_data([(*x, _i_to_chr(i)) for i, x in enumerate(writtings)])
    if size_and_offset:
        trim_start = size_and_offset[1]
        trim_end = sum(size_and_offset + size_and_offset)
        expected = expected[trim_start:trim_end]
    result = build_data(data_elems)

    assert result == expected
