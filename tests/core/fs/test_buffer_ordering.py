import pytest
from hypothesis import given, strategies as st, note, assume

from parsec.core.fs2.buffer_ordering import normalize_buffers, Buffer


writting_strategy = st.tuples(st.integers(min_value=0, max_value=2**16), st.integers(min_value=0, max_value=2**16))


def _i_to_chr(i):
    return chr(ord("a") + (i % 26))


@given(writtings=st.lists(elements=writting_strategy))
def test_normalize_buffers(writtings):
    writtings = [sorted(x) for x in writtings]

    def build_data(entries):
        size = max([end for start, end, _ in entries if start != end], default=0)
        data = bytearray(size)
        for start, end, sign in entries:
            data[start:end] = sign.encode() * (end - start)
        return data.decode()

    buffers = [Buffer(*writting, _i_to_chr(i)) for i, writting in enumerate(writtings)]
    normalized = normalize_buffers(buffers)

    start = min([start for start, end in writtings if start != end], default=0)
    end = max([end for start, end in writtings if start != end], default=0)
    assert normalized.start == start
    assert normalized.end == end

    data_elems = []
    previous_cs_end = -1
    for cs in normalized.spaces:
        assert cs.start >= start
        assert cs.end <= end

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
    result = build_data(data_elems)

    assert result == expected
