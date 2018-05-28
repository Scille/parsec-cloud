import pytest
from hypothesis import given, strategies as st, note, assume

from parsec.core.fs2.buffer_ordering import (
    merge_buffers,
    merge_buffers_with_limits,
    merge_buffers_with_limits_and_alignment,
    Buffer,
)


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


block_size_strategy = st.shared(st.integers(min_value=1, max_value=2 ** 8), key="block_size")


def build_contiguous_writtings(non_contiguous_writtings, block_size):
    # Writtings must not form holes for this test
    writtings = []
    curr_offset = 0
    for writting in non_contiguous_writtings:
        start = min(*writting, curr_offset)
        end = max(*writting)
        writtings.append((start, end))
        curr_offset = max(end, curr_offset)
    if curr_offset < block_size:
        writtings.append((curr_offset, block_size))

    return writtings


contiguous_writtings_strategy = st.shared(
    st.builds(
        build_contiguous_writtings, st.lists(elements=writting_strategy), block_size_strategy
    ),
    key="contiguous_writtings",
)


data_size_strategy = st.shared(
    st.builds(lambda ws: max([x[1] for x in ws], default=0), contiguous_writtings_strategy)
)


def build_aligned_interval(data_size, block_size, start_block_count, size_block_count):
    start = start_block_count * block_size
    while start > data_size:
        start -= block_size
    for i in range(size_block_count, -1, -1):
        end = start + i * block_size
        if end <= data_size:
            break
    return start, end


aligned_interval_strategy = st.builds(
    build_aligned_interval,
    data_size_strategy,
    block_size_strategy,
    start_block_count=st.integers(min_value=0, max_value=2 ** 8),
    size_block_count=st.integers(min_value=0, max_value=2 ** 3),
)


@given(
    writtings=contiguous_writtings_strategy,
    block_size=block_size_strategy,
    interval=aligned_interval_strategy,
)
def test_merge_buffers_with_limits_and_alignment(writtings, block_size, interval):
    start, end = interval

    buffers = [Buffer(*writting, _i_to_chr(i)) for i, writting in enumerate(writtings)]
    normalized = merge_buffers_with_limits_and_alignment(buffers, start, end, block_size)

    assert normalized.start == start
    assert normalized.end == end

    data_elems = []
    previous_cs_end = normalized.start
    for cs in normalized.spaces:
        #  Each contiguous block should follow each other here
        assert cs.size == block_size
        assert cs.start == previous_cs_end
        previous_cs_end = cs.end

        previous_bs_end = cs.start
        for bs in cs.buffers:
            assert bs.start == previous_bs_end
            previous_bs_end = bs.end

            assert bs.start >= bs.buffer.start
            assert bs.end <= bs.buffer.end

            data_elems.append((bs.start, bs.end, bs.buffer.data))

    assert previous_cs_end == normalized.end

    expected = _build_data([(*x, _i_to_chr(i)) for i, x in enumerate(writtings)], size=end)
    expected = expected[start:end]

    result = _build_data(data_elems, size=end).encode()
    result = result[start:end]

    assert result.decode() == expected
