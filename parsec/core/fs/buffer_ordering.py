# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import bisect
from math import inf
from itertools import dropwhile


@attr.s(slots=True)
class BaseOrderedSpace:
    start = attr.ib()
    end = attr.ib()

    @property
    def size(self):
        return self.end - self.start

    def __eq__(self, other):
        if not isinstance(other, BaseOrderedSpace):
            return NotImplemented
        return self.start == other.start

    def __lt__(self, other):
        if not isinstance(other, BaseOrderedSpace):
            return NotImplemented
        return self.start < other.start


@attr.s(slots=True)
class Buffer(BaseOrderedSpace):
    data = attr.ib()


@attr.s(slots=True)
class NullFillerBuffer(BaseOrderedSpace):
    @property
    def data(self):
        return bytearray(self.size)


@attr.s(slots=True)
class ContiguousSpace(BaseOrderedSpace):
    buffers = attr.ib()


@attr.s(slots=True)
class UncontiguousSpace(BaseOrderedSpace):
    spaces = attr.ib()


@attr.s(slots=True)
class InBufferSpace(BaseOrderedSpace):
    buffer = attr.ib()

    buffer_slice_start = attr.ib(init=False)
    buffer_slice_end = attr.ib(init=False)

    def __attrs_post_init__(self):
        self.buffer_slice_start = self.start - self.buffer.start
        self.buffer_slice_end = self.end - self.buffer.start

    def get_data(self):
        if self.slice_needed():
            return self.buffer.data[self.buffer_slice_start : self.buffer_slice_end]
        else:
            return self.buffer.data

    @property
    def size(self):
        return self.end - self.start

    def slice_needed(self):
        return self.buffer_slice_start != 0 or self.buffer_slice_end != self.buffer.size


def _is_overlaid(buff1, buff2):
    return (
        buff1.start <= buff2.start <= buff1.end
        or buff1.start <= buff2.end <= buff1.end
        or buff2.start < buff1.start < buff2.end
        or buff2.start < buff1.end <= buff2.end
    )


def _trim_buffers(buffers, new_start=None, new_end=None):
    trimmed = []

    for buff_space in buffers:
        buff_start = buff_space.start
        buff_end = buff_space.end

        if new_start is not None:
            if buff_end <= new_start:
                continue
            if buff_start < new_start:
                buff_start = new_start

        if new_end is not None:
            if buff_start >= new_end:
                continue
            if buff_end > new_end:
                buff_end = new_end

        trimmed.append(InBufferSpace(buff_start, buff_end, buff_space.buffer))

    return trimmed


def _trim_contiguous_space(cs, new_start=None, new_end=None):
    trimmed_buffers = _trim_buffers(cs.buffers, new_start=new_start, new_end=new_end)
    if trimmed_buffers:
        trimmed_start = trimmed_buffers[0].start
        trimmed_end = trimmed_buffers[-1].end
    else:
        trimmed_start = new_start if new_start is not None else cs.start
        trimmed_end = trimmed_start

    return ContiguousSpace(trimmed_start, trimmed_end, trimmed_buffers)


def _trim_uncontiguous_space(ucs, new_start=None, new_end=None):
    trimmed_start = ucs.start if new_start is None else new_start
    trimmed_end = ucs.end if new_end is None else new_end
    trimmed_spaces = ucs.spaces

    if new_start is not None and ucs.start < new_start:
        trimmed_spaces = list(dropwhile(lambda x: x.end <= new_start, trimmed_spaces))
        if trimmed_spaces:
            trimmed_spaces[0] = _trim_contiguous_space(trimmed_spaces[0], new_start=new_start)

    if new_end is not None and ucs.end > new_end:
        trimmed_spaces = list(
            reversed(list(dropwhile(lambda x: x.start >= new_end, reversed(trimmed_spaces))))
        )
        if trimmed_spaces:
            trimmed_spaces[-1] = _trim_contiguous_space(trimmed_spaces[-1], new_end=new_end)

    return UncontiguousSpace(trimmed_start, trimmed_end, trimmed_spaces or [])


def _split_aligned_contiguous_space(cs, block_size):
    if cs.size == block_size:
        return [cs]

    splitted_spaces = []
    overflowing_bs = None

    def take_next_buffer_space():
        nonlocal overflowing_bs
        for buff in cs.buffers:
            yield buff
            while overflowing_bs:
                buff = overflowing_bs
                overflowing_bs = None
                yield buff

    curr_acs_remain_space = block_size
    curr_acs_buffers = []
    curr_acs_start = cs.start
    for bs in take_next_buffer_space():
        if bs.size > curr_acs_remain_space:
            # Current buffer must be splitted in two
            split_offset = curr_acs_start + block_size
            overflowing_bs = InBufferSpace(split_offset, bs.end, bs.buffer)
            bs = InBufferSpace(bs.start, split_offset, bs.buffer)

        curr_acs_buffers.append(bs)
        curr_acs_remain_space -= bs.size

        if not curr_acs_remain_space:
            end = curr_acs_start + block_size
            splitted_spaces.append(ContiguousSpace(curr_acs_start, end, curr_acs_buffers))
            curr_acs_start = end
            curr_acs_remain_space = block_size
            curr_acs_buffers = []

    # Last space maybe smaller than block size
    if curr_acs_remain_space < block_size:
        splitted_spaces.append(
            ContiguousSpace(
                curr_acs_start,
                curr_acs_start + block_size - curr_acs_remain_space,
                curr_acs_buffers,
            )
        )

    return splitted_spaces


