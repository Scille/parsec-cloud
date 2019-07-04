# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import string
from hypothesis import given, strategies as st

from parsec.core.fs.buffer_ordering import (
    merge_buffers,
    merge_buffers_with_limits,
    merge_buffers_with_limits_and_alignment,
    Buffer,
)


DATA_MAX_SIZE = 2 ** 8


class ColoredBuffer(Buffer):
    """
    We don't care about the buffer's data in those tests. However it makes
    debugging much easier if each buffer is made of a single caracter
    (mostly) per-buffer unique.
    """

    COLOR_CHOICES = string.ascii_lowercase + string.digits

    def __init__(self, start, end, data=None):
        if not data:
            color_key = (DATA_MAX_SIZE * start + end) % len(self.COLOR_CHOICES)
            color = self.COLOR_CHOICES[color_key]
            data = (color * (end - start)).encode("utf8")
        super().__init__(start, end, data)


buffer_size_strategy = st.integers(min_value=0, max_value=DATA_MAX_SIZE)
buffer_oversize_strategy = st.integers(min_value=0, max_value=int(DATA_MAX_SIZE * 1.2))
buffer_bounds_strategy = st.builds(sorted, st.tuples(buffer_size_strategy, buffer_size_strategy))
limits_strategy = st.builds(sorted, st.tuples(buffer_oversize_strategy, buffer_oversize_strategy))
buffer_strategy = st.builds(lambda x: ColoredBuffer(*x), buffer_bounds_strategy)


def _build_data_from_buffers(buffers, size=None):
    if size is None:
        size = max((b.end for b in buffers), default=0)
    data = bytearray(size)
    for b in buffers:
        data[b.start : b.end] = b.data
    return data


def _build_data_from_uncontiguous_space(ucs):
    data = bytearray(ucs.end)
    for cs in ucs.spaces:
        for buff in cs.buffers:
            data[buff.start : buff.end] = buff.buffer.data[
                buff.buffer_slice_start : buff.buffer_slice_end
            ]
    return data


def uncontigous_space_sanity_checks(ucs, expected_start, expected_end):
    previous_cs_end = -1
    for cs in ucs.spaces:
        assert cs.start >= expected_start
        assert cs.end <= expected_end

        assert cs.start >= previous_cs_end
        previous_cs_end = cs.end

        previous_bs_end = cs.start
        for bs in cs.buffers:
            assert bs.start == previous_bs_end
            previous_bs_end = bs.end

            assert bs.start >= bs.buffer.start
            assert bs.end <= bs.buffer.end


@given(buffers=st.lists(elements=buffer_strategy))
def test_merge_buffers(buffers):
    non_empty_buffers = [b for b in buffers if b.size]

    merged = merge_buffers(buffers)

    expected_start = min([b.start for b in non_empty_buffers], default=0)
    expected_end = max([b.end for b in non_empty_buffers], default=0)

    assert merged.start == expected_start
    assert merged.end == expected_end
    if merged.spaces:
        merged.spaces[0].start == merged.start
        merged.spaces[-1].end == merged.end
    uncontigous_space_sanity_checks(merged, expected_start, expected_end)

    expected = _build_data_from_buffers(non_empty_buffers)
    result = _build_data_from_uncontiguous_space(merged)

    assert result == expected


@given(buffers=st.lists(elements=buffer_strategy), limits=limits_strategy)
def test_merge_buffers_with_limits(buffers, limits):
    start, end = limits
    non_empty_buffers = [b for b in buffers if b.size]

    merged = merge_buffers_with_limits(buffers, start, end)

    expected_in_cs_min_start = min([b.start for b in non_empty_buffers], default=0)
    expected_in_cs_max_end = max([b.end for b in non_empty_buffers], default=0)

    assert merged.start == start
    assert merged.end == end
    if merged.spaces:
        merged.spaces[0].start == expected_in_cs_min_start
        merged.spaces[-1].end == expected_in_cs_max_end
    uncontigous_space_sanity_checks(merged, start, end)

    expected = _build_data_from_buffers(non_empty_buffers, end)
    result = _build_data_from_uncontiguous_space(merged)

    assert result[start:end] == expected[start:end]


@given(
    buffers=st.lists(elements=buffer_strategy),
    limits=limits_strategy,
    alignment=buffer_oversize_strategy.filter(lambda x: x != 0),
)
def test_merge_buffers_with_limits_and_alignment(buffers, limits, alignment):
    start, end = limits
    start = start - start % alignment
    non_empty_buffers = [b for b in buffers if b.size]

    merged = merge_buffers_with_limits_and_alignment(buffers, start, end, alignment)

    expected_in_cs_min_start = min([b.start for b in non_empty_buffers], default=0)
    expected_in_cs_max_end = max([b.end for b in non_empty_buffers], default=0)

    assert merged.start == start
    assert merged.end == end
    if merged.spaces:
        merged.spaces[0].start == expected_in_cs_min_start
        merged.spaces[-1].end == expected_in_cs_max_end
    uncontigous_space_sanity_checks(merged, start, end)

    expected = _build_data_from_buffers(non_empty_buffers, end)
    result = _build_data_from_uncontiguous_space(merged)

    assert result[start:end] == expected[start:end]


def test_merge_buffers_performance():
    blocks = []
    dirty_size = 4 * 1024
    clean_size = 512 * 1024
    total_size = 2 * 1024 * 1024
    for offset in range(0, total_size, dirty_size):
        block = Buffer(offset, offset + dirty_size, None)
        blocks.append(block)

    merged = merge_buffers_with_limits_and_alignment(blocks, 0, total_size, clean_size)

    assert merged.start == 0
    assert merged.end == merged.size == total_size
    assert len(merged.spaces) == 4
    assert sum(len(space.buffers) for space in merged.spaces) == 512
    buffers = [buff.buffer for space in merged.spaces for buff in space.buffers]
    assert buffers == blocks
