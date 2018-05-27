import attr
import bisect
from math import inf


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

    @property
    def size(self):
        return self.end - self.start

    def slice_needed(self):
        return self.buffer_slice_start != 0 or self.buffer_slice_end != self.buffer.size


def is_overlaid(buff1, buff2):
    return (
        buff1.start <= buff2.start <= buff1.end or buff1.start <= buff2.end <= buff1.end or
        buff2.start < buff1.start < buff2.end or buff2.start < buff1.end <= buff2.end
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


def _merge_in_contiguous_space(overlaid_contiguous_spaces, ibs):
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

    return ContiguousSpace(start, end, sorted(unsorted_spaces))


def merge_buffers(buffers, size=inf, offset=0):
    # Bruteforce mode: Insert one buffer after another and modify the
    # elements already present in the list if needed.
    # Should be fine enough for a small number of buffers.

    spaces = []
    expected_start = offset
    expected_end = offset + size
    max_start = inf
    max_end = -inf

    for buff in buffers:
        if not buff.size:
            continue

        if buff.end <= expected_start or buff.start >= expected_end:
            continue

        ibs_start = expected_start if buff.start < expected_start else buff.start
        ibs_end = expected_end if buff.end > expected_end else buff.end

        if ibs_start < max_start:
            max_start = ibs_start
        if ibs_end > max_end:
            max_end = ibs_end

        # Retrieve the spaces our buffer overlaid to merge them all
        overlaid = [cs for cs in spaces if is_overlaid(cs, buff)]
        ibs = InBufferSpace(ibs_start, ibs_end, buff)
        if overlaid:
            # All the overlaid spaces now constitute a contiguous space
            new_cs = _merge_in_contiguous_space(overlaid, ibs)
            for to_remove in overlaid:
                spaces.remove(to_remove)
            bisect.insort(spaces, new_cs)
        else:
            # The buffer create a new contiguous space on it own
            bisect.insort(spaces, ContiguousSpace(ibs_start, ibs_end, [ibs]))

    if not spaces:
        max_start = 0
        max_end = 0

    return UncontiguousSpace(max_start, max_end, spaces)


def quick_filter_block_accesses(block_entries, start, end):
    interestings = []
    for candidate in block_entries:
        candidate_start = candidate['offset']
        candidate_end = candidate_start + candidate['size']
        if (start <= candidate_start <= end or start <= candidate_end <= end or
                candidate_start < start < candidate_end or candidate_start < end <= candidate_end):
            interestings.append((candidate_start, candidate_end, candidate))
    return interestings
