import pytest
from hypothesis import given, strategies as st, note, assume

from parsec.core.fs2.buffer_ordering import merge_buffers, merge_buffers_with_limits, Buffer


writting_strategy = st.tuples(
    st.integers(min_value=0, max_value=2 ** 16), st.integers(min_value=0, max_value=2 ** 16)
)


def _i_to_chr(i):
    return chr(ord("a") + (i % 26))


def _build_data(entries, size=None):
    if size is None:
        size = max([end for start, end, _ in entries if start != end], default=0)
    data = bytearray(size)
    for start, end, sign in entries:
        data[start:end] = sign.encode() * (end - start)
    return data[:size].decode()


@given(writtings=st.lists(elements=writting_strategy))
def test_merge_buffers(writtings):
    writtings = [sorted(x) for x in writtings]

    buffers = [Buffer(*writting, _i_to_chr(i)) for i, writting in enumerate(writtings)]
    normalized = merge_buffers(buffers)

    expected_start = min([start for start, end in writtings if start != end], default=0)
    expected_end = max([end for start, end in writtings if start != end], default=0)

    assert normalized.start == expected_start
    assert normalized.end == expected_end
    if normalized.spaces:
        normalized.spaces[0].start == normalized.start
        normalized.spaces[-1].end == normalized.end

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

    expected = _build_data([(*x, _i_to_chr(i)) for i, x in enumerate(writtings)])
    result = _build_data(data_elems)

    assert result == expected


@given(
    writtings=st.lists(elements=writting_strategy),
    size=st.integers(min_value=0, max_value=2 ** 16),
    offset=st.integers(min_value=0, max_value=2 ** 16),
)
def test_merge_buffers_with_limits(writtings, size, offset):

    writtings = [sorted(x) for x in writtings]

    buffers = [Buffer(*writting, _i_to_chr(i)) for i, writting in enumerate(writtings)]
    normalized = merge_buffers_with_limits(buffers, offset, offset + size)

    in_cs_min_start = min([start for start, end in writtings if start != end], default=0)
    in_cs_max_end = max([end for start, end in writtings if start != end], default=0)

    assert offset <= normalized.start <= offset + size
    assert offset <= normalized.end <= offset + size

    data_elems = []
    previous_cs_end = -1
    for cs in normalized.spaces:
        assert cs.start >= in_cs_min_start
        assert cs.start >= offset
        assert cs.end <= in_cs_max_end
        assert cs.end <= size + offset

        assert cs.start > previous_cs_end
        previous_cs_end = cs.end

        previous_bs_end = cs.start
        for bs in cs.buffers:
            assert bs.start == previous_bs_end
            previous_bs_end = bs.end

            assert bs.start >= bs.buffer.start
            assert bs.end <= bs.buffer.end

            data_elems.append((bs.start, bs.end, bs.buffer.data))

    expected = _build_data(
        [(*x, _i_to_chr(i)) for i, x in enumerate(writtings)], size=offset + size
    )
    expected = expected[offset : offset + size]

    result = _build_data(data_elems, size=offset + size).encode()
    result = result[offset : offset + size]

    assert result.decode() == expected