def _merge_in_contiguous_space(overlaid_contiguous_spaces, buff):
    ibs = InBufferSpace(buff.start, buff.end, buff)
    start = ibs.start
    end = ibs.end
    unsorted_spaces = [ibs]

    for sub_cs in overlaid_contiguous_spaces:
        if sub_cs.start < start:
            start = sub_cs.start
        if sub_cs.end > end:
            end = sub_cs.end

        if sub_cs.start == ibs.end or ibs.start == sub_cs.end:
            # Strictly contiguous
            trimmed = sub_cs.buffers
        elif sub_cs.start >= ibs.start:
            if sub_cs.end <= ibs.end:
                # Contiguous spaces totally overwritten by the new buffer
                continue
            else:
                trimmed = _trim_buffers(sub_cs.buffers, new_start=ibs.end)
        else:
            if sub_cs.end > ibs.end:
                # Contiguous space now split in two by the new buffer
                trimmed = _trim_buffers(sub_cs.buffers, new_start=ibs.end)
                trimmed += _trim_buffers(sub_cs.buffers, new_end=ibs.start)
            else:
                trimmed = _trim_buffers(sub_cs.buffers, new_end=ibs.start)

        unsorted_spaces += trimmed

    # Comparing attrs instances is costy and causes this sorting process to be
    # really slow (0.5 seconds for 500 items). Using a lambda key speeds up the
    # process by a factor of 20.
    sorted_spaces = sorted(unsorted_spaces, key=lambda x: (x.start, x.end))
    return ContiguousSpace(start, end, sorted_spaces)


def merge_buffers(buffers):
    """
    Flatten multiple (possibly overlapping) buffers.

    Args:
        buffers: list of :class:`Buffer` to merge.

    Returns:
        An :class:`UncontiguousSpace`.
    """
    # Bruteforce mode: Insert one buffer after another and modify the
    # elements already present in the list if needed.
    # Should be fine enough for a small number of buffers.

    spaces = []
    min_start = inf
    max_end = -inf

    for buff in buffers:
        if not buff.size:
            continue

        if buff.start < min_start:
            min_start = buff.start
        if buff.end > max_end:
            max_end = buff.end

        # Retrieve the spaces our buffer overlaid to merge them all
        overlaid = [cs for cs in spaces if _is_overlaid(cs, buff)]
        if overlaid:
            # All the overlaid spaces now constitute a contiguous space
            new_cs = _merge_in_contiguous_space(overlaid, buff)
            for to_remove in overlaid:
                spaces.remove(to_remove)
            bisect.insort(spaces, new_cs)
        else:
            # The buffer create a new contiguous space on it own
            ibs = InBufferSpace(buff.start, buff.end, buff)
            bisect.insort(spaces, ContiguousSpace(buff.start, buff.end, [ibs]))

    if not spaces:
        min_start = 0
        max_end = 0

    return UncontiguousSpace(min_start, max_end, spaces)


def merge_buffers_with_limits(buffers, start, end):
    """
    Flatten multiple (possibly overlapping) buffers between the given bounds.

    Args:
        buffers: list of :class:`Buffer` to merge.
        start: starting offset, everything before will be ignored.
        end: ending offset, everything after will be ignored.

    Returns:
        An :class:`UncontiguousSpace`.
    """
    nolimit = merge_buffers(buffers)

    return _trim_uncontiguous_space(nolimit, new_start=start, new_end=end)


def merge_buffers_with_limits_and_alignment(buffers, start, end, block_size):
    """
    Flatten multiple (possibly overlapping) buffers between the given bounds
    and split the result in multiples :class:`ContiguousSpace` of block_size size.

    Args:
        buffers: list of :class:`Buffer` to merge. If those buffers doesn't
                 form a contiguous space, the returned :class:`ContiguousSpace`
                 will contains :class:`NullFillerBuffer` buffers as padding.
        start: starting offset, everything before will be ignored.
               Must be aligned on block_size.
        end: ending offset, everything after will be ignored.
        block_size: size of the alignment

    Raises:
        ValueError: If start is not a multiple of block_size

    Returns:
        An :class:`UncontiguousSpace`.
    """
    if start % block_size:
        raise ValueError("start must be a multiple of block_size")

    unaligned = merge_buffers(buffers)
    aligned = _trim_uncontiguous_space(unaligned, new_start=start, new_end=end)
    assert aligned.start == start
    assert aligned.end == end

    if aligned.spaces:
        if len(aligned.spaces) > 1 or aligned.spaces[0].start != start:
            curr_pos = start
            forced_contiguous_buffers = []
            for cs in aligned.spaces:
                if curr_pos != cs.start:
                    forced_contiguous_buffers.append(
                        InBufferSpace(curr_pos, cs.start, NullFillerBuffer(curr_pos, cs.start))
                    )
                forced_contiguous_buffers += cs.buffers
                curr_pos = cs.end
            cs = ContiguousSpace(start, end, forced_contiguous_buffers)
        else:
            cs = aligned.spaces[0]
        splitted_spaces = _split_aligned_contiguous_space(cs, block_size)
        assert splitted_spaces[0].start == start
        assert splitted_spaces[-1].end <= end
    else:
        splitted_spaces = []

    return UncontiguousSpace(start, end, splitted_spaces)


def quick_filter_block_accesses(block_entries, start, end):
    """
    Filter a list of block accesses to only return the ones fitting in the
    given range.
    """
    interestings = []
    for candidate in block_entries:
        candidate_start = candidate.offset
        candidate_end = candidate_start + candidate.size
        if (
            start <= candidate_start <= end
            or start <= candidate_end <= end
            or candidate_start < start < candidate_end
            or candidate_start < end <= candidate_end
        ):
            interestings.append((candidate_start, candidate_end, candidate))
    return interestings
